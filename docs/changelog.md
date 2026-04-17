# Changelog

All notable changes to this project will be documented in this file.

## [2.0.0] - 2025-04-17

### Changed
- 🦀 **Complete Rust Rewrite** - Entire core rebuilt using Rust
- ⚡ **6-7x Faster** - Scanning speed improved from ~2s to ~0.3s (100 projects)
- 💾 **10x Less Memory** - Reduced from ~50MB to ~5MB
- 🚀 **10x Faster Startup** - Launch time from ~500ms to ~50ms

### Added
- Rust core library with Node.js wrapper
- Native platform-specific deletion commands
- Enhanced error handling with fallback methods
- Pre-built binary support for Windows, macOS, Linux
- Cargo build system integration

### Updated
- Clap v4.4 for CLI argument parsing
- Enhanced i18n system (zh-CN, en-US)
- Improved logging with multiple levels
- Enhanced interactive mode with dialoguer
- Progress bar using indicatif

## [1.0.0] - 2025-04-09

### Added
- Recursive scanning for `node_modules` directories
- Safe mode with confirmation prompt
- Interactive mode with guided wizard
- Progress bar during deletion
- Multi-language support (zh-CN, en-US)
- Directory size calculation
- Ignore directory patterns
- Advanced logging system (debug, info, warn, error)
- Log file output
- Silent mode
- Cross-platform support (Windows, macOS, Linux)
- CI/CD pipeline with multi-platform testing
- Git pre-commit hooks
- GitHub Actions automatic release workflow
