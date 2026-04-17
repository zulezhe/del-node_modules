# Contributing

Contributions are welcome! Here's how you can help.

## Development Setup

### Option 1: Full Development (Recommended)

```bash
# Clone the repo
git clone https://github.com/yourusername/dnm.git
cd dnm

# Install dependencies (automatically builds Rust binary)
npm install

# Link for local development
npm link

# Run tests
npm test
cargo test
```

### Option 2: Rust-Only Development

```bash
# Clone the repo
git clone https://github.com/yourusername/dnm.git
cd dnm

# Build Rust binary
cargo build --release

# Run tests
cargo test
```

## Workflow

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests:
   - Rust: `cargo test`
   - Node.js: `npm test`
5. Format code: `cargo fmt`
6. Commit (`git commit -m 'Add amazing feature'`)
7. Push (`git push origin feature/amazing-feature`)
8. Open a Pull Request

## Code Style

### Rust

- Follow Rust naming conventions
- Use 4 spaces for indentation
- Run `cargo fmt` before committing
- Run `cargo clippy` for linting
- Add tests for new features

### JavaScript

- Follow existing code style
- Keep functions focused and small
- Add tests for new features
- Update documentation

## Documentation

Update the following as needed:

- `README.md` - Main user documentation
- `RUST_README.md` - Rust-specific documentation
- `CHANGELOG.md` - Version history
- `docs/` - Additional documentation

## Testing

### Rust Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

### Node.js Tests

```bash
npm test
```

## Adding Translations

### Rust (src/i18n.rs)

Add to both `zh-CN` and `en-US` sections:

```rust
translations.insert("newKey".to_string(), "中文翻译".to_string());
// ...
translations.insert("newKey".to_string(), "English translation".to_string());
```

Use in code:
```rust
i18n.t("newKey", &[("param", &value)])
```

### JavaScript (if needed)

Add to `lib/i18n.js`:

```javascript
'zh-CN': {
  newKey: '中文翻译'
},
'en-US': {
  newKey: 'English translation'
}
```

## Building for Release

```bash
# Release build (optimized)
cargo build --release

# Or via npm
npm run build
```

## Reporting Issues

- Use [GitHub Issues](https://github.com/yourusername/dnm/issues)
- Include your OS, Rust version, and dnm version
- Provide steps to reproduce
- Attach logs if applicable

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
