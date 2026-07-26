<div align="center">

# `0xtools`

### Security tools, mapped.

**A fast, keyboard-first cybersecurity tool explorer for Arch Linux & BlackArch.**

Discover · Understand · Install · Organize

[![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Arch Linux](https://img.shields.io/badge/Arch%20Linux-1793D1?style=flat&logo=archlinux&logoColor=white)](https://archlinux.org/)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](LICENSE-MIT)

</div>

<br>

```
    ╔═══════════════════════════════════╗
    ║     ┌─┐ 0xtools                  ║
    ║     │▒│ security tools browser    ║
    ║     └─┘─────────────────── ▒ ▒ ▒  ║
    ╚═══════════════════════════════════╝
```

<br>

## What is 0xtools?

Arch Linux and BlackArch together ship thousands of security tools — from `nmap` to `ghidra`, from `sqlmap` to `volatility3`. Finding the right tool for a job, understanding what it does, and managing it from package metadata is cumbersome.

**0xtools** is a native Rust TUI and CLI that dynamically indexes your local Arch and BlackArch package databases into a searchable, categorized catalog. Every tool gets a description, classification, and package status — no more memorizing package names or browsing wikis.

It works entirely offline. No telemetry, no daemons, no background processes. Just your package databases, indexed fast.

<br>

## Preview

```
╭────────────────────────────────────────────────────────────────╮
│  0xtools                                          Dashboard   │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│  ● Reconnaissance                  1089 tools                 │
│    OSINT                            106 tools                 │
│    Web Security                     477 tools                 │
│    Network Security                 274 tools                 │
│    Wireless                         103 tools                 │
│    Reverse Engineering              342 tools                 │
│    Exploit Development              395 tools                 │
│    Malware Analysis                 115 tools                 │
│    Cloud Security                   150 tools                 │
│    Cryptography                     119 tools                 │
│    ...                                                        │
│                                                                │
│  35 categories │ dynamically indexed │ offline capable        │
╰────────────────────────────────────────────────────────────────╯
```

Each tool has a description — never a bare package name:

```
  ● ffuf             Fast web fuzzer written in Go.              [BlackArch]
  ● nuclei           Fast and customizable vulnerability scan…   [BlackArch]
  ● sqlmap           Automatic SQL injection and database tak…   [BlackArch]
  ● masscan          TCP port scanner, transmits 10M packets/…   [Extra]
```

<br>

## Features

| | Feature | Description |
|---|---|---|
| 🔎 | **Fuzzy Search** | Find tools by name, purpose, or description |
| 🗂️ | **35 Categories** | Recon, OSINT, Web, RE, Forensics, Cloud, and more |
| 🧠 | **Rich Descriptions** | Understand what each tool does before installing |
| 📦 | **Package Management** | Install and remove via pacman with transaction previews |
| ⭐ | **Favorites** | Build a personal toolkit for quick access |
| 📋 | **Profiles** | Curated collections: Bug Bounty, OSINT, Forensics, etc. |
| ⌨️ | **Keyboard First** | Vim-style navigation in the TUI |
| 🐉 | **BlackArch** | First-class integration — runs on normal Arch too |
| ⚡ | **Native Rust** | Single binary, no Electron, no webview, no daemon |
| 🔍 | **Structured Search** | Filter by `category:`, `repo:`, `installed:`, `tag:` |

<br>

## 🧠 Don't know the tool name? Search the purpose.

```
$ 0xtools search "subdomain discovery"

  subfinder        Modular subdomain discovery tool
  ct-exposer       Discovers sub-domains via Certificate Transparency
  sublert          Leverages certificate transparency logs
  amass            In-depth attack surface mapping
```

```
$ 0xtools search "wireless deauth"

  aircrack-ng      WiFi security assessment tools
  wifite           Automated wireless attack tool
  bettercap        Network attack and monitoring framework
```

0xtools searches tool descriptions and metadata, not just package names. Describe *what you need* and find it.

<br>

## Installation

```sh
git clone https://github.com/0xhroot/0xtools.git
cd 0xtools
./0xtools              # builds release binary on first run, then launches
```

The `./0xtools` launcher script checks for a prebuilt binary. If none exists and Cargo is available, it builds a release binary automatically. Subsequent launches use the cached binary.

### Requirements

| Requirement | Notes |
|---|---|
| **Arch Linux** | x86_64, pacman-based |
| **Rust / Cargo** | For building from source |
| **BlackArch repo** | Optional — enhances catalog |

<br>

## 🐉 BlackArch

You do **not** need the full BlackArch distribution.

0xtools automatically detects and indexes any configured BlackArch repository. If BlackArch is not configured, it continues operating with whatever repositories you have.

```
  Arch Linux          ← base system
  + BlackArch repo    ← optional, adds 14k+ security tools
  + 0xtools           ← browse, search, install
```

If BlackArch is missing, 0xtools gracefully degrades — you still get full access to security-classified tools in the official Arch repos.

<br>

## Usage

### TUI

```sh
./0xtools              # launch interactive TUI
```

### CLI

```
  Command              Purpose
  ─────────────────────────────────────────────
  search <query>       Search tools by name or description
  info <name>          Detailed tool information and usage
  categories           List all categories with tool counts
  list <category>      List tools in a specific category
  installed            Show installed security tools
  available            Show all available tools
  favorites            Manage your favorite tools
  profiles             List curated tool profiles
  profile <name>       View or install a profile (--install)
  sync                 Refresh the package database cache
  doctor               Check system health and configuration
  version              Show version information
```

### Examples

```sh
./0xtools search nmap                    # fuzzy search
./0xtools search "subdomain discovery"   # purpose-based search
./0xtools search category:osint email    # structured filters
./0xtools info nmap                      # rich tool detail
./0xtools list web                       # tools in Web Security
./0xtools profile "Bug Bounty" --install # install a profile
./0xtools doctor                         # verify your setup
```

<br>

## Keyboard Controls

### Navigation

| Key | Action |
|---|---|
| <kbd>j</kbd> / <kbd>↓</kbd> | Move down |
| <kbd>k</kbd> / <kbd>↑</kbd> | Move up |
| <kbd>h</kbd> / <kbd>←</kbd> / <kbd>Esc</kbd> | Go back |
| <kbd>l</kbd> / <kbd>→</kbd> / <kbd>Enter</kbd> | Open / Select |
| <kbd>Tab</kbd> | Switch pane |
| <kbd>n</kbd> | Next category |

### Actions

| Key | Action |
|---|---|
| <kbd>/</kbd> or <kbd>s</kbd> | Open search |
| <kbd>f</kbd> | Toggle favorite |
| <kbd>i</kbd> | Install tool |
| <kbd>u</kbd> | Uninstall tool |
| <kbd>r</kbd> | Run executable |
| <kbd>p</kbd> | Profiles |
| <kbd>?</kbd> | Help |
| <kbd>q</kbd> | Quit |

### Search Syntax

| Syntax | Example |
|---|---|
| Free text | `nmap scanner` |
| `category:` | `category:web` |
| `repo:` | `repo:blackarch` |
| `installed:` | `installed:true` |
| `favorite:` | `favorite:true` |
| `tag:` | `tag:scanner` |

<br>

## Categories

35 security categories, dynamically populated from your installed package databases:

<details>
<summary><b>View all categories</b></summary>

| | | |
|---|---|---|
| Reconnaissance | OSINT | Network Security |
| Web Security | API Security | Wireless |
| Password Security | Vulnerability Assessment | Active Directory |
| Exploit Development | Reverse Engineering | Binary Analysis |
| Malware Analysis | Digital Forensics | Incident Response |
| Mobile Security | Cloud Security | Container Security |
| Kubernetes Security | Source Code Security | Cryptography |
| Steganography | Hardware / IoT | Firmware |
| Bluetooth / BLE | RFID / NFC | SDR |
| Threat Intelligence | Defensive Security | IDS / IPS |
| Honeypots | Proxy / Tunneling | Fuzzing |
| Reporting | Security Utilities | |

</details>

<br>

## Profiles

Curated tool collections for common workflows:

| Profile | Description |
|---|---|
| Web Security | ffuf, gobuster, nikto, sqlmap, nuclei, whatweb, wpscan, feroxbuster, httpx, dirsearch |
| Bug Bounty | ffuf, nuclei, httpx, subfinder, amass, gobuster, sqlmap, nikto, whatweb |
| OSINT | theharvester, recon-ng, maltego, sherlock, spiderfoot, amass, subfinder |
| Network Assessment | nmap, masscan, wireshark-cli, netdiscover, hping, tcpdump |
| Reverse Engineering | ghidra, radare2, binwalk, strace, ltrace, gdb |
| Digital Forensics | autopsy, sleuthkit, volatility3, bulk-extractor, binwalk, foremost |
| Wireless | aircrack-ng, kismet, wifite, reaver, hashcat |
| Password Security | hashcat, john, hydra, medusa, ophcrack |

<br>

## How It Works

```
              0xtools
                 │
       ┌─────────┴─────────┐
       │                   │
      TUI                 CLI
       │                   │
       └─────────┬─────────┘
                 │
          Search / Catalog
                 │
           ALPM + Cache
                 │
       ┌─────────┴─────────┐
       │                   │
      Arch              BlackArch
```

- **Package metadata is indexed dynamically** from your local ALPM/libalpm databases — no API calls, no scraping.
- **Descriptions and categories** are automatically generated using classifier heuristics and a built-in knowledge base for popular tools.
- **Binary cache** stores the indexed catalog in XDG directories with atomic writes for fast startup.
- **pacman/ALPM** remains the source of truth. 0xtools reads, never modifies, the package database until you explicitly install or remove.

<br>

## Lightweight by Design

- **Native Rust** — single binary, no runtime dependencies
- **Event-driven TUI** — zero idle CPU via crossterm
- **No Electron / webview** — renders in your terminal
- **No background daemon** — nothing running after exit
- **No telemetry / analytics** — fully offline by default
- **No remote API calls** — package databases are local

<br>

## Security

- **Unprivileged by default** — browsing and search require no root
- **Privilege escalation only on explicit action** — install/remove prompts for sudo
- **No arbitrary script execution** — never runs `curl | sh` or similar
- **Package names validated** against repository metadata before transactions
- **Package metadata treated as untrusted input**
- **Core functionality works entirely offline**

0xtools is a security-tool catalog and package interface — not an automated exploitation framework.

See [SECURITY.md](SECURITY.md) for vulnerability reporting.

<br>

<details>
<summary><b>Development</b></summary>

### Build

```sh
cargo build --release
```

### Test

```sh
cargo test --all
```

### Quality

```sh
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```

### Project Structure

```
src/
├── main.rs              Entry point, CLI + TUI dispatch
├── cli.rs               clap CLI definition
├── app/                 TUI state machine and input handling
├── ui/                  ratatui render functions
├── catalog/             Tool, Category, classifier, knowledge base
├── search/              nucleo fuzzy search index
├── package/             ALPM backend, executables, transactions
├── cache/               Binary cache with atomic writes
├── config/              Settings, favorites, themes
├── profiles/            Bundled tool profiles
└── reference/           TOML-based tool references
```

</details>

<br>

## Roadmap

- [x] Arch / BlackArch package indexing via ALPM
- [x] Interactive TUI with keyboard-first navigation
- [x] CLI for scripting and quick queries
- [x] Fuzzy search across names, descriptions, categories
- [x] Rich tool descriptions and usage guidance
- [x] 35 security categories with automatic classification
- [x] Favorites and curated profiles
- [x] Package install / remove with transaction previews
- [x] System health checks (`doctor`)
- [ ] Precompiled GitHub releases
- [ ] AUR package (`0xtools-bin`)
- [ ] Curated tool references (CLI examples, flags)
- [ ] Custom profiles from TOML files

<br>

## Contributing

Contributions welcome — metadata, categories, profiles, references, bug fixes, and performance improvements.

See [CONTRIBUTING.md](CONTRIBUTING.md) for setup instructions and guidelines.

<br>

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE) at your option.

<br>

## Acknowledgements

- [Arch Linux](https://archlinux.org/) — base system
- [BlackArch](https://blackarch.org/) — security distribution
- [Rust](https://www.rust-lang.org/) — programming language
- [ratatui](https://ratatui.rs/) — TUI framework
- [libalpm](https://gitlab.archlinux.org/pacman/pacman) — package manager backend

<br>

<div align="center">

`0xtools`

*Security tools, mapped.*

</div>
