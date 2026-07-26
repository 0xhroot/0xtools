use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Category {
    Reconnaissance,
    Osint,
    NetworkSecurity,
    WebSecurity,
    ApiSecurity,
    Wireless,
    PasswordSecurity,
    VulnerabilityAssessment,
    ActiveDirectory,
    ExploitDevelopment,
    ReverseEngineering,
    BinaryAnalysis,
    MalwareAnalysis,
    DigitalForensics,
    IncidentResponse,
    MobileSecurity,
    CloudSecurity,
    ContainerSecurity,
    KubernetesSecurity,
    SourceCodeSecurity,
    Cryptography,
    Steganography,
    HardwareIot,
    Firmware,
    BluetoothBle,
    RfidNfc,
    Sdr,
    ThreatIntelligence,
    DefensiveSecurity,
    IdsIps,
    Honeypots,
    ProxyTunneling,
    Fuzzing,
    Reporting,
    SecurityUtilities,
}

impl Category {
    pub fn all() -> &'static [Category] {
        &[
            Category::Reconnaissance,
            Category::Osint,
            Category::NetworkSecurity,
            Category::WebSecurity,
            Category::ApiSecurity,
            Category::Wireless,
            Category::PasswordSecurity,
            Category::VulnerabilityAssessment,
            Category::ActiveDirectory,
            Category::ExploitDevelopment,
            Category::ReverseEngineering,
            Category::BinaryAnalysis,
            Category::MalwareAnalysis,
            Category::DigitalForensics,
            Category::IncidentResponse,
            Category::MobileSecurity,
            Category::CloudSecurity,
            Category::ContainerSecurity,
            Category::KubernetesSecurity,
            Category::SourceCodeSecurity,
            Category::Cryptography,
            Category::Steganography,
            Category::HardwareIot,
            Category::Firmware,
            Category::BluetoothBle,
            Category::RfidNfc,
            Category::Sdr,
            Category::ThreatIntelligence,
            Category::DefensiveSecurity,
            Category::IdsIps,
            Category::Honeypots,
            Category::ProxyTunneling,
            Category::Fuzzing,
            Category::Reporting,
            Category::SecurityUtilities,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Category::Reconnaissance => "Reconnaissance",
            Category::Osint => "OSINT",
            Category::NetworkSecurity => "Network Security",
            Category::WebSecurity => "Web Security",
            Category::ApiSecurity => "API Security",
            Category::Wireless => "Wireless",
            Category::PasswordSecurity => "Password Security",
            Category::VulnerabilityAssessment => "Vulnerability Assessment",
            Category::ActiveDirectory => "Active Directory",
            Category::ExploitDevelopment => "Exploit Development",
            Category::ReverseEngineering => "Reverse Engineering",
            Category::BinaryAnalysis => "Binary Analysis",
            Category::MalwareAnalysis => "Malware Analysis",
            Category::DigitalForensics => "Digital Forensics",
            Category::IncidentResponse => "Incident Response",
            Category::MobileSecurity => "Mobile Security",
            Category::CloudSecurity => "Cloud Security",
            Category::ContainerSecurity => "Container Security",
            Category::KubernetesSecurity => "Kubernetes Security",
            Category::SourceCodeSecurity => "Source Code Security",
            Category::Cryptography => "Cryptography",
            Category::Steganography => "Steganography",
            Category::HardwareIot => "Hardware / IoT",
            Category::Firmware => "Firmware",
            Category::BluetoothBle => "Bluetooth / BLE",
            Category::RfidNfc => "RFID / NFC",
            Category::Sdr => "SDR",
            Category::ThreatIntelligence => "Threat Intelligence",
            Category::DefensiveSecurity => "Defensive Security",
            Category::IdsIps => "IDS / IPS",
            Category::Honeypots => "Honeypots",
            Category::ProxyTunneling => "Proxy / Tunneling",
            Category::Fuzzing => "Fuzzing",
            Category::Reporting => "Reporting",
            Category::SecurityUtilities => "Security Utilities",
        }
    }

    pub fn slug(&self) -> &'static str {
        match self {
            Category::Reconnaissance => "recon",
            Category::Osint => "osint",
            Category::NetworkSecurity => "network",
            Category::WebSecurity => "web",
            Category::ApiSecurity => "api",
            Category::Wireless => "wireless",
            Category::PasswordSecurity => "password",
            Category::VulnerabilityAssessment => "vuln",
            Category::ActiveDirectory => "ad",
            Category::ExploitDevelopment => "exploit",
            Category::ReverseEngineering => "reversing",
            Category::BinaryAnalysis => "binary",
            Category::MalwareAnalysis => "malware",
            Category::DigitalForensics => "forensics",
            Category::IncidentResponse => "ir",
            Category::MobileSecurity => "mobile",
            Category::CloudSecurity => "cloud",
            Category::ContainerSecurity => "container",
            Category::KubernetesSecurity => "k8s",
            Category::SourceCodeSecurity => "sast",
            Category::Cryptography => "crypto",
            Category::Steganography => "stego",
            Category::HardwareIot => "hardware",
            Category::Firmware => "firmware",
            Category::BluetoothBle => "bluetooth",
            Category::RfidNfc => "rfid",
            Category::Sdr => "sdr",
            Category::ThreatIntelligence => "threat-intel",
            Category::DefensiveSecurity => "defense",
            Category::IdsIps => "ids-ips",
            Category::Honeypots => "honeypots",
            Category::ProxyTunneling => "proxy",
            Category::Fuzzing => "fuzzing",
            Category::Reporting => "reporting",
            Category::SecurityUtilities => "utils",
        }
    }

    pub fn from_slug(s: &str) -> Option<Category> {
        Self::all().iter().find(|c| c.slug() == s).copied()
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subcategory {
    pub name: String,
    pub parent: Category,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolTag {
    pub tag: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Repository {
    Core,
    Extra,
    Community,
    BlackArch,
    Custom(String),
    Unknown,
}

impl Repository {
    pub fn name(&self) -> String {
        match self {
            Repository::Core => "Core".to_string(),
            Repository::Extra => "Extra".to_string(),
            Repository::Community => "Community".to_string(),
            Repository::BlackArch => "BlackArch".to_string(),
            Repository::Custom(name) => name.clone(),
            Repository::Unknown => "Unknown".to_string(),
        }
    }

    pub fn is_security_repo(&self) -> bool {
        matches!(self, Repository::BlackArch)
    }
}

impl fmt::Display for Repository {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
