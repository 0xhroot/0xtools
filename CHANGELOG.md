# Changelog

All notable changes to 0xtools will be documented in this file.

## [0.1.0] - 2026-07-26

First public release.

### Added
- Interactive TUI with keyboard-first navigation (ratatui + crossterm)
- Full CLI with search, info, categories, list, installed, favorites, profiles
- ALPM/libalpm integration for Arch Linux package metadata
- BlackArch repository detection and indexing
- 35 security tool categories with automatic classification
- Fuzzy search with weighted scoring across names, descriptions, categories, tags
- Structured search queries (`category:web installed:true repo:blackarch`)
- Rich tool detail view with descriptions, usage guidance, executables, dependencies
- Package installation and removal with transaction previews and pacman output
- Favorites system with persistent storage
- 8 bundled profiles (Web Security, Bug Bounty, OSINT, etc.)
- Doctor command for system health checks
- Binary cache with atomic writes and corruption recovery
- Root launcher script for building from source
- Prebuilt Linux x86_64 binary via GitHub Releases
- Arch Linux PKGBUILD template
- MIT/Apache-2.0 dual license
