use crate::catalog::{Category, Repository};
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationRule {
    pub categories: Vec<Category>,
    pub tags: Vec<String>,
    pub related: Vec<String>,
    pub detailed_description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationDb {
    pub rules: HashMap<String, ClassificationRule>,
    pub group_map: HashMap<String, Vec<Category>>,
    pub version: u32,
}

impl Default for ClassificationDb {
    fn default() -> Self {
        let mut group_map = HashMap::new();
        group_map.insert("blackarch-automobile".into(), vec![Category::HardwareIot]);
        group_map.insert(
            "blackarch-backdoor".into(),
            vec![Category::ExploitDevelopment],
        );
        group_map.insert("blackarch-bluetooth".into(), vec![Category::BluetoothBle]);
        group_map.insert("blackarch-crypto".into(), vec![Category::Cryptography]);
        group_map.insert(
            "blackarch-database".into(),
            vec![Category::SecurityUtilities],
        );
        group_map.insert(
            "blackarch-debugger".into(),
            vec![Category::ReverseEngineering],
        );
        group_map.insert(
            "blackarch-decompiler".into(),
            vec![Category::ReverseEngineering],
        );
        group_map.insert(
            "blackarch-disassembler".into(),
            vec![Category::ReverseEngineering],
        );
        group_map.insert("blackarch-dos".into(), vec![Category::NetworkSecurity]);
        group_map.insert(
            "blackarch-exploitation".into(),
            vec![Category::ExploitDevelopment],
        );
        group_map.insert(
            "blackarch-fingerprint".into(),
            vec![Category::SecurityUtilities],
        );
        group_map.insert(
            "blackarch-forensics".into(),
            vec![Category::DigitalForensics],
        );
        group_map.insert("blackarch-fuzzer".into(), vec![Category::Fuzzing]);
        group_map.insert("blackarch-hardware".into(), vec![Category::HardwareIot]);
        group_map.insert("honeypot".into(), vec![Category::Honeypots]);
        group_map.insert(
            "blackarch-keylogger".into(),
            vec![Category::MalwareAnalysis],
        );
        group_map.insert("blackarch-malware".into(), vec![Category::MalwareAnalysis]);
        group_map.insert("blackarch-misc".into(), vec![Category::SecurityUtilities]);
        group_map.insert("blackarch-mobile".into(), vec![Category::MobileSecurity]);
        group_map.insert("blackarch-network".into(), vec![Category::NetworkSecurity]);
        group_map.insert("blackarch-nfc".into(), vec![Category::RfidNfc]);
        group_map.insert("blackarch-osint".into(), vec![Category::Osint]);
        group_map.insert("blackarch-proxy".into(), vec![Category::ProxyTunneling]);
        group_map.insert("blackarch-recon".into(), vec![Category::Reconnaissance]);
        group_map.insert(
            "blackarch-reversing".into(),
            vec![Category::ReverseEngineering],
        );
        group_map.insert("blackarch-radio".into(), vec![Category::Sdr]);
        group_map.insert("blackarch-scanner".into(), vec![Category::Reconnaissance]);
        group_map.insert("blackarch-sniffer".into(), vec![Category::NetworkSecurity]);
        group_map.insert("blackarch-social".into(), vec![Category::Osint]);
        group_map.insert(
            "blackarch-software".into(),
            vec![Category::SecurityUtilities],
        );
        group_map.insert("blackarch-spoof".into(), vec![Category::NetworkSecurity]);
        group_map.insert("blackarch-stego".into(), vec![Category::Steganography]);
        group_map.insert("blackarch-tunnel".into(), vec![Category::ProxyTunneling]);
        group_map.insert("blackarch-web".into(), vec![Category::WebSecurity]);
        group_map.insert(
            "blackarch-windows".into(),
            vec![Category::SecurityUtilities],
        );
        group_map.insert("blackarch-wireless".into(), vec![Category::Wireless]);

        Self {
            rules: HashMap::new(),
            group_map,
            version: 1,
        }
    }
}

impl ClassificationDb {
    pub fn rule_for(&self, pkg_name: &str) -> Option<&ClassificationRule> {
        self.rules.get(pkg_name)
    }

    pub fn categories_for_groups(&self, groups: &[String]) -> Vec<Category> {
        let mut cats = Vec::new();
        for group in groups {
            if let Some(group_cats) = self.group_map.get(group.as_str()) {
                for cat in group_cats {
                    if !cats.contains(cat) {
                        cats.push(*cat);
                    }
                }
            }
        }
        cats
    }
}

pub struct Classifier {
    db: ClassificationDb,
}

impl Classifier {
    pub fn new(db: ClassificationDb) -> Self {
        Self { db }
    }

    pub fn classify(
        &self,
        pkg_name: &str,
        description: &str,
        groups: &[String],
        repository: &Repository,
    ) -> (Vec<Category>, Vec<String>, String) {
        if let Some(rule) = self.db.rule_for(pkg_name) {
            let detailed = rule
                .detailed_description
                .clone()
                .unwrap_or_else(|| description.to_string());
            return (rule.categories.clone(), rule.tags.clone(), detailed);
        }

        let mut categories = self.db.categories_for_groups(groups);
        let tags = self.infer_tags(pkg_name, description);

        if categories.is_empty() {
            categories = self.classify_by_name_and_desc(pkg_name, description);
        }

        if categories.is_empty() && repository.is_security_repo() {
            categories = vec![Category::SecurityUtilities];
        }

        (categories, tags, description.to_string())
    }

    fn infer_tags(&self, name: &str, desc: &str) -> Vec<String> {
        let mut tags = Vec::new();
        let combined = format!("{} {}", name.to_lowercase(), desc.to_lowercase());

        let tag_patterns = &[
            ("scanner", "scanner"),
            ("fuzzer", "fuzzer"),
            ("exploit", "exploit"),
            ("sniffer", "sniffer"),
            ("proxy", "proxy"),
            ("password", "password"),
            ("crack", "cracker"),
            ("brute", "brute-force"),
            ("forensic", "forensic"),
            ("malware", "malware"),
            ("revers", "reversing"),
            ("debug", "debugger"),
            ("packet", "packet"),
            ("wireless", "wireless"),
            ("wifi", "wifi"),
            ("bluetooth", "bluetooth"),
            ("web", "web"),
            ("http", "http"),
            ("sql", "sql"),
            ("ssh", "ssh"),
            ("dns", "dns"),
            ("network", "network"),
            ("osint", "osint"),
            ("recon", "recon"),
            ("enum", "enumeration"),
            ("vuln", "vulnerability"),
            ("scan", "scanning"),
            ("inject", "injection"),
            ("xss", "xss"),
            ("steganograph", "stego"),
            ("crypto", "crypto"),
            ("android", "android"),
            ("ios", "ios"),
            ("mobile", "mobile"),
            ("cloud", "cloud"),
            ("container", "container"),
            ("kubernetes", "kubernetes"),
            ("active directory", "active-directory"),
            ("mitm", "mitm"),
            ("phishing", "phishing"),
            ("keylog", "keylogger"),
            ("rootkit", "rootkit"),
            ("backdoor", "backdoor"),
            ("trojan", "trojan"),
            ("firmware", "firmware"),
            ("iot", "iot"),
            ("sdr", "sdr"),
            ("rfid", "rfid"),
            ("nfc", "nfc"),
        ];

        for (pattern, tag) in tag_patterns {
            if combined.contains(pattern) {
                tags.push(tag.to_string());
            }
        }

        tags
    }

    fn classify_by_name_and_desc(&self, name: &str, desc: &str) -> Vec<Category> {
        let combined = format!("{} {}", name.to_lowercase(), desc.to_lowercase());
        let mut cats = Vec::new();

        let mappings: &[(&[&str], Category)] = &[
            (
                &["scan", "nmap", "masscan", "zmap", "rustscan"],
                Category::Reconnaissance,
            ),
            (
                &["osint", "shodan", "censys", "theharvester"],
                Category::Osint,
            ),
            (
                &["wireshark", "tcpdump", "packet", "sniff"],
                Category::NetworkSecurity,
            ),
            (
                &["sqlmap", "nikto", "dirb", "gobuster", "ffuf", "burp", "web"],
                Category::WebSecurity,
            ),
            (
                &["aircrack", "kismet", "wifite", "reaver"],
                Category::Wireless,
            ),
            (
                &["hashcat", "john", "hydra", "medusa", "password"],
                Category::PasswordSecurity,
            ),
            (
                &["nuclei", "vulnerability", "cve", "exploit"],
                Category::VulnerabilityAssessment,
            ),
            (
                &["bloodhound", "ldap", "active directory", "kerberos"],
                Category::ActiveDirectory,
            ),
            (
                &["gdb", "radare", "ida", "ghidra", "reverse", "disassembl"],
                Category::ReverseEngineering,
            ),
            (
                &["volatility", "rekall", "forensic", "autopsy"],
                Category::DigitalForensics,
            ),
            (
                &["frida", "apk", "android", "drozer", "mobsf"],
                Category::MobileSecurity,
            ),
            (
                &["scout", "prowler", "cloud", "aws", "gcloud"],
                Category::CloudSecurity,
            ),
            (
                &["fuzz", "afl", "libfuzzer", "honggfuzz"],
                Category::Fuzzing,
            ),
            (
                &["stegano", "steghide", "stegsolve"],
                Category::Steganography,
            ),
            (
                &["mitmproxy", "proxychains", "tunnel", "chisel"],
                Category::ProxyTunneling,
            ),
            (
                &["malware", "yara", "clamav", "signature"],
                Category::MalwareAnalysis,
            ),
            (&["ids", "ips", "suricata", "snort"], Category::IdsIps),
            (&["honeypot", "cowrie", "dionaea"], Category::Honeypots),
            (
                &["radio", "sdr", "hackrf", "rtlsdr", "gnuradio"],
                Category::Sdr,
            ),
        ];

        for (keywords, category) in mappings {
            for kw in *keywords {
                if combined.contains(kw) && !cats.contains(category) {
                    cats.push(*category);
                }
            }
        }

        cats
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classifier_known_groups() {
        let db = ClassificationDb::default();
        let classifier = Classifier::new(db);

        let groups = vec!["blackarch-web".to_string()];
        let (cats, _, _) =
            classifier.classify("ffuf", "Fast web fuzzer", &groups, &Repository::BlackArch);
        assert!(cats.contains(&Category::WebSecurity));
    }

    #[test]
    fn test_infer_tags() {
        let db = ClassificationDb::default();
        let classifier = Classifier::new(db);
        let tags = classifier.infer_tags(
            "nmap",
            "Network exploration tool and security / port scanner",
        );
        assert!(tags.contains(&"scanner".to_string()));
        assert!(tags.contains(&"network".to_string()));
    }

    #[test]
    fn test_classify_by_name_fallback() {
        let db = ClassificationDb::default();
        let classifier = Classifier::new(db);
        let (cats, _, _) = classifier.classify(
            "wireshark",
            "Network protocol analyzer",
            &[],
            &Repository::Extra,
        );
        assert!(cats.contains(&Category::NetworkSecurity));
    }

    #[test]
    fn test_empty_fallback_to_utils() {
        let db = ClassificationDb::default();
        let classifier = Classifier::new(db);
        let (cats, _, _) =
            classifier.classify("some-random-tool", "A utility", &[], &Repository::BlackArch);
        assert!(cats.contains(&Category::SecurityUtilities));
    }
}
