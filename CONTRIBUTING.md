# Contributing to 0xtools

Thank you for considering contributing to 0xtools!

## Development Setup

1. Install Rust: https://www.rust-lang.org/tools/install
2. Clone the repository
3. Build: `cargo build`
4. Run tests: `cargo test`
5. Run: `cargo run`

## Code Quality

Before submitting a PR, ensure:

```sh
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
cargo build --release
```

## Adding Tool References

Create a TOML file in `references/`:

```toml
tool = "nmap"
overview = "Network exploration tool and security / port scanner"
purpose = "Discover hosts, open ports, and services on a network"
common_options = [
    { flag = "-sV", description = "Version detection" },
    { flag = "-sS", description = "SYN scan" },
]
examples = [
    "nmap -sV 192.168.1.0/24",
    "nmap -sS -p 1-1000 target",
]
official_docs = "https://nmap.org/book/man.html"
```

## Reporting Issues

Use the GitHub issue tracker. Include:
- Steps to reproduce
- Expected behavior
- Actual behavior
- System information (OS, Rust version, terminal)

## Security

See [SECURITY.md](SECURITY.md) for vulnerability reporting.
