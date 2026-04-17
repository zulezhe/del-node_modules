# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.0.0] - 2025-04-17

### Changed
- 🦀 **Complete Rust Rewrite** - Rebuilt the entire core using Rust for maximum performance
- ⚡ **Performance Improvement** - 6-7x faster scanning speed (from ~2s to ~0.3s for 100 projects)
- 💾 **Memory Efficiency** - 10x reduction in memory usage (from ~50MB to ~5MB)
- 🚀 **Faster Startup** - 10x faster launch time (from ~500ms to ~50ms)

### Added
- 🦀 Rust core library with Node.js wrapper for seamless integration
- 🎯 Native platform-specific deletion commands for better performance
- 🛠️ Enhanced error handling with fallback deletion methods
- 📦 Pre-built binary support for multiple platforms
- 🔧 Cargo build system integration

### Updated
- Updated package version to 2.0.0 in package.json
- Updated Cargo.toml with proper metadata and dependencies
- Enhanced CLI argument parsing using clap v4.4
- Improved internationalization (i18n) system with zh-CN and en-US support
- Upgraded logging system with multiple levels and file output
- Enhanced interactive mode with dialoguer crate
- Improved progress bar using indicatif crate

### Technical Details
- Rust edition 2021 with optimized release profile (LTO, strip symbols)
- Dependencies: clap (4.4), colored (2.1), indicatif (0.17), dialoguer (0.11), chrono (0.4), anyhow (1.0), thiserror (1.0)
- Cross-platform compilation support (Windows, macOS, Linux)
- Binary size optimization (~2MB release binary)
- Node.js wrapper automatically detects and uses pre-built Rust binary

### Deprecated
- Original pure Node.js implementation (replaced by Rust core)

## [1.0.0] - 2024-01-21

### Added
- 🎉 Initial release of dnm (Delete Node Modules)
- 🔍 Recursive scanning for node_modules directories
- 🛡️ Safe mode with interactive selection (default)
- 🌐 Multi-language support (Chinese and English)
- 📊 Progress bar with real-time updates
- 🎨 Colorful terminal output using chalk
- 💬 Interactive mode for easy configuration
- 📝 Advanced logging system with multiple levels
- 📏 Optional directory size calculation
- 🚫 Ignore specific directories functionality
- ⚡ Fast deletion using platform-specific commands
- 🧪 Comprehensive automated test suite
- 🔧 Git pre-commit hooks with Husky
- 🚀 GitHub Actions CI/CD pipeline
- 📖 Detailed bilingual README (Chinese/English)

### Features
- Safe mode allows users to select which directories to delete:
  - Delete all (leave empty)
  - Range selection (e.g., 1-5)
  - Individual selection (e.g., 1,3,5)
  - Combined selection (e.g., 1-3,7,9-12)
  - Cancel operation (q/Q)
- Command line options:
  - `--ignore <dir>` - Ignore specific directories
  - `--no-safe` - Disable safe mode
  - `--lang <lang>` - Set language (zh-CN/en-US)
  - `-s, --size` - Show directory sizes
  - `-i, --interactive` - Interactive mode
  - `-f, --log-file` - Write logs to file
  - `-l, --log-level` - Set log level
  - `--no-progress` - Disable progress bar
  - `--silent` - Suppress console output

### Technical Details
- Cross-platform support (Windows, macOS, Linux)
- Node.js 16.x, 18.x, 20.x compatibility
- Async/await implementation
- Error handling with fallback methods
- UTF-8 support for file paths
- Memory efficient directory traversal

### Development
- Automated testing with 7 test cases
- Pre-commit hooks ensure code quality
- GitHub Actions for continuous integration
- Automatic versioning and release creation
- NPM publishing workflow

[2.0.0]: https://github.com/yourusername/dnm/releases/tag/v2.0.0
[1.0.0]: https://github.com/yourusername/dnm/releases/tag/v1.0.0
