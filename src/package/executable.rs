use std::path::{Path, PathBuf};

pub struct ExecutableDiscovery;

impl ExecutableDiscovery {
    pub fn for_installed_package(pkg_name: &str) -> Vec<String> {
        let mut executables = Vec::new();

        let bin_dirs = ["/usr/bin", "/usr/sbin", "/usr/lib", "/usr/local/bin"];

        for dir in &bin_dirs {
            let path = Path::new(dir);
            if !path.exists() {
                continue;
            }

            if let Ok(entries) = std::fs::read_dir(path) {
                for entry in entries.flatten() {
                    let file_name = entry.file_name();
                    let name_str = file_name.to_string_lossy();

                    if name_str.contains(pkg_name)
                        || (pkg_name.contains('-') && name_str == pkg_name.replace('-', "_"))
                        || (pkg_name.contains('_') && name_str == pkg_name.replace('_', "-"))
                    {
                        let full_path = entry.path();
                        if is_executable(&full_path) {
                            executables.push(full_path.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }

        executables.sort();
        executables.dedup();
        executables
    }

    pub fn find_executables_in_dir(dir: &Path) -> Vec<PathBuf> {
        let mut execs = Vec::new();
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && is_executable(&path) {
                    execs.push(path);
                }
            }
        }
        execs.sort();
        execs
    }
}

pub fn discover_from_name(name: &str) -> Vec<String> {
    let mut execs = ExecutableDiscovery::for_installed_package(name);

    let primary = format!("/usr/bin/{}", name);
    if Path::new(&primary).exists() && !execs.contains(&primary) {
        execs.insert(0, primary);
    }

    let sbin = format!("/usr/sbin/{}", name);
    if Path::new(&sbin).exists() && !execs.contains(&sbin) {
        execs.push(sbin);
    }

    execs.sort();
    execs.dedup();
    execs
}

fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    match std::fs::metadata(path) {
        Ok(meta) => {
            let perm = meta.permissions();
            perm.mode() & 0o111 != 0
        }
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discover_nonexistent() {
        let execs = discover_from_name("nonexistent-pkg-xyz-123");
        assert!(execs.is_empty());
    }

    #[test]
    fn test_is_executable_real() {
        assert!(is_executable(Path::new("/usr/bin/ls")));
    }

    #[test]
    fn test_is_not_executable() {
        assert!(!is_executable(Path::new("/etc/hostname")));
    }
}
