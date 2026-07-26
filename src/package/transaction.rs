use crate::error::{AppError, Result};
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionOp {
    Install,
    Remove,
    Upgrade,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionPreview {
    pub package: String,
    pub repository: String,
    pub version: String,
    pub operation: TransactionOp,
    pub dependencies: Vec<String>,
    pub already_installed_deps: Vec<String>,
}

pub struct Transaction;

impl Transaction {
    pub fn preview_install(package: &str) -> Result<TransactionPreview> {
        let output = Command::new("pacman")
            .args(["-Si", package])
            .output()
            .map_err(|e| AppError::TransactionFailed(format!("Failed to query package: {}", e)))?;

        if !output.status.success() {
            return Err(AppError::TransactionFailed(format!(
                "Package '{}' not found in repositories",
                package
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let version = parse_pacman_field(&stdout, "Version").unwrap_or_default();
        let repo = parse_pacman_field(&stdout, "Repository").unwrap_or_default();

        let deps_output = Command::new("pacman")
            .args(["-Si", package])
            .output()
            .map_err(|e| AppError::TransactionFailed(format!("Failed to query deps: {}", e)))?;

        let deps_stdout = String::from_utf8_lossy(&deps_output.stdout);
        let dependencies = parse_dep_list(&deps_stdout, "Depends On");

        let already_installed_deps = if !dependencies.is_empty() {
            let query_output = Command::new("pacman").args(["-Q"]).output().map_err(|e| {
                AppError::TransactionFailed(format!("Failed to query installed: {}", e))
            })?;

            if query_output.status.success() {
                let installed_str = String::from_utf8_lossy(&query_output.stdout);
                let installed_names: std::collections::HashSet<&str> = installed_str
                    .lines()
                    .filter_map(|line| line.split_whitespace().next())
                    .collect();
                dependencies
                    .iter()
                    .filter(|d| installed_names.contains(d.as_str()))
                    .cloned()
                    .collect()
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        Ok(TransactionPreview {
            package: package.to_string(),
            repository: repo,
            version,
            operation: TransactionOp::Install,
            dependencies,
            already_installed_deps,
        })
    }

    pub fn preview_remove(package: &str) -> Result<TransactionPreview> {
        Ok(TransactionPreview {
            package: package.to_string(),
            repository: String::new(),
            version: String::new(),
            operation: TransactionOp::Remove,
            dependencies: Vec::new(),
            already_installed_deps: Vec::new(),
        })
    }

    pub fn execute(preview: &TransactionPreview, sudo: bool) -> Result<String> {
        let (cmd, args) = match preview.operation {
            TransactionOp::Install => {
                if sudo {
                    (
                        "sudo",
                        vec!["pacman", "-S", "--noconfirm", &preview.package],
                    )
                } else {
                    ("pacman", vec!["-S", "--noconfirm", &preview.package])
                }
            }
            TransactionOp::Remove => {
                if sudo {
                    (
                        "sudo",
                        vec!["pacman", "-R", "--noconfirm", &preview.package],
                    )
                } else {
                    ("pacman", vec!["-R", "--noconfirm", &preview.package])
                }
            }
            TransactionOp::Upgrade => {
                if sudo {
                    (
                        "sudo",
                        vec!["pacman", "-S", "--noconfirm", &preview.package],
                    )
                } else {
                    ("pacman", vec!["-S", "--noconfirm", &preview.package])
                }
            }
        };

        let output = Command::new(cmd)
            .args(&args)
            .output()
            .map_err(|e| AppError::TransactionFailed(format!("Failed to execute: {}", e)))?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if !output.status.success() {
            return Err(AppError::TransactionFailed(format!(
                "Transaction failed:\n{}",
                if stderr.is_empty() { &stdout } else { &stderr }
            )));
        }

        Ok(stdout)
    }
}

fn parse_pacman_field(output: &str, field: &str) -> Option<String> {
    for line in output.lines() {
        if let Some(colon_pos) = line.find(" : ") {
            let prefix = &line[..colon_pos];
            if prefix.trim() == field {
                let value = &line[colon_pos + 3..];
                let trimmed = value.trim();
                if !trimmed.is_empty() && trimmed != "None" {
                    return Some(trimmed.to_string());
                }
            }
        }
    }
    None
}

fn parse_dep_list(output: &str, field: &str) -> Vec<String> {
    if let Some(line) = output.lines().find(|l| l.starts_with(field)) {
        if let Some(value) = line.split(':').nth(1) {
            return value
                .split_whitespace()
                .map(|s| {
                    s.split(['>', '<', '='])
                        .next()
                        .unwrap_or(s)
                        .trim()
                        .to_string()
                })
                .filter(|s| !s.is_empty() && s != "None")
                .collect();
        }
    }
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pacman_field() {
        let output = "Repository      : extra\nVersion         : 1.2.3-1\nDescription     : A tool";
        assert_eq!(
            parse_pacman_field(output, "Version"),
            Some("1.2.3-1".to_string())
        );
        assert_eq!(
            parse_pacman_field(output, "Repository"),
            Some("extra".to_string())
        );
        assert_eq!(parse_pacman_field(output, "Nonexistent"), None);
    }
}
