# Security Policy

## Reporting Vulnerabilities

If you discover a security vulnerability in 0xtools, please report it responsibly:

1. **Do NOT** open a public GitHub issue for security vulnerabilities
2. Email security reports to: security@0xtools.dev (or use GitHub's private vulnerability reporting)
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

## Security Model

### Privilege Boundary

- 0xtools runs as a normal user for all browsing and search operations
- Root privileges are only requested for package installation/removal
- The application warns and prefers to refuse interactive use when running as root

### Package Management

- Package names are validated against repository metadata before transactions
- Transaction previews are shown before any system changes
- Uses `pacman` process arguments (not shell interpolation) for package operations
- Never executes `curl | sh`, `wget | bash`, or arbitrary install scripts

### Network

- Core functionality works entirely offline
- No telemetry, analytics, or background network requests
- Network features (if added later) will be clearly optional

### Data Handling

- Package metadata is treated as untrusted input
- No passwords or credentials are stored
- Cache files use atomic writes to prevent corruption
- Logs never contain secrets

### Trust Assumptions

- Trusts the local pacman/libalpm databases
- Trusts the local filesystem for configuration
- Does NOT trust remote sources for core operation

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x   | Yes       |
| < 0.1   | No        |
