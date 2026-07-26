use crate::catalog::tool::Tool;
use crate::catalog::{Category, Repository};

pub struct ToolKnowledge {
    pub what_it_does: String,
    pub how_it_works: String,
    pub use_cases: Vec<String>,
    pub attack_types: Vec<String>,
    pub difficulty: String,
    pub protocols: Vec<String>,
    pub key_features: Vec<String>,
    pub typical_workflow: Vec<String>,
    pub targets: Vec<String>,
    pub strengths: Vec<String>,
    pub limitations: Vec<String>,
    pub alternatives: Vec<String>,
    pub best_practices: Vec<String>,
}

pub fn get_knowledge(tool: &Tool) -> ToolKnowledge {
    let name = tool.name.to_lowercase();

    match name.as_str() {
        "nmap" => ToolKnowledge {
            what_it_does: "Nmap (Network Mapper) is the industry-standard open-source network scanner. It discovers hosts, identifies open ports, detects running services and their versions, fingerprints operating systems, and runs hundreds of optional NSE scripts.".into(),
            how_it_works: "Sends specially crafted packets to targets and analyzes responses to determine open ports, running services, OS, and firewall rules. Supports TCP connect, SYN stealth, UDP, and protocol-specific scans. The NSE (Nmap Scripting Engine) extends functionality with Lua scripts.".into(),
            use_cases: vec![
                "Network inventory and asset discovery".into(),
                "Security auditing and vulnerability assessment".into(),
                "Port scanning and service enumeration".into(),
                "OS fingerprinting and detection".into(),
                "Firewall/IDS evasion and testing".into(),
                "Network mapping and topology discovery".into(),
            ],
            attack_types: vec![
                "Reconnaissance".into(),
                "Service Enumeration".into(),
                "OS Detection".into(),
                "Vulnerability Scanning".into(),
            ],
            difficulty: "Beginner to Advanced".into(),
            protocols: vec![
                "TCP".into(), "UDP".into(), "ICMP".into(),
                "SCTP".into(), "ARP".into(), "IPv4".into(), "IPv6".into(),
            ],
            key_features: vec![
                "Port scanning (SYN, Connect, ACK, NULL, FIN, Xmas, UDP)".into(),
                "Service/version detection (-sV)".into(),
                "OS fingerprinting (-O)".into(),
                "Nmap Scripting Engine (NSE) with 600+ scripts".into(),
                "Host discovery (-sn, -Pn)".into(),
                "Aggressive scan mode (-A)".into(),
                "Output formats (normal, XML, grepable)".into(),
                "Timing templates (T0-T5) and firewall/IDS evasion".into(),
            ],
            typical_workflow: vec![
                "1. Discover live hosts: nmap -sn 192.168.1.0/24".into(),
                "2. Scan ports: nmap -sS -p- <target>".into(),
                "3. Detect services: nmap -sV -sC <target>".into(),
                "4. OS fingerprint: nmap -O <target>".into(),
                "5. Run scripts: nmap --script vuln <target>".into(),
            ],
            targets: vec![
                "IP addresses".into(), "Hostnames".into(),
                "Network ranges (CIDR)".into(), "Entire subnets".into(),
            ],
            strengths: vec![
                "Most comprehensive network scanner available".into(),
                "Massive NSE script library".into(),
                "Excellent documentation and community".into(),
                "Works on all major platforms".into(),
            ],
            limitations: vec![
                "Requires root for SYN scans and OS detection".into(),
                "Can be slow for full port scans".into(),
                "May trigger IDS/IPS alerts".into(),
            ],
            alternatives: vec![
                "masscan".into(), "rustscan".into(),
                "zmap".into(), "Unicornscan".into(),
            ],
            best_practices: vec![
                "Use -sS (SYN scan) for speed and stealth".into(),
                "Always scan with -sV for service version detection".into(),
                "Use timing templates (-T4) carefully".into(),
                "Combine with NSE scripts for deeper analysis".into(),
            ],
        },
        "arp-scan" => ToolKnowledge {
            what_it_does: "arp-scan is a network scanning tool that discovers hosts on a local network by sending ARP requests. It can identify all devices on a subnet including those that block ICMP/ping, since ARP operates at Layer 2.".into(),
            how_it_works: "Constructs and sends ARP requests to every address in a specified range on the local Ethernet segment. Displays any ARP responses, revealing IP address, MAC address, and vendor information of each responding host.".into(),
            use_cases: vec![
                "Local network host discovery".into(),
                "Detecting unauthorized or rogue devices".into(),
                "Network inventory and asset mapping".into(),
                "Identifying hosts that block ICMP/ping".into(),
                "Verifying DHCP scope assignments".into(),
            ],
            attack_types: vec![
                "Reconnaissance".into(),
                "Network Discovery".into(),
            ],
            difficulty: "Beginner".into(),
            protocols: vec!["ARP".into(), "Ethernet".into(), "IPv4".into()],
            key_features: vec![
                "Fast Layer 2 host discovery".into(),
                "Bypasses ICMP-blocking hosts".into(),
                "MAC vendor OUI lookup".into(),
                "Flexible address range specification".into(),
                "CIDR and wildcard notation".into(),
                "Auto-detect local network (-l)".into(),
            ],
            typical_workflow: vec![
                "1. Scan subnet: arp-scan 192.168.1.0/24".into(),
                "2. Specific range: arp-scan 192.168.1.1-100".into(),
                "3. Auto-detect: arp-scan -l".into(),
            ],
            targets: vec![
                "Local network hosts".into(),
                "Ethernet segments".into(),
            ],
            strengths: vec![
                "Very fast on local networks".into(),
                "Works when ICMP is blocked".into(),
                "Simple and lightweight".into(),
            ],
            limitations: vec![
                "Only works on local (Layer 2) networks".into(),
                "Requires root privileges".into(),
                "Cannot discover hosts beyond local subnet".into(),
            ],
            alternatives: vec![
                "nmap -sn".into(), "netdiscover".into(), "arping".into(),
            ],
            best_practices: vec![
                "Use -l for automatic local network detection".into(),
                "Use -O for MAC vendor identification".into(),
                "Run with sudo for raw packet access".into(),
            ],
        },
        "sqlmap" => ToolKnowledge {
            what_it_does: "sqlmap is an automatic SQL injection detection and exploitation tool. It supports a wide range of SQL database backends and can enumerate databases, tables, columns, and dump data from vulnerable targets.".into(),
            how_it_works: "Sends crafted HTTP requests with SQL injection payloads and analyzes responses to detect injection points. Once a vulnerability is found, it can extract data, escalate privileges, and attempt OS command execution.".into(),
            use_cases: vec![
                "SQL injection vulnerability detection".into(),
                "Database enumeration and data extraction".into(),
                "Automated exploitation of SQL injection flaws".into(),
                "Testing web application input fields".into(),
                "Bypassing Web Application Firewalls (WAF)".into(),
            ],
            attack_types: vec![
                "Injection".into(),
                "Data Exfiltration".into(),
                "Privilege Escalation".into(),
            ],
            difficulty: "Beginner to Intermediate".into(),
            protocols: vec![
                "HTTP".into(), "HTTPS".into(), "MySQL".into(),
                "PostgreSQL".into(), "MSSQL".into(), "Oracle".into(), "SQLite".into(),
            ],
            key_features: vec![
                "6+ SQL injection techniques (boolean-blind, time-based, error-based, UNION, stacked, out-of-band)".into(),
                "Database enumeration for MySQL, PostgreSQL, MSSQL, Oracle, SQLite".into(),
                "Automated data extraction and dumping".into(),
                "OS shell access via database functions".into(),
                "WAF/IPS bypass with tamper scripts".into(),
                "HTTP request parsing from Burp/OWASP ZAP".into(),
            ],
            typical_workflow: vec![
                "1. Test URL parameter: sqlmap -u 'http://target/page?id=1'".into(),
                "2. Enumerate databases: sqlmap -u '<URL>' --dbs".into(),
                "3. Dump table: sqlmap -u '<URL>' -D db -T users --dump".into(),
                "4. OS shell: sqlmap -u '<URL>' --os-shell".into(),
            ],
            targets: vec![
                "Web applications".into(), "REST APIs".into(),
                "URL parameters".into(), "POST data".into(), "Cookies".into(),
            ],
            strengths: vec![
                "Automates complex SQL injection attacks".into(),
                "Supports virtually all SQL databases".into(),
                "Active development with regular updates".into(),
                "Extensive tamper script library".into(),
            ],
            limitations: vec![
                "Only tests for SQL injection".into(),
                "Can generate large amounts of traffic".into(),
                "May modify or delete data during exploitation".into(),
            ],
            alternatives: vec![
                "jSQL Injection".into(), "Havij".into(),
                "BBQSQL".into(), "NoSQLMap".into(),
            ],
            best_practices: vec![
                "Always test on authorized targets only".into(),
                "Use --batch for non-interactive scanning".into(),
                "Combine with --tamper scripts for WAF evasion".into(),
                "Use -r to load requests from Burp Suite".into(),
            ],
        },
        "nikto" => ToolKnowledge {
            what_it_does: "Nikto is an open-source web server scanner that tests for dangerous files and CGIs, outdated server versions, server configuration issues, and over 6700 potentially dangerous files and programs.".into(),
            how_it_works: "Sends crafted HTTP requests to the target web server, checking its database of 6700+ dangerous files, 1250+ outdated server version checks, and version-specific problems on over 270 servers.".into(),
            use_cases: vec![
                "Web server vulnerability scanning".into(),
                "Server configuration auditing".into(),
                "CGI and script testing".into(),
                "Outdated software detection".into(),
            ],
            attack_types: vec![
                "Vulnerability Scanning".into(),
                "Configuration Audit".into(),
            ],
            difficulty: "Beginner".into(),
            protocols: vec!["HTTP".into(), "HTTPS".into()],
            key_features: vec![
                "6700+ dangerous files and programs checks".into(),
                "1250+ outdated server version checks".into(),
                "Server configuration issue detection".into(),
                "SSL and proxy support".into(),
                "Tuning and caching options".into(),
            ],
            typical_workflow: vec![
                "1. Basic scan: nikto -h http://target".into(),
                "2. Specific port: nikto -h target -p 8080".into(),
                "3. Tuned scan: nikto -h target -Tuning 123b".into(),
                "4. SSL scan: nikto -h target -ssl".into(),
            ],
            targets: vec![
                "Web servers".into(),
                "Web applications".into(),
            ],
            strengths: vec![
                "Comprehensive database of known checks".into(),
                "Easy to use".into(),
                "Good for initial web assessment".into(),
            ],
            limitations: vec![
                "Noisy and easily detected by IDS".into(),
                "Slow compared to modern scanners".into(),
                "High false positive rate".into(),
            ],
            alternatives: vec![
                "nuclei".into(), "nmap --script http-*".into(),
                "w3af".into(), "whatweb".into(),
            ],
            best_practices: vec![
                "Use as initial quick scan only".into(),
                "Combine with nuclei for modern vulnerabilities".into(),
                "Use -Tuning to reduce false positives".into(),
            ],
        },
        "ffuf" => ToolKnowledge {
            what_it_does: "FFUF (Fuzz Faster U Fool) is a fast web fuzzer for directory/file discovery, virtual host enumeration, parameter fuzzing, and POST data mutation with highly configurable filtering and matching.".into(),
            how_it_works: "Sends HTTP requests for every word in the provided wordlist, substituting the FUZZ keyword in the URL, headers, or POST data. Filters responses by status code, size, word count, or regex to identify interesting results.".into(),
            use_cases: vec![
                "Directory and file discovery".into(),
                "Virtual host enumeration".into(),
                "Parameter fuzzing and brute-force".into(),
                "POST data fuzzing".into(),
                "Header fuzzing".into(),
                "Recursive directory discovery".into(),
            ],
            attack_types: vec![
                "Discovery".into(),
                "Brute Force".into(),
                "Fuzzing".into(),
            ],
            difficulty: "Beginner".into(),
            protocols: vec!["HTTP".into(), "HTTPS".into()],
            key_features: vec![
                "Extremely fast performance (written in Go)".into(),
                "Filter by status code, size, word count, regex".into(),
                "FUZZ keyword placement anywhere in request".into(),
                "Rate limiting and throttling".into(),
                "Recursion for deep directory discovery".into(),
                "Multiple output formats (JSON, HTML, CSV)".into(),
            ],
            typical_workflow: vec![
                "1. Directory fuzzing: ffuf -u http://target/FUZZ -w wordlist.txt".into(),
                "2. Filter 404s: ffuf -u http://target/FUZZ -w wordlist.txt -fc 404".into(),
                "3. Vhost fuzzing: ffuf -u http://target -H 'Host: FUZZ.target.com' -w subdomains.txt".into(),
                "4. POST fuzzing: ffuf -u http://target/login -X POST -d 'user=FUZZ' -w users.txt".into(),
            ],
            targets: vec![
                "Web servers".into(), "REST APIs".into(), "Web applications".into(),
            ],
            strengths: vec![
                "Extremely fast performance".into(),
                "Flexible FUZZ keyword placement".into(),
                "Powerful filtering system".into(),
                "Simple and intuitive CLI".into(),
            ],
            limitations: vec![
                "Requires a good wordlist for results".into(),
                "No built-in vulnerability detection".into(),
            ],
            alternatives: vec![
                "gobuster".into(), "dirsearch".into(),
                "feroxbuster".into(), "wfuzz".into(),
            ],
            best_practices: vec![
                "Use SecLists wordlists for coverage".into(),
                "Always filter by response size (-fc)".into(),
                "Start with common extensions (-x php,html,js)".into(),
                "Use -ac for auto-calibration".into(),
            ],
        },
        "gobuster" => ToolKnowledge {
            what_it_does: "Gobuster is a directory/file, DNS subdomain, and virtual host brute-force tool. It supports multiple modes for content discovery, DNS enumeration, and vhost discovery with multithreaded performance.".into(),
            how_it_works: "Sends HTTP requests (dir mode), DNS queries (dns mode), or host header requests (vhost mode) for each wordlist entry. Reports responses that differ from the baseline to identify valid results.".into(),
            use_cases: vec![
                "Directory and file brute-forcing".into(),
                "DNS subdomain enumeration".into(),
                "Virtual host discovery".into(),
                "Web content discovery".into(),
            ],
            attack_types: vec![
                "Discovery".into(),
                "Enumeration".into(),
            ],
            difficulty: "Beginner".into(),
            protocols: vec!["HTTP".into(), "HTTPS".into(), "DNS".into()],
            key_features: vec![
                "Multiple modes: dir, dns, vhost, fuzz".into(),
                "Multithreaded for speed".into(),
                "Status code and size-based filtering".into(),
                "Custom HTTP headers and cookies".into(),
                "Recursive directory discovery".into(),
                "SSL/TLS support".into(),
            ],
            typical_workflow: vec![
                "1. Dir mode: gobuster dir -u http://target -w wordlist.txt".into(),
                "2. DNS mode: gobuster dns -d target.com -w subdomains.txt".into(),
                "3. Vhost mode: gobuster vhost -u http://target -w vhosts.txt".into(),
            ],
            targets: vec![
                "Web servers".into(), "DNS servers".into(), "Virtual hosts".into(),
            ],
            strengths: vec![
                "Simple CLI with clear modes".into(),
                "Good multithreading performance".into(),
                "Wide wordlist compatibility".into(),
            ],
            limitations: vec![
                "Less flexible than ffuf".into(),
                "No POST data fuzzing in dir mode".into(),
            ],
            alternatives: vec![
                "ffuf".into(), "feroxbuster".into(),
                "dirsearch".into(), "wfuzz".into(),
            ],
            best_practices: vec![
                "Use -t 50 or higher for faster scans".into(),
                "Combine with SecLists wordlists".into(),
                "Use -b to exclude specific status codes".into(),
            ],
        },
        "nuclei" => ToolKnowledge {
            what_it_does: "Nuclei is a fast, template-based vulnerability scanner using community-contributed YAML templates to detect CVEs, misconfigurations, exposed panels, and other security issues across web apps, networks, and infrastructure.".into(),
            how_it_works: "Sends HTTP requests or runs network protocols defined in YAML templates and matches responses against expected patterns. Templates define the request, matchers, and extractors for each vulnerability check.".into(),
            use_cases: vec![
                "CVE detection across web applications".into(),
                "Configuration exposure scanning".into(),
                "Technology-specific vulnerability checks".into(),
                "Batch scanning of multiple targets".into(),
                "CI/CD security integration".into(),
            ],
            attack_types: vec![
                "Vulnerability Scanning".into(),
                "Misconfiguration Detection".into(),
                "Information Disclosure".into(),
            ],
            difficulty: "Beginner".into(),
            protocols: vec![
                "HTTP".into(), "HTTPS".into(), "TCP".into(),
                "UDP".into(), "SSL".into(), "DNS".into(),
                "SSH".into(), "SMB".into(),
            ],
            key_features: vec![
                "Community-driven template library (9000+ templates)".into(),
                "Supports multiple protocols beyond HTTP".into(),
                "Severity-based template filtering".into(),
                "Rate limiting and retry logic".into(),
                "Template customization and creation".into(),
                "Tags for organized scanning".into(),
            ],
            typical_workflow: vec![
                "1. Basic scan: nuclei -u http://target".into(),
                "2. CVE only: nuclei -u http://target -t cves/".into(),
                "3. Severe only: nuclei -u http://target -severity critical,high".into(),
                "4. Batch: nuclei -l urls.txt -t severe/ -o results.txt".into(),
            ],
            targets: vec![
                "Web applications".into(), "APIs".into(),
                "Network services".into(), "Cloud infrastructure".into(),
            ],
            strengths: vec![
                "Massive template library".into(),
                "Very fast execution".into(),
                "Active community".into(),
                "Easy to write custom templates".into(),
            ],
            limitations: vec![
                "Templates can produce false positives".into(),
                "Large scans generate significant traffic".into(),
                "Requires regular template updates".into(),
            ],
            alternatives: vec![
                "nikto".into(), "nmap --script".into(),
                "OpenVAS".into(), "Nessus".into(),
            ],
            best_practices: vec![
                "Keep templates updated: nuclei -update-templates".into(),
                "Use -severity to focus on critical findings".into(),
                "Combine with httpx for probing before scanning".into(),
                "Use -rl to limit request rate".into(),
            ],
        },
        "wireshark-cli" | "tshark" => ToolKnowledge {
            what_it_does: "tshark is the command-line version of Wireshark, the world's foremost network protocol analyzer. It captures and analyzes network traffic with deep protocol inspection and hundreds of protocol dissectors.".into(),
            how_it_works: "Captures packets from a network interface or reads from a capture file, decodes protocol headers and payloads, and displays information in human-readable format using the same dissector engine as Wireshark GUI.".into(),
            use_cases: vec![
                "Network traffic analysis".into(),
                "Protocol debugging".into(),
                "Security incident investigation".into(),
                "Network forensics".into(),
                "Performance troubleshooting".into(),
            ],
            attack_types: vec![
                "Analysis".into(),
                "Forensics".into(),
                "Traffic Inspection".into(),
            ],
            difficulty: "Intermediate to Advanced".into(),
            protocols: vec![
                "TCP".into(), "UDP".into(), "HTTP".into(), "HTTPS".into(),
                "DNS".into(), "ARP".into(), "ICMP".into(), "SSH".into(),
                "TLS".into(), "2000+ protocols".into(),
            ],
            key_features: vec![
                "Deep protocol dissection (2000+ protocols)".into(),
                "Capture and display filters (Wireshark syntax)".into(),
                "Statistics and conversation analysis".into(),
                "TLS decryption with keylog file".into(),
                "IO graphs and flow analysis".into(),
                "Lua scripting support".into(),
            ],
            typical_workflow: vec![
                "1. Capture: tshark -i eth0".into(),
                "2. Filter HTTP: tshark -i eth0 -Y 'http'".into(),
                "3. Read file: tshark -r capture.pcap".into(),
                "4. Stats: tshark -r file.pcap -z conv,tcp".into(),
            ],
            targets: vec![
                "Network interfaces".into(),
                "PCAP files".into(),
                "Network traffic".into(),
            ],
            strengths: vec![
                "Most comprehensive protocol analysis tool".into(),
                "Huge protocol support".into(),
                "Powerful filtering".into(),
                "Active development".into(),
            ],
            limitations: vec![
                "Requires root for live capture".into(),
                "Steep learning curve for filters".into(),
                "Can be overwhelming for beginners".into(),
            ],
            alternatives: vec![
                "tcpdump".into(), "ngrep".into(), "Wireshark (GUI)".into(),
            ],
            best_practices: vec![
                "Use capture filters (-f) to reduce volume".into(),
                "Use display filters (-Y) for analysis".into(),
                "Save captures for later analysis (-w)".into(),
                "Use -z statistics for quick summaries".into(),
            ],
        },
        "hashcat" => ToolKnowledge {
            what_it_does: "Hashcat is the world's fastest password recovery utility. It supports over 300 hash types and leverages GPU acceleration (CUDA, OpenCL, HIP) for massive parallelism in password cracking operations.".into(),
            how_it_works: "Loads a hash file and wordlist/rules, generates candidate passwords, computes their hashes, and compares against target hashes. GPU acceleration allows billions of guesses per second for fast hash types like MD5 and NTLM.".into(),
            use_cases: vec![
                "Password hash cracking".into(),
                "Credential recovery from dumps".into(),
                "Password policy auditing".into(),
                "Penetration testing credential access".into(),
                "Digital forensics investigations".into(),
            ],
            attack_types: vec![
                "Credential Access".into(),
                "Password Cracking".into(),
            ],
            difficulty: "Intermediate".into(),
            protocols: vec![
                "MD5".into(), "SHA-1".into(), "SHA-256".into(),
                "SHA-512".into(), "bcrypt".into(), "scrypt".into(),
                "NTLM".into(), "Kerberos".into(), "WPA/WPA2".into(),
            ],
            key_features: vec![
                "GPU-accelerated cracking (CUDA, OpenCL, HIP, Metal)".into(),
                "300+ hash algorithm support".into(),
                "Dictionary, brute-force, mask, hybrid, and rule-based attacks".into(),
                "Session management with pause/resume".into(),
                "Distributed cracking across multiple GPUs".into(),
                "Custom rule engine for password mutations".into(),
            ],
            typical_workflow: vec![
                "1. Identify hash type: hashcat --example-hashes | grep -B1 'keyword'".into(),
                "2. Crack MD5: hashcat -m 0 hashes.txt wordlist.txt".into(),
                "3. Apply rules: hashcat -m 0 hashes.txt wordlist.txt -r rules/best64.rule".into(),
                "4. Mask attack: hashcat -m 0 -a 3 hashes.txt '?a?a?a?a?a?a'".into(),
            ],
            targets: vec![
                "Password hashes".into(), "NTLM hashes".into(),
                "Kerberos tickets".into(), "WPA handshakes".into(),
                "Shadow files".into(),
            ],
            strengths: vec![
                "Fastest password cracker available".into(),
                "Massive GPU support".into(),
                "Most comprehensive hash type coverage".into(),
                "Active development".into(),
            ],
            limitations: vec![
                "Requires powerful GPU for fast results".into(),
                "Memory-hard hashes (bcrypt, scrypt) are slower".into(),
                "Complex rule syntax has learning curve".into(),
            ],
            alternatives: vec![
                "john (John the Ripper)".into(),
                "bcrypt".into(), "oclHashcat (legacy)".into(),
            ],
            best_practices: vec![
                "Start with dictionary attacks before brute force".into(),
                "Use rules for mutations (rules/best64.rule)".into(),
                "Use -i for incremental mode with mask attacks".into(),
                "Monitor GPU temperature during long sessions".into(),
            ],
        },
        "john" => ToolKnowledge {
            what_it_does: "John the Ripper is a versatile password cracker supporting hundreds of hash types. It features CPU-based and GPU-accelerated modes with intelligent password candidate generation and automatic format detection.".into(),
            how_it_works: "Loads password hashes and tries candidates using multiple modes: single crack (using user info), wordlist, incremental (brute-force), and external. The --show flag displays previously cracked passwords.".into(),
            use_cases: vec![
                "Password hash cracking".into(),
                "Offline credential recovery".into(),
                "Password auditing".into(),
                "Forensic investigations".into(),
            ],
            attack_types: vec![
                "Credential Access".into(),
                "Password Cracking".into(),
            ],
            difficulty: "Intermediate".into(),
            protocols: vec![
                "MD5".into(), "SHA-*".into(), "bcrypt".into(),
                "DES".into(), "NTLM".into(), "Kerberos".into(), "200+ formats".into(),
            ],
            key_features: vec![
                "200+ hash format support".into(),
                "Automatic hash type detection".into(),
                "Multiple attack modes (single, wordlist, incremental, external)".into(),
                "Password policy checking".into(),
                "GPU acceleration".into(),
                "Session management".into(),
            ],
            typical_workflow: vec![
                "1. Auto-detect and crack: john hashes.txt".into(),
                "2. Wordlist mode: john --wordlist=rockyou.txt hashes.txt".into(),
                "3. Show cracked: john --show hashes.txt".into(),
                "4. Format specific: john --format=raw-md5 hashes.txt".into(),
            ],
            targets: vec![
                "Shadow files".into(), "Password hashes".into(),
                "Kerberos tickets".into(), "Wireless handshakes".into(),
            ],
            strengths: vec![
                "Automatic format detection".into(),
                "Versatile attack modes".into(),
                "Strong rule engine".into(),
                "Well-established and trusted".into(),
            ],
            limitations: vec![
                "Slower than hashcat for GPU cracking".into(),
                "UI is less polished".into(),
                "Some formats only in jumbo version".into(),
            ],
            alternatives: vec![
                "hashcat".into(), "ophcrack".into(), "RainbowCrack".into(),
            ],
            best_practices: vec![
                "Let john auto-detect the format first".into(),
                "Use --format if auto-detect fails".into(),
                "Combine with wordlists and rules".into(),
                "Use --show to review previous sessions".into(),
            ],
        },
        "hydra" => ToolKnowledge {
            what_it_does: "THC-Hydra is a fast network logon cracker supporting 50+ protocols including HTTP, SSH, FTP, SMB, RDP, and databases. It performs online brute-force and dictionary attacks against authentication services.".into(),
            how_it_works: "Establishes connections to target services and tries username/password combinations in parallel. Supports multiple authentication protocols and can use multiple parallel tasks for speed.".into(),
            use_cases: vec![
                "Online password brute-forcing".into(),
                "Service authentication testing".into(),
                "Credential stuffing".into(),
                "Penetration testing password auditing".into(),
            ],
            attack_types: vec![
                "Brute Force".into(),
                "Credential Stuffing".into(),
            ],
            difficulty: "Beginner to Intermediate".into(),
            protocols: vec![
                "SSH".into(), "FTP".into(), "HTTP".into(), "HTTPS".into(),
                "SMB".into(), "RDP".into(), "MySQL".into(), "PostgreSQL".into(),
                "MongoDB".into(), "LDAP".into(),
            ],
            key_features: vec![
                "50+ protocol support".into(),
                "Parallel task execution".into(),
                "HTTP form-based authentication".into(),
                "SSL/TLS support".into(),
                "Restore interrupted sessions".into(),
                "IPv4 and IPv6 support".into(),
            ],
            typical_workflow: vec![
                "1. SSH brute-force: hydra -l admin -P wordlist.txt target ssh".into(),
                "2. HTTP form: hydra -l admin -P pass.txt target http-post-form '/login:user=^USER^&pass=^PASS^:F=incorrect'".into(),
                "3. FTP: hydra -L users.txt -P wordlist.txt target ftp".into(),
            ],
            targets: vec![
                "SSH servers".into(), "FTP servers".into(),
                "Web login forms".into(), "RDP".into(),
                "Databases".into(), "SMTP".into(),
            ],
            strengths: vec![
                "Fast parallel attacks".into(),
                "Wide protocol support".into(),
                "Simple command syntax".into(),
                "Session restore capability".into(),
            ],
            limitations: vec![
                "May trigger account lockouts".into(),
                "No distributed cracking".into(),
                "Limited HTTP form complexity".into(),
            ],
            alternatives: vec![
                "medusa".into(), "ncrack".into(),
                "patator".into(), "CrackMapExec".into(),
            ],
            best_practices: vec![
                "Use -t to control parallel connections".into(),
                "Use -f to stop on first valid credential".into(),
                "Combine with known username lists".into(),
                "Be careful with lockout policies".into(),
            ],
        },
        "masscan" => ToolKnowledge {
            what_it_does: "Masscan is the fastest Internet port scanner, capable of scanning the entire IPv4 address space in under 5 minutes. It transmits at 10+ million packets/second using its own custom TCP/IP stack.".into(),
            how_it_works: "Generates and sends raw TCP/IP packets directly from the network card, bypassing the OS kernel network stack. Uses asynchronous transmission and adaptive rate control to achieve extreme speeds.".into(),
            use_cases: vec![
                "Internet-wide port scanning".into(),
                "Large-scale network surveys".into(),
                "Quick discovery of exposed services".into(),
                "Internet research and census".into(),
            ],
            attack_types: vec![
                "Reconnaissance".into(),
                "Discovery".into(),
            ],
            difficulty: "Intermediate".into(),
            protocols: vec![
                "TCP".into(), "UDP".into(), "ICMP".into(), "ARP".into(),
            ],
            key_features: vec![
                "10+ million packets/second scanning rate".into(),
                "Custom TCP/IP stack (bypasses OS)".into(),
                "Asynchronous transmission".into(),
                "Banner grabbing".into(),
                "Output compatible with nmap".into(),
                "Adaptive rate limiting".into(),
            ],
            typical_workflow: vec![
                "1. Scan all ports: masscan 0.0.0.0/0 -p0-65535 --rate=1000".into(),
                "2. Specific ports: masscan 192.168.1.0/24 -p80,443".into(),
                "3. Banner grab: masscan 10.0.0.0/8 -p80 --banners".into(),
            ],
            targets: vec![
                "Entire IP ranges".into(),
                "Network subnets".into(),
                "Individual hosts".into(),
            ],
            strengths: vec![
                "Fastest port scanner in existence".into(),
                "Can scan the entire internet".into(),
                "Custom TCP/IP stack avoids OS limitations".into(),
            ],
            limitations: vec![
                "Less accurate than nmap for service detection".into(),
                "Requires root privileges".into(),
                "May miss responses at very high rates".into(),
            ],
            alternatives: vec![
                "nmap".into(), "zmap".into(), "rustscan".into(),
            ],
            best_practices: vec![
                "Use --rate to control speed".into(),
                "Output to file for nmap import: -oL or -oX".into(),
                "Use --banners for service identification".into(),
                "Be aware of legal implications of large-scale scanning".into(),
            ],
        },
        "amass" => ToolKnowledge {
            what_it_does: "OWASP Amass is a comprehensive network mapping and attack surface discovery tool. It performs OSINT-driven subdomain enumeration using 30+ data sources and active reconnaissance techniques.".into(),
            how_it_works: "Aggregates data from APIs, certificate transparency logs, DNS brute-forcing, web scraping, and search engines to discover subdomains and map an organization's external attack surface.".into(),
            use_cases: vec![
                "Subdomain enumeration".into(),
                "Attack surface mapping".into(),
                "Asset discovery for bug bounty".into(),
                "Infrastructure inventory".into(),
                "Continuous monitoring of external assets".into(),
            ],
            attack_types: vec![
                "Reconnaissance".into(),
                "OSINT".into(),
                "Subdomain Enumeration".into(),
            ],
            difficulty: "Intermediate".into(),
            protocols: vec![
                "DNS".into(), "HTTP".into(), "HTTPS".into(),
                "Certificate Transparency".into(),
            ],
            key_features: vec![
                "30+ OSINT data sources".into(),
                "Active and passive enumeration modes".into(),
                "Certificate transparency log analysis".into(),
                "DNS brute-forcing".into(),
                "ASN and IP mapping".into(),
                "Relationship graph building".into(),
            ],
            typical_workflow: vec![
                "1. Passive recon: amass enum -passive -d target.com".into(),
                "2. Active recon: amass enum -active -d target.com".into(),
                "3. Visualize: amass viz -d3 -d target.com".into(),
                "4. Track changes: amass track -d target.com".into(),
            ],
            targets: vec![
                "Domain names".into(),
                "Organizations".into(),
                "Autonomous Systems (ASN)".into(),
            ],
            strengths: vec![
                "Most comprehensive subdomain enumeration tool".into(),
                "Massive data source coverage".into(),
                "Active and passive modes".into(),
                "Good visualization".into(),
            ],
            limitations: vec![
                "Active mode is noisy and detectable".into(),
                "Slow for large domains".into(),
                "Requires API keys for many data sources".into(),
            ],
            alternatives: vec![
                "subfinder".into(), "sublist3r".into(),
                "assetfinder".into(), "crobat".into(),
            ],
            best_practices: vec![
                "Use passive mode first to avoid detection".into(),
                "Configure API keys for maximum coverage".into(),
                "Combine with subfinder for broader results".into(),
            ],
        },
        "subfinder" => ToolKnowledge {
            what_it_does: "Subfinder is a fast passive subdomain enumeration tool that uses many passive sources including certificate transparency logs, search engines, and threat intelligence platforms.".into(),
            how_it_works: "Queries multiple passive data sources (CT logs, search engines, APIs, DNS records) to find subdomains without directly interacting with the target infrastructure, making it completely stealthy.".into(),
            use_cases: vec![
                "Passive subdomain discovery".into(),
                "Bug bounty reconnaissance".into(),
                "Asset inventory".into(),
                "Feeding results into other tools".into(),
            ],
            attack_types: vec![
                "Reconnaissance".into(),
                "OSINT".into(),
            ],
            difficulty: "Beginner".into(),
            protocols: vec![
                "DNS".into(), "HTTP".into(),
                "Certificate Transparency".into(),
            ],
            key_features: vec![
                "30+ passive data sources".into(),
                "Zero interaction with target (fully passive)".into(),
                "Fast execution".into(),
                "Multiple output formats".into(),
                "Pipes output to other tools".into(),
                "Configurable source selection".into(),
            ],
            typical_workflow: vec![
                "1. Basic enumeration: subfinder -d target.com".into(),
                "2. Specific sources: subfinder -d target.com -sources crtsh,rapiddns".into(),
                "3. Save results: subfinder -d target.com -o subdomains.txt".into(),
                "4. Pipe to httpx: subfinder -d target.com -silent | httpx".into(),
            ],
            targets: vec![
                "Domain names".into(),
            ],
            strengths: vec![
                "Very fast".into(),
                "Completely passive and stealthy".into(),
                "Simple CLI".into(),
                "Great for piping to other tools".into(),
            ],
            limitations: vec![
                "Passive only - will not find unindexed subdomains".into(),
                "No active DNS validation".into(),
                "Results may be outdated".into(),
            ],
            alternatives: vec![
                "amass".into(), "assetfinder".into(),
                "crobat".into(), "dnsx".into(),
            ],
            best_practices: vec![
                "Use as first step in recon pipeline".into(),
                "Pipe output to httpx for probing".into(),
                "Combine with amass for broader coverage".into(),
            ],
        },
        "metasploit" | "msfconsole" => ToolKnowledge {
            what_it_does: "Metasploit is the world's most used penetration testing framework. It provides exploit development, payload delivery, post-exploitation, and reconnaissance capabilities across thousands of modules.".into(),
            how_it_works: "Loads exploit modules that leverage specific vulnerabilities to deliver payloads to target systems. Payloads provide interactive shells, meterpreter sessions, or other post-exploitation capabilities.".into(),
            use_cases: vec![
                "Penetration testing".into(),
                "Exploit development and testing".into(),
                "Post-exploitation activities".into(),
                "Social engineering attacks".into(),
                "Vulnerability verification".into(),
            ],
            attack_types: vec![
                "Exploitation".into(),
                "Post-Exploitation".into(),
                "Social Engineering".into(),
                "Reconnaissance".into(),
            ],
            difficulty: "Intermediate to Advanced".into(),
            protocols: vec![
                "TCP".into(), "UDP".into(), "HTTP".into(),
                "SMB".into(), "RDP".into(), "SSH".into(),
                "Database protocols".into(),
            ],
            key_features: vec![
                "3000+ exploit modules".into(),
                "Meterpreter post-exploitation agent".into(),
                "Payload generation framework".into(),
                "Auxiliary modules for scanning/fuzzing".into(),
                "Post-exploitation module library".into(),
                "Database integration (hosts, services, creds)".into(),
                "RPC API for external tools".into(),
            ],
            typical_workflow: vec![
                "1. Start: msfconsole".into(),
                "2. Search exploit: search eternalblue".into(),
                "3. Use module: use exploit/windows/smb/ms17_010_eternalblue".into(),
                "4. Configure: set RHOSTS target".into(),
                "5. Exploit: exploit".into(),
            ],
            targets: vec![
                "Network services".into(), "Web applications".into(),
                "Client-side vulnerabilities".into(), "Wireless networks".into(),
            ],
            strengths: vec![
                "Most comprehensive exploitation framework".into(),
                "Massive module library".into(),
                "Active development".into(),
                "Industry standard".into(),
            ],
            limitations: vec![
                "Large resource footprint".into(),
                "Complex for beginners".into(),
                "Some exploits may be unstable".into(),
            ],
            alternatives: vec![
                "Core Impact".into(), "Canvas".into(), "Cobalt Strike".into(),
            ],
            best_practices: vec![
                "Always get written authorization before testing".into(),
                "Use database for tracking findings".into(),
                "Start with auxiliary modules for recon".into(),
                "Use Meterpreter for post-exploitation".into(),
            ],
        },
        "crackmapexec" | "cme" => ToolKnowledge {
            what_it_does: "CrackMapExec (CME) is a post-exploitation tool for Active Directory environments. It supports SMB, LDAP, WinRM, SSH, RDP and more for network enumeration, credential attacks, and lateral movement.".into(),
            how_it_works: "Authenticates to multiple target hosts simultaneously using provided credentials, then performs requested actions like enumeration, command execution, or credential harvesting. Results stored in SQLite database.".into(),
            use_cases: vec![
                "Active Directory enumeration".into(),
                "Credential spraying".into(),
                "Lateral movement".into(),
                "Password spraying".into(),
                "SMB shares enumeration".into(),
                "Token manipulation".into(),
            ],
            attack_types: vec![
                "Credential Access".into(),
                "Lateral Movement".into(),
                "Enumeration".into(),
            ],
            difficulty: "Intermediate".into(),
            protocols: vec![
                "SMB".into(), "LDAP".into(), "WinRM".into(),
                "SSH".into(), "RDP".into(), "MSSQL".into(),
                "MySQL".into(), "FTP".into(), "SNMP".into(),
            ],
            key_features: vec![
                "Multi-protocol support".into(),
                "Credential spraying across many hosts".into(),
                "SMB share and file enumeration".into(),
                "Bloodhound integration".into(),
                "Pass-the-Hash and Pass-the-Ticket".into(),
                "Local and remote command execution".into(),
                "LAPS and gMSA support".into(),
                "SQLite database for results".into(),
            ],
            typical_workflow: vec![
                "1. Spray credentials: cme smb 192.168.1.0/24 -u users.txt -p Password1".into(),
                "2. Enumerate shares: cme smb target -u user -p pass --shares".into(),
                "3. Execute command: cme smb target -u user -p pass -x 'whoami'".into(),
                "4. Dump SAM: cme smb target -u user -p pass --sam".into(),
            ],
            targets: vec![
                "Active Directory domains".into(), "Windows networks".into(),
                "Linux/Unix SSH".into(), "MSSQL databases".into(),
            ],
            strengths: vec![
                "Multi-protocol support".into(),
                "Fast credential spraying".into(),
                "Excellent AD integration".into(),
                "Tracks results in database".into(),
            ],
            limitations: vec![
                "Primarily focused on Windows/AD".into(),
                "May trigger detection systems".into(),
                "Credential stuffing can lock accounts".into(),
            ],
            alternatives: vec![
                "Impacket suite".into(), "Evil-WinRM".into(),
                "Rubeus".into(), "PowerView".into(),
            ],
            best_practices: vec![
                "Use --verbose for detailed output".into(),
                "Store results in database for tracking".into(),
                "Use specific protocols when possible".into(),
                "Combine with Bloodhound for AD mapping".into(),
            ],
        },
        "binwalk" => ToolKnowledge {
            what_it_does: "Binwalk is a firmware analysis tool for searching binary images for embedded files and executable code. It identifies common file formats and filesystem structures for firmware component extraction.".into(),
            how_it_works: "Scans binary files using magic byte signatures to identify embedded file types (filesystems, archives, executables, images) and can extract identified components using system tools like unsquashfs and tar.".into(),
            use_cases: vec![
                "Firmware reverse engineering".into(),
                "Embedded file extraction".into(),
                "Firmware analysis".into(),
                "IoT security research".into(),
                "Filesystem structure analysis".into(),
            ],
            attack_types: vec![
                "Reverse Engineering".into(),
                "Analysis".into(),
            ],
            difficulty: "Beginner to Intermediate".into(),
            protocols: vec![
                "SquashFS".into(), "CramFS".into(), "JFFS2".into(),
                "YAFFS".into(), "UBIFS".into(), "ext2/3/4".into(),
                "tar".into(), "gzip".into(), "LZMA".into(),
            ],
            key_features: vec![
                "Magic byte signature database".into(),
                "Automatic file extraction".into(),
                "Recursive scanning".into(),
                "Entropy analysis for encrypted sections".into(),
                "Custom signature support".into(),
                "Plugin architecture".into(),
            ],
            typical_workflow: vec![
                "1. Scan firmware: binwalk firmware.bin".into(),
                "2. Extract files: binwalk -e firmware.bin".into(),
                "3. Recursive extract: binwalk -Me firmware.bin".into(),
                "4. Entropy analysis: binwalk -E firmware.bin".into(),
            ],
            targets: vec![
                "Firmware images".into(),
                "Binary files".into(),
                "Embedded systems".into(),
            ],
            strengths: vec![
                "Excellent signature database".into(),
                "Simple extraction workflow".into(),
                "Good for IoT research".into(),
            ],
            limitations: vec![
                "Extraction depends on system tools being installed".into(),
                "Encrypted sections cannot be analyzed".into(),
                "Some formats require specific extraction tools".into(),
            ],
            alternatives: vec![
                "firmware-mod-kit".into(),
                "Flashrom".into(), "sasquatch".into(),
            ],
            best_practices: vec![
                "Use -e for simple extraction".into(),
                "Use -M for recursive extraction".into(),
                "Use -E for entropy analysis to find encrypted sections".into(),
            ],
        },
        "radare2" | "r2" => ToolKnowledge {
            what_it_does: "Radare2 is a complete framework for reverse engineering, binary analysis, and exploitation development. It includes a disassembler, debugger, hex editor, and analysis tool in a single CLI interface.".into(),
            how_it_works: "Loads binary files and provides a command-line interface to analyze, disassemble, debug, and patch them. Supports dozens of architectures and file formats with rich analysis commands and visual modes.".into(),
            use_cases: vec![
                "Binary reverse engineering".into(),
                "Vulnerability research".into(),
                "Malware analysis".into(),
                "Exploit development".into(),
                "Binary patching".into(),
            ],
            attack_types: vec![
                "Reverse Engineering".into(),
                "Vulnerability Research".into(),
                "Exploit Development".into(),
            ],
            difficulty: "Advanced".into(),
            protocols: vec![
                "ELF".into(), "PE".into(), "Mach-O".into(), "DEX".into(),
                "Java class".into(), "x86".into(), "ARM".into(),
                "MIPS".into(), "RISC-V".into(),
            ],
            key_features: vec![
                "50+ architecture support".into(),
                "Visual mode with graph view".into(),
                "Integrated debugger".into(),
                "Scripting (Python, Lua, Ruby, Go)".into(),
                "Binary patching".into(),
                "YARA rule scanning".into(),
                "Web UI server".into(),
            ],
            typical_workflow: vec![
                "1. Analyze binary: r2 -A binary".into(),
                "2. List functions: afl".into(),
                "3. Disassemble: pdf @main".into(),
                "4. Debug: r2 -d binary".into(),
            ],
            targets: vec![
                "ELF binaries".into(), "PE executables".into(),
                "Mach-O files".into(), "Android APK/DEX".into(),
            ],
            strengths: vec![
                "Extremely comprehensive".into(),
                "Very active development".into(),
                "Powerful scripting".into(),
                "Good architecture support".into(),
            ],
            limitations: vec![
                "Steep learning curve".into(),
                "Complex command syntax".into(),
                "Documentation can be sparse".into(),
            ],
            alternatives: vec![
                "Ghidra".into(), "IDA Pro".into(),
                "Binary Ninja".into(), "objdump".into(),
            ],
            best_practices: vec![
                "Use -A for auto-analysis".into(),
                "Use visual mode (VV) for graph view".into(),
                "Learn the command set gradually".into(),
                "Use r2pipe for scripting".into(),
            ],
        },
        "ghidra" => ToolKnowledge {
            what_it_does: "Ghidra is the NSA's open-source software reverse engineering framework. It provides a powerful decompiler, disassembler, and collaborative analysis environment for compiled code across multiple architectures.".into(),
            how_it_works: "Imports binaries, performs auto-analysis to identify functions, data, and cross-references, then provides a decompiler that converts machine code back to readable C-like pseudocode. Supports collaborative projects.".into(),
            use_cases: vec![
                "Reverse engineering closed-source software".into(),
                "Vulnerability research".into(),
                "Malware analysis".into(),
                "Binary compatibility analysis".into(),
                "Legacy code recovery".into(),
            ],
            attack_types: vec![
                "Reverse Engineering".into(),
                "Vulnerability Research".into(),
                "Malware Analysis".into(),
            ],
            difficulty: "Intermediate to Advanced".into(),
            protocols: vec![
                "x86".into(), "x86_64".into(), "ARM".into(),
                "MIPS".into(), "PowerPC".into(), "RISC-V".into(),
                "Java".into(), "60+ processors".into(),
            ],
            key_features: vec![
                "High-quality decompiler (C pseudocode output)".into(),
                "60+ processor/architecture support".into(),
                "Collaborative reverse engineering projects".into(),
                "Scripting (Java, Python via GhidraBridge)".into(),
                "Function graph and flow analysis".into(),
                "Data type manager".into(),
                "Built-in debugger".into(),
                "Extension/plugin ecosystem".into(),
            ],
            typical_workflow: vec![
                "1. Launch Ghidra GUI or headless".into(),
                "2. Import binary file".into(),
                "3. Auto-analyze (analyze all)".into(),
                "4. Browse decompiled functions".into(),
                "5. Add comments, labels, data types".into(),
            ],
            targets: vec![
                "ELF binaries".into(), "PE executables".into(),
                "Mach-O files".into(), "Java class files".into(),
            ],
            strengths: vec![
                "World-class decompiler".into(),
                "Free and open source".into(),
                "Collaborative features".into(),
                "Extensible".into(),
            ],
            limitations: vec![
                "Java-based (slow startup, memory hungry)".into(),
                "UI can lag with large binaries".into(),
                "Learning curve for advanced features".into(),
            ],
            alternatives: vec![
                "IDA Pro".into(), "Radare2".into(),
                "Binary Ninja".into(), "RetDec".into(),
            ],
            best_practices: vec![
                "Use headless mode for batch analysis".into(),
                "Write scripts for repetitive tasks".into(),
                "Use GhidraBridge for Python integration".into(),
            ],
        },
        "sherlock" => ToolKnowledge {
            what_it_does: "Sherlock hunts down social media accounts by username across 300+ social networks. It checks for the presence of a given username on hundreds of websites simultaneously.".into(),
            how_it_works: "Sends HTTP requests to profile and user pages of hundreds of social media platforms and checks HTTP response status codes to determine if a username exists on each platform.".into(),
            use_cases: vec![
                "OSINT username investigation".into(),
                "Social media profile discovery".into(),
                "Identity verification".into(),
                "Threat intelligence gathering".into(),
            ],
            attack_types: vec![
                "OSINT".into(),
                "Reconnaissance".into(),
            ],
            difficulty: "Beginner".into(),
            protocols: vec!["HTTP".into(), "HTTPS".into()],
            key_features: vec![
                "300+ social media platform support".into(),
                "Fast parallel checking".into(),
                "Multiple output formats".into(),
                "JSON output for tool integration".into(),
                "Proxy support".into(),
            ],
            typical_workflow: vec![
                "1. Search username: sherlock <username>".into(),
                "2. Save results: sherlock <username> --output results.txt".into(),
                "3. JSON output: sherlock <username> --json results.json".into(),
            ],
            targets: vec![
                "Social media platforms".into(),
                "Web services".into(),
                "Forums".into(),
            ],
            strengths: vec![
                "Large platform coverage".into(),
                "Simple to use".into(),
                "Fast results".into(),
                "Active maintenance".into(),
            ],
            limitations: vec![
                "False positives from username patterns".into(),
                "Some sites require authentication".into(),
                "Rate limiting on some platforms".into(),
            ],
            alternatives: vec![
                "whatsmyname".into(), "Namechk".into(), "Maigret".into(),
            ],
            best_practices: vec![
                "Use --json for structured output".into(),
                "Combine with other OSINT tools".into(),
                "Verify results manually for important cases".into(),
            ],
        },
        "enum4linux" => ToolKnowledge {
            what_it_does: "Enum4linux is a tool for enumerating information from Windows and Samba systems using SMB. It discovers users, shares, policies, groups, passwords, and other information from SMB targets.".into(),
            how_it_works: "Queries SMB services on the target using various SMB operations to enumerate users, groups, shares, policies, and other system information. Wraps smbclient, net, and rpcclient.".into(),
            use_cases: vec![
                "Windows and Samba enumeration".into(),
                "SMB share discovery".into(),
                "User enumeration".into(),
                "Password policy discovery".into(),
                "Security assessment of Windows networks".into(),
            ],
            attack_types: vec![
                "Enumeration".into(),
                "Information Gathering".into(),
            ],
            difficulty: "Beginner".into(),
            protocols: vec![
                "SMB".into(), "NetBIOS".into(), "LDAP".into(), "RPC".into(),
            ],
            key_features: vec![
                "User and group enumeration".into(),
                "Share enumeration and listing".into(),
                "Password policy discovery".into(),
                "RID cycling".into(),
                "SMB signing detection".into(),
                "OS information gathering".into(),
            ],
            typical_workflow: vec![
                "1. Basic enumeration: enum4linux <target>".into(),
                "2. With credentials: enum4linux -u user -p pass <target>".into(),
                "3. Specific operations: enum4linux -S <target>".into(),
            ],
            targets: vec![
                "Windows servers".into(),
                "Samba shares".into(),
                "Domain controllers".into(),
            ],
            strengths: vec![
                "Comprehensive SMB enumeration".into(),
                "Simple CLI".into(),
                "Good for initial Windows assessment".into(),
            ],
            limitations: vec![
                "Dated and not actively maintained".into(),
                "Noisy and easily detected".into(),
                "Limited to SMBv1 by default".into(),
            ],
            alternatives: vec![
                "CrackMapExec".into(), "smbclient".into(),
                "enum4linux-ng".into(), "NetExec".into(),
            ],
            best_practices: vec![
                "Try anonymous or null sessions first".into(),
                "Use enum4linux-ng for modern replacement".into(),
                "Combine with CrackMapExec for comprehensive results".into(),
            ],
        },
        "whatweb" => ToolKnowledge {
            what_it_does: "WhatWeb identifies technologies used by websites including web servers, CMS, frameworks, JavaScript libraries, analytics, and more using over 1800 plugins.".into(),
            how_it_works: "Sends HTTP requests and analyzes response headers, HTML content, cookies, and JavaScript to identify technologies using a database of over 1800 plugins with regex matching and signature detection.".into(),
            use_cases: vec![
                "Technology fingerprinting".into(),
                "Web application identification".into(),
                "CMS detection".into(),
                "Security assessment reconnaissance".into(),
            ],
            attack_types: vec![
                "Reconnaissance".into(),
                "Fingerprinting".into(),
            ],
            difficulty: "Beginner".into(),
            protocols: vec!["HTTP".into(), "HTTPS".into()],
            key_features: vec![
                "1800+ plugins for technology detection".into(),
                "Verbose and aggressive scanning modes".into(),
                "Multiple output formats".into(),
                "Custom plugin development".into(),
                "Proxy support".into(),
                "Throttling and stealth options".into(),
            ],
            typical_workflow: vec![
                "1. Basic scan: whatweb http://target.com".into(),
                "2. Verbose: whatweb -v http://target.com".into(),
                "3. Aggressive: whatweb -a 3 http://target.com".into(),
                "4. Batch: whatweb -i urls.txt".into(),
            ],
            targets: vec![
                "Websites".into(),
                "Web applications".into(),
                "Web servers".into(),
            ],
            strengths: vec![
                "Massive plugin database".into(),
                "Fast fingerprinting".into(),
                "Active development".into(),
            ],
            limitations: vec![
                "May miss obfuscated technologies".into(),
                "Aggressive mode is noisy".into(),
                "Some plugins have false positives".into(),
            ],
            alternatives: vec![
                "wappalyzer".into(), "BuiltWith".into(),
                "nmap http-generator".into(),
            ],
            best_practices: vec![
                "Use -v for detailed output".into(),
                "Use -a for aggressive mode when stealth is not needed".into(),
                "Combine with other tools for comprehensive recon".into(),
            ],
        },
        "wifite" => ToolKnowledge {
            what_it_does: "Wifite is an automated wireless attack tool that automates WEP, WPA/WPA2, and WPS cracking. It integrates aircrack-ng, reaver, and other tools into a single automated workflow.".into(),
            how_it_works: "Puts the wireless interface into monitor mode, discovers nearby networks, captures handshakes or PMKID, and automatically launches the appropriate cracking tool based on the target's security settings.".into(),
            use_cases: vec![
                "Automated WiFi auditing".into(),
                "WPA handshake cracking".into(),
                "WEP key recovery".into(),
                "WPS PIN brute-force".into(),
                "Wireless penetration testing".into(),
            ],
            attack_types: vec![
                "Wireless".into(),
                "Credential Access".into(),
            ],
            difficulty: "Beginner".into(),
            protocols: vec![
                "WEP".into(), "WPA".into(), "WPA2".into(),
                "WPS".into(), "802.11".into(),
            ],
            key_features: vec![
                "Fully automated wireless attacks".into(),
                "WEP, WPA/WPA2, WPS support".into(),
                "PMKID capture support".into(),
                "Integrated aircrack-ng suite".into(),
                "Reaver and wpscrack integration".into(),
                "Kill conflicting processes".into(),
            ],
            typical_workflow: vec![
                "1. Scan networks: wifite --scan".into(),
                "2. Attack all: wifite".into(),
                "3. Target specific: wifite --bssid XX:XX:XX:XX:XX:XX".into(),
            ],
            targets: vec![
                "WiFi networks".into(),
                "Wireless access points".into(),
            ],
            strengths: vec![
                "Fully automated workflow".into(),
                "Beginner friendly".into(),
                "Multiple attack vectors".into(),
            ],
            limitations: vec![
                "Requires compatible wireless adapter".into(),
                "Only works on 2.4GHz by default".into(),
                "Success depends on handshake capture".into(),
            ],
            alternatives: vec![
                "aircrack-ng (manual)".into(),
                "fern wifi cracker".into(), "linset".into(),
            ],
            best_practices: vec![
                "Use a high-gain antenna for better range".into(),
                "Ensure monitor mode is supported by adapter".into(),
                "Use -w for wordlist".into(),
                "Kill network manager first".into(),
            ],
        },
        "burpsuite" | "burp" => ToolKnowledge {
            what_it_does: "Burp Suite is the leading web application security testing platform. It provides an intercepting proxy, automated scanner, intruder for fuzzing, repeater for request manipulation, and vulnerability scanning.".into(),
            how_it_works: "Acts as a man-in-the-middle proxy between browser and web application, allowing interception and modification of all traffic. The scanner automates vulnerability detection while manual tools allow deep testing.".into(),
            use_cases: vec![
                "Web application security testing".into(),
                "API security testing".into(),
                "Manual vulnerability research".into(),
                "Automated vulnerability scanning".into(),
                "Session handling and authentication testing".into(),
            ],
            attack_types: vec![
                "Injection".into(),
                "XSS".into(),
                "CSRF".into(),
                "Authentication Bypass".into(),
                "Business Logic Flaws".into(),
            ],
            difficulty: "Intermediate to Advanced".into(),
            protocols: vec![
                "HTTP".into(), "HTTPS".into(), "WebSocket".into(),
                "HTTP/2".into(), "gRPC".into(),
            ],
            key_features: vec![
                "Intercepting proxy with full request/response control".into(),
                "Automated vulnerability scanner".into(),
                "Intruder (customizable attack tool)".into(),
                "Repeater (manual request modification)".into(),
                "Decoder (encoding/decoding)".into(),
                "Comparer (diff analysis)".into(),
                "Extender (plugin system)".into(),
                "Collaborator (out-of-band testing)".into(),
            ],
            typical_workflow: vec![
                "1. Configure browser proxy to 127.0.0.1:8080".into(),
                "2. Browse target through Burp Proxy".into(),
                "3. Send interesting requests to Repeater".into(),
                "4. Test with Intruder for parameter fuzzing".into(),
                "5. Run automated scan on target scope".into(),
            ],
            targets: vec![
                "Web applications".into(),
                "REST/SOAP APIs".into(),
                "Mobile app backends".into(),
            ],
            strengths: vec![
                "Industry standard for web app testing".into(),
                "Powerful manual testing tools".into(),
                "Extensible via plugins".into(),
                "Professional reporting".into(),
            ],
            limitations: vec![
                "Community edition is limited".into(),
                "Professional version is expensive".into(),
                "Java-based and memory hungry".into(),
            ],
            alternatives: vec![
                "OWASP ZAP".into(), "mitmproxy".into(), "OWASP AXE".into(),
            ],
            best_practices: vec![
                "Use with FoxyProxy for easy switching".into(),
                "Define target scope to focus scanning".into(),
                "Use Collaborator for blind vulnerability detection".into(),
                "Install extensions from BApp Store".into(),
            ],
        },
        "aircrack-ng" => ToolKnowledge {
            what_it_does: "Aircrack-ng is a complete suite of tools for WiFi network assessment including monitoring, attacking, testing, and cracking. It covers the entire WiFi auditing workflow from capture to crack.".into(),
            how_it_works: "Includes tools to put interfaces in monitor mode (airmon-ng), capture traffic (airodump-ng), inject packets (aireplay-ng), and crack WEP/WPA handshakes (aircrack-ng) using statistical analysis or dictionary attacks.".into(),
            use_cases: vec![
                "WiFi network security auditing".into(),
                "WEP key recovery".into(),
                "WPA/WPA2 handshake cracking".into(),
                "Deauthentication attacks".into(),
                "Rogue access point detection".into(),
            ],
            attack_types: vec![
                "Wireless".into(),
                "Credential Access".into(),
                "Denial of Service".into(),
            ],
            difficulty: "Intermediate".into(),
            protocols: vec![
                "WEP".into(), "WPA".into(), "WPA2".into(),
                "802.11a/b/g/n/ac".into(),
            ],
            key_features: vec![
                "Complete WiFi auditing suite".into(),
                "WEP and WPA/WPA2 cracking".into(),
                "Packet injection".into(),
                "Deauthentication attacks".into(),
                "PMKID attack support".into(),
                "Multiple capture format support".into(),
            ],
            typical_workflow: vec![
                "1. Enable monitor mode: airmon-ng start wlan0".into(),
                "2. Scan networks: airodump-ng wlan0mon".into(),
                "3. Capture: airodump-ng -c CH --bssid MAC -w capture wlan0mon".into(),
                "4. Deauth client: aireplay-ng -0 5 -a AP -c CLIENT wlan0mon".into(),
                "5. Crack: aircrack-ng -w wordlist.txt capture-01.cap".into(),
            ],
            targets: vec![
                "WiFi networks".into(),
                "Wireless clients".into(),
            ],
            strengths: vec![
                "Most complete WiFi auditing suite".into(),
                "Mature and well-tested".into(),
                "Large community".into(),
            ],
            limitations: vec![
                "Requires compatible wireless adapter".into(),
                "Manual process is complex".into(),
                "WPA3 not supported".into(),
            ],
            alternatives: vec![
                "wifite (automated)".into(),
                "hashcat".into(), "Fern WiFi Cracker".into(),
            ],
            best_practices: vec![
                "Ensure adapter supports monitor mode and injection".into(),
                "Use a compatible USB wireless adapter".into(),
                "Capture a clean handshake before cracking".into(),
            ],
        },
        _ => generate_category_based_knowledge(tool),
    }
}

