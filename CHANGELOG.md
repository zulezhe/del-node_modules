# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

[1.0.0]: https://github.com/yourusername/dnm/releases/tag/v1.0.0
