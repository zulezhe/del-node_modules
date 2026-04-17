# Contributing to dnm

Thank you for your interest in contributing to dnm! This document provides guidelines and instructions for contributing.

## 🤝 How to Contribute

### Reporting Bugs

If you find a bug, please create an issue with the following information:

- **Bug Description**: Clear description of the issue
- **Steps to Reproduce**: Detailed steps to reproduce the bug
- **Expected Behavior**: What you expected to happen
- **Actual Behavior**: What actually happened
- **Environment**: OS, Node.js/Rust version, dnm version
- **Screenshots/Logs**: If applicable

### Suggesting Features

Feature requests are welcome! Please create an issue with:

- **Feature Description**: Clear description of the feature
- **Use Case**: Why this feature would be useful
- **Proposed Solution**: How you think it should work
- **Alternatives**: Alternative solutions you've considered

### Pull Requests

1. **Fork the Repository**
   ```bash
   git clone https://github.com/yourusername/dnm.git
   cd dnm
   ```

2. **Create a Branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **Install Dependencies**
   ```bash
   # Install Node.js dependencies
   npm install
   
   # Rust will be built automatically during npm install
   ```

4. **Make Your Changes**
   - Follow the existing code style
   - Add tests for new features
   - Update documentation as needed

5. **Run Tests**
   ```bash
   # Run Node.js test suite
   npm test
   
   # Run Rust test suite
   cargo test
   ```

6. **Build the Rust Binary**
   ```bash
   # Release build
   npm run build
   
   # Or directly
   cargo build --release
   ```

7. **Commit Your Changes**
   ```bash
   git add .
   git commit -m "Add: your feature description"
   ```
   
   Commit message format:
   - `Add:` for new features
   - `Fix:` for bug fixes
   - `Update:` for improvements
   - `Docs:` for documentation
   - `Test:` for tests
   - `Refactor:` for refactoring

8. **Push to Your Fork**
   ```bash
   git push origin feature/your-feature-name
   ```

9. **Create a Pull Request**
   - Go to the original repository
   - Click "New Pull Request"
   - Select your branch
   - Fill in the PR template

## 📝 Development Guidelines

### Code Style

#### Rust Code
- Use 4 spaces for indentation
- Follow Rust naming conventions (snake_case for functions/variables, PascalCase for types)
- Use Rust's built-in error handling (Result, Option)
- Add comments for complex logic
- Follow existing patterns in the codebase

#### JavaScript Code
- Use 2 spaces for indentation
- Use meaningful variable names
- Add comments for complex logic
- Follow existing patterns in the codebase

### Testing

#### Rust Tests

Add tests in `src/tests.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_name() {
        // Arrange
        let input = setup();
        
        // Act
        let result = function_under_test(input);
        
        // Assert
        assert_eq!(expected, result);
    }
}
```

Run Rust tests:
```bash
cargo test
```

#### Node.js Tests

Add test cases to `test.js`:

```javascript
runTest('Test name', async () => {
  // Setup
  setupTestStructure();
  
  // Execute
  const result = await findAndDeleteNodeModules(testDir, options);
  
  // Assert
  assert(condition, 'error message');
  
  // Cleanup
  cleanup();
});
```

Run Node.js tests:
```bash
npm test
```

- Ensure all tests pass before submitting PR
- Aim for high test coverage

### Documentation

- Update README.md for user-facing changes
- Update inline comments for code changes
- Update CHANGELOG.md following Keep a Changelog format
- Update RUST_README.md for Rust-specific changes

### Internationalization

When adding new strings:

#### Rust (src/i18n.rs)

```rust
// Add to both zh-CN and en-US translation sections
translations.insert("newKey".to_string(), "中文翻译".to_string());
// ...
translations.insert("newKey".to_string(), "English translation".to_string());
```

Use in Rust code:
```rust
i18n.t("newKey", &[("param", &value)])
```