fn generate_category_based_knowledge(tool: &Tool) -> ToolKnowledge {
    let primary_category = tool.categories.first().copied();

    let (cat_desc, cat_use_cases, cat_attack_types, cat_protocols, cat_difficulty): (String, Vec<String>, Vec<String>, Vec<String>, String) = match primary_category {
        Some(Category::Reconnaissance) => (
            "This is a reconnaissance and network discovery tool. It is used in the information gathering phase of security assessments to identify live hosts, open ports, services, and network topology.".into(),
            vec!["Network discovery and mapping".into(), "Service enumeration".into(), "Host identification".into(), "Attack surface discovery".into()],
            vec!["Reconnaissance".into(), "Enumeration".into()],
            vec!["TCP".into(), "UDP".into(), "ICMP".into(), "DNS".into(), "HTTP".into()],
            "Beginner to Intermediate".into(),
        ),
        Some(Category::WebSecurity) => (
            "This is a web application security tool. It tests websites and web APIs for vulnerabilities including injection flaws, misconfigurations, authentication weaknesses, and other security issues.".into(),
            vec!["Web vulnerability scanning".into(), "Directory discovery".into(), "Parameter testing".into(), "API security testing".into()],
            vec!["Injection".into(), "XSS".into(), "CSRF".into(), "Misconfiguration".into()],
            vec!["HTTP".into(), "HTTPS".into(), "WebSocket".into()],
            "Beginner to Intermediate".into(),
        ),
        Some(Category::NetworkSecurity) => (
            "This is a network security tool. It analyzes, protects, or tests network infrastructure for security issues including misconfigurations, weak protocols, and unauthorized access.".into(),
            vec!["Network traffic analysis".into(), "Intrusion detection".into(), "Firewall testing".into(), "Protocol analysis".into()],
            vec!["Network Analysis".into(), "Traffic Inspection".into()],
            vec!["TCP".into(), "UDP".into(), "ICMP".into(), "ARP".into(), "DNS".into()],
            "Intermediate".into(),
        ),
        Some(Category::ExploitDevelopment) => (
            "This is an exploit development tool. It helps security researchers develop, test, and deliver exploits for known vulnerabilities, including payload generation and shellcode crafting.".into(),
            vec!["Exploit development".into(), "Payload creation".into(), "Shellcode testing".into(), "Vulnerability verification".into()],
            vec!["Exploitation".into(), "Post-Exploitation".into()],
            vec!["TCP".into(), "UDP".into(), "HTTP".into(), "Various protocols".into()],
            "Advanced".into(),
        ),
        Some(Category::ReverseEngineering) => (
            "This is a reverse engineering tool. It analyzes compiled binaries to understand their functionality, identify vulnerabilities, or recover lost source code through disassembly and decompilation.".into(),
            vec!["Binary analysis".into(), "Code decompilation".into(), "Malware analysis".into(), "Protocol reverse engineering".into()],
            vec!["Reverse Engineering".into(), "Analysis".into()],
            vec!["ELF".into(), "PE".into(), "Mach-O".into(), "Various binary formats".into()],
            "Intermediate to Advanced".into(),
        ),
        Some(Category::PasswordSecurity) => (
            "This is a password security tool. It tests password strength through cracking, auditing, or analysis to identify weak credentials that could be exploited.".into(),
            vec!["Password strength testing".into(), "Hash cracking".into(), "Credential auditing".into(), "Password policy verification".into()],
            vec!["Credential Access".into(), "Password Cracking".into()],
            vec!["MD5".into(), "SHA".into(), "bcrypt".into(), "NTLM".into(), "Various hash formats".into()],
            "Beginner to Intermediate".into(),
        ),
        Some(Category::DigitalForensics) => (
            "This is a digital forensics tool. It collects, preserves, and analyzes digital evidence from computers, networks, and storage media for incident response or legal proceedings.".into(),
            vec!["Evidence collection".into(), "Disk imaging".into(), "Memory analysis".into(), "File carving".into()],
            vec!["Forensics".into(), "Evidence Collection".into()],
            vec!["File systems".into(), "Memory".into(), "Network captures".into()],
            "Intermediate to Advanced".into(),
        ),
        Some(Category::Fuzzing) => (
            "This is a fuzzing tool. It discovers software vulnerabilities by automatically generating and sending malformed or unexpected inputs to target applications.".into(),
            vec!["Bug discovery".into(), "Crash detection".into(), "Input validation testing".into(), "Protocol fuzzing".into()],
            vec!["Fuzzing".into(), "Bug Discovery".into()],
            vec!["TCP".into(), "UDP".into(), "HTTP".into(), "File formats".into(), "Application protocols".into()],
            "Intermediate to Advanced".into(),
        ),
        Some(Category::Cryptography) => (
            "This is a cryptography tool. It provides encryption, decryption, cryptographic analysis, or key management capabilities for securing communications or testing cryptographic implementations.".into(),
            vec!["Encryption and decryption".into(), "Key generation".into(), "Certificate analysis".into(), "Hash computation".into()],
            vec!["Cryptography".into(), "Data Protection".into()],
            vec!["Various cryptographic protocols".into()],
            "Intermediate".into(),
        ),
        Some(Category::Wireless) => (
            "This is a wireless security tool. It analyzes or tests the security of WiFi, Bluetooth, and other wireless protocols through scanning, cracking, or analysis.".into(),
            vec!["WiFi network auditing".into(), "Bluetooth scanning".into(), "Wireless protocol analysis".into(), "Signal analysis".into()],
            vec!["Wireless".into(), "Network Analysis".into()],
            vec!["802.11".into(), "Bluetooth".into(), "Zigbee".into(), "SDR".into()],
            "Beginner to Intermediate".into(),
        ),
        Some(Category::MalwareAnalysis) => (
            "This is a malware analysis tool. It examines malicious software to understand its behavior, identify indicators of compromise, and develop countermeasures.".into(),
            vec!["Malware detection".into(), "Behavioral analysis".into(), "Indicator extraction".into(), "Sandbox analysis".into()],
            vec!["Malware Analysis".into(), "Threat Intelligence".into()],
            vec!["Various file formats".into(), "Network protocols".into()],
            "Intermediate to Advanced".into(),
        ),
        Some(Category::MobileSecurity) => (
            "This is a mobile security tool. It tests the security of Android and iOS applications and mobile device infrastructure.".into(),
            vec!["Mobile app testing".into(), "APK analysis".into(), "API testing".into(), "Device security assessment".into()],
            vec!["Mobile Security".into(), "Application Testing".into()],
            vec!["HTTP".into(), "HTTPS".into(), "ADB".into(), "Mobile APIs".into()],
            "Intermediate".into(),
        ),
        Some(Category::CloudSecurity) => (
            "This is a cloud security tool. It audits and tests the security configurations of cloud platforms like AWS, Azure, and GCP for misconfigurations and vulnerabilities.".into(),
            vec!["Cloud misconfiguration detection".into(), "IAM policy analysis".into(), "Container security".into(), "Cloud infrastructure auditing".into()],
            vec!["Cloud Security".into(), "Misconfiguration Detection".into()],
            vec!["AWS API".into(), "Azure API".into(), "GCP API".into(), "Kubernetes API".into()],
            "Intermediate".into(),
        ),
        Some(Category::Steganography) => (
            "This is a steganography tool. It hides, detects, or extracts hidden data within image, audio, or video files.".into(),
            vec!["Data concealment".into(), "Steganographic detection".into(), "Hidden data extraction".into(), "Digital watermarking".into()],
            vec!["Steganography".into(), "Data Concealment".into()],
            vec!["PNG".into(), "JPEG".into(), "WAV".into(), "MP3".into(), "Video formats".into()],
            "Beginner to Intermediate".into(),
        ),
        Some(Category::Osint) => (
            "This is an OSINT (Open Source Intelligence) tool. It gathers intelligence from publicly available sources including social media, search engines, and public databases.".into(),
            vec!["Social media investigation".into(), "Domain research".into(), "People search".into(), "Data aggregation".into()],
            vec!["OSINT".into(), "Reconnaissance".into()],
            vec!["HTTP".into(), "HTTPS".into(), "DNS".into(), "Various APIs".into()],
            "Beginner".into(),
        ),
        Some(Category::BluetoothBle) => (
            "This is a Bluetooth and BLE security tool. It discovers, analyzes, or tests the security of Bluetooth and Bluetooth Low Energy devices and protocols.".into(),
            vec!["Bluetooth device discovery".into(), "BLE protocol analysis".into(), "Bluetooth vulnerability testing".into(), "GATT service enumeration".into()],
            vec!["Wireless".into(), "Bluetooth Security".into()],
            vec!["Bluetooth Classic".into(), "BLE".into(), "GATT".into(), "SMP".into()],
            "Intermediate".into(),
        ),
        _ => (
            "This is a cybersecurity tool. Consult the tool's documentation or run with --help for detailed usage information.".into(),
            vec!["Security testing".into(), "Vulnerability assessment".into(), "Penetration testing".into()],
            vec!["Security Assessment".into()],
            vec!["Various protocols".into()],
            "Intermediate".into(),
        ),
    };

    let generic_features: Vec<String> = if !tool.executables.is_empty() {
        tool.executables
            .iter()
            .map(|e| format!("Executable: {}", e.split('/').next_back().unwrap_or(e)))
            .collect()
    } else {
        vec!["Command-line tool".into()]
    };

    ToolKnowledge {
        what_it_does: format!(
            "{} It is classified under: {}",
            cat_desc,
            tool.categories
                .iter()
                .map(|c| c.name())
                .collect::<Vec<_>>()
                .join(", ")
        ),
        how_it_works: format!(
            "Package '{}' from the {} repository. {}",
            tool.name,
            tool.repository.name(),
            if tool.short_description != tool.detailed_description
                && !tool.detailed_description.is_empty()
            {
                tool.detailed_description.as_str()
            } else {
                "Consult the tool's --help output or homepage for operational details."
            }
        ),
        use_cases: cat_use_cases,
        attack_types: cat_attack_types,
        difficulty: cat_difficulty,
        protocols: cat_protocols,
        key_features: generic_features,
        typical_workflow: vec![
            format!("1. Run {} --help to see available options", tool.name),
            "2. Consult the homepage for documentation".into(),
            "3. Check the tool's man page for detailed usage".into(),
        ],
        targets: vec!["Various targets depending on configuration".into()],
        strengths: vec![format!(
            "Integrated with {} package management",
            if tool.repository == Repository::BlackArch {
                "BlackArch"
            } else {
                "Arch"
            }
        )],
        limitations: vec!["Run --help for specific usage details".into()],
        alternatives: vec![],
        best_practices: vec![
            "Read the documentation before use".into(),
            "Test in a controlled environment first".into(),
            "Use appropriate privileges only when needed".into(),
        ],
    }
}
