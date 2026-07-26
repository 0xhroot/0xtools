use crate::error::{AppError, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub description: String,
    pub packages: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileCollection {
    pub profiles: Vec<Profile>,
}

impl ProfileCollection {
    pub fn load_bundled() -> Self {
        let profiles = vec![
            Profile {
                name: "Web Security".to_string(),
                description: "Common web application security assessment toolkit".to_string(),
                packages: vec![
                    "ffuf".to_string(),
                    "gobuster".to_string(),
                    "nikto".to_string(),
                    "sqlmap".to_string(),
                    "nuclei".to_string(),
                    "whatweb".to_string(),
                    "wpscan".to_string(),
                    "dirsearch".to_string(),
                    "feroxbuster".to_string(),
                    "httpx".to_string(),
                ],
            },
            Profile {
                name: "OSINT".to_string(),
                description: "Open source intelligence gathering tools".to_string(),
                packages: vec![
                    "theharvester".to_string(),
                    "recon-ng".to_string(),
                    "maltego".to_string(),
                    "sherlock".to_string(),
                    "spiderfoot".to_string(),
                    "amass".to_string(),
                    "subfinder".to_string(),
                ],
            },
            Profile {
                name: "Network Assessment".to_string(),
                description: "Network scanning and analysis toolkit".to_string(),
                packages: vec![
                    "nmap".to_string(),
                    "masscan".to_string(),
                    "wireshark-cli".to_string(),
                    "netdiscover".to_string(),
                    "hping".to_string(),
                    "tcpdump".to_string(),
                ],
            },
            Profile {
                name: "Reverse Engineering".to_string(),
                description: "Binary analysis and reverse engineering toolkit".to_string(),
                packages: vec![
                    "ghidra".to_string(),
                    "radare2".to_string(),
                    "binwalk".to_string(),
                    "strace".to_string(),
                    "ltrace".to_string(),
                    "gdb".to_string(),
                ],
            },
            Profile {
                name: "Digital Forensics".to_string(),
                description: "Digital forensics and incident response toolkit".to_string(),
                packages: vec![
                    "autopsy".to_string(),
                    "sleuthkit".to_string(),
                    "volatility3".to_string(),
                    "bulk-extractor".to_string(),
                    "binwalk".to_string(),
                    "foremost".to_string(),
                ],
            },
            Profile {
                name: "Wireless".to_string(),
                description: "Wireless network security assessment toolkit".to_string(),
                packages: vec![
                    "aircrack-ng".to_string(),
                    "kismet".to_string(),
                    "wifite".to_string(),
                    "reaver".to_string(),
                    "hashcat".to_string(),
                ],
            },
            Profile {
                name: "Password Security".to_string(),
                description: "Password testing and auditing toolkit".to_string(),
                packages: vec![
                    "hashcat".to_string(),
                    "john".to_string(),
                    "hydra".to_string(),
                    "medusa".to_string(),
                    "ophcrack".to_string(),
                ],
            },
            Profile {
                name: "Bug Bounty".to_string(),
                description: "Essential bug bounty hunting toolkit".to_string(),
                packages: vec![
                    "ffuf".to_string(),
                    "nuclei".to_string(),
                    "httpx".to_string(),
                    "subfinder".to_string(),
                    "amass".to_string(),
                    "gobuster".to_string(),
                    "sqlmap".to_string(),
                    "nikto".to_string(),
                    "whatweb".to_string(),
                ],
            },
        ];

        Self { profiles }
    }

    pub fn load_from_dir(dir: &PathBuf) -> Result<Self> {
        let mut profiles = Vec::new();

        if dir.exists() {
            for entry in fs::read_dir(dir)
                .map_err(|e| AppError::Config(format!("Failed to read profiles dir: {}", e)))?
            {
                let entry =
                    entry.map_err(|e| AppError::Config(format!("Failed to read entry: {}", e)))?;
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("toml") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(profile) = toml::from_str::<Profile>(&content) {
                            profiles.push(profile);
                        }
                    }
                }
            }
        }

        Ok(Self { profiles })
    }

    pub fn find(&self, name: &str) -> Option<&Profile> {
        self.profiles
            .iter()
            .find(|p| p.name.to_lowercase() == name.to_lowercase())
    }

    pub fn list(&self) -> &[Profile] {
        &self.profiles
    }
}