#### JavaScript (lib/i18n.js)

```javascript
'zh-CN': {
  newKey: '中文翻译'
},
'en-US': {
  newKey: 'English translation'
}
```

Use in JavaScript code:
```javascript
i18n.t('newKey', { param: value })
```

## 🔧 Development Setup

### Prerequisites

#### For Rust Development
- Rust 1.70+ (edition 2021)
- Cargo (comes with Rust)

#### For Node.js Development
- Node.js 14.x or higher
- npm 6.x or higher
- Git

### Local Development

#### Option 1: Full Development Environment

```bash
# Clone repository
git clone https://github.com/yourusername/dnm.git
cd dnm

# Install dependencies (automatically builds Rust binary)
npm install

# Link for local testing
npm link

# Run tests
npm test
cargo test

# Test CLI
dnm --help
```

#### Option 2: Rust-Only Development

```bash
# Clone repository
git clone https://github.com/yourusername/dnm.git
cd dnm

# Build Rust binary
cargo build --release

# Run tests
cargo test

# Test CLI
./target/release/dnm --help
```

### Project Structure

```
dnm/
├── src/                    # Rust source code
│   ├── main.rs            # CLI entry point (Rust)
│   ├── lib.rs             # Core library (Rust)
│   ├── cli.rs             # Command-line argument parsing
│   ├── scanner.rs         # Directory scanning logic
│   ├── deleter.rs         # Deletion logic
│   ├── i18n.rs            # Internationalization
│   ├── logger.rs          # Logging system
│   ├── interactive.rs     # Interactive mode
│   ├── utils.rs           # Utility functions
│   └── tests.rs           # Rust test suite
├── js/                     # JavaScript wrappers
│   ├── wrapper.js         # Node.js module wrapper
│   └── cli.js             # CLI wrapper (legacy)
├── lib/                    # JavaScript utilities
│   ├── i18n.js            # Legacy i18n (if needed)
│   └── logger.js          # Legacy logger (if needed)
├── bin/                    # Pre-built binaries
│   └── cli.js             # Binary entry point
├── .github/
│   └── workflows/
│       └── ci-cd.yml      # CI/CD configuration
├── .husky/
│   └── pre-commit         # Pre-commit hook
├── Cargo.toml             # Rust dependencies
├── package.json           # Node.js configuration
├── index.js               # Main entry (exports wrapper)
├── test.js                # Node.js test suite
└── README.md
```

## 🚀 Build Commands

### Rust

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run program
cargo run -- [options]

# Format code
cargo fmt

# Lint code
cargo clippy
```

### Node.js

```bash
# Install dependencies and build Rust binary
npm install

# Build Rust binary only
npm run build

# Debug build
npm run build:debug

# Run tests
npm test

# Copy binary to bin directory
npm run copy-binary
```

## 🚀 Release Process

Releases are automated via GitHub Actions:

1. Update version in `package.json`
2. Update version in `Cargo.toml`
3. Update `CHANGELOG.md`
4. Commit changes
5. Push to `main` branch
6. GitHub Actions will:
   - Run tests (Rust and Node.js)
   - Create git tag
   - Build binaries for multiple platforms
   - Create GitHub release
   - Publish to NPM

## 📋 Checklist Before Submitting PR

- [ ] Rust tests pass (`cargo test`)
- [ ] Node.js tests pass (`npm test`)
- [ ] Rust code follows project style (cargo fmt, cargo clippy)
- [ ] JavaScript code follows project style
- [ ] Documentation updated (README.md, RUST_README.md)
- [ ] CHANGELOG.md updated (if needed)
- [ ] Commit messages are clear
- [ ] PR description is detailed
- [ ] No merge conflicts
- [ ] Both Rust and JavaScript versions aligned (if applicable)

## 💡 Questions?

If you have questions:

- Check existing issues
- Create a new issue with the "question" label
- Reach out to maintainers

## 📄 License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to dnm! 🎉
