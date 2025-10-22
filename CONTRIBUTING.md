# Contributing to dnm

Thank you for your interest in contributing to dnm! This document provides guidelines and instructions for contributing.

## 🤝 How to Contribute

### Reporting Bugs

If you find a bug, please create an issue with the following information:

- **Bug Description**: Clear description of the issue
- **Steps to Reproduce**: Detailed steps to reproduce the bug
- **Expected Behavior**: What you expected to happen
- **Actual Behavior**: What actually happened
- **Environment**: OS, Node.js version, dnm version
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
   npm install
   ```

4. **Make Your Changes**
   - Follow the existing code style
   - Add tests for new features
   - Update documentation as needed

5. **Run Tests**
   ```bash
   npm test
   ```

6. **Commit Your Changes**
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

7. **Push to Your Fork**
   ```bash
   git push origin feature/your-feature-name
   ```

8. **Create a Pull Request**
   - Go to the original repository
   - Click "New Pull Request"
   - Select your branch
   - Fill in the PR template

## 📝 Development Guidelines

### Code Style

- Use 2 spaces for indentation
- Use meaningful variable names
- Add comments for complex logic
- Follow existing patterns in the codebase

### Testing

- Write tests for new features
- Ensure all tests pass before submitting PR
- Aim for high test coverage

### Documentation

- Update README.md for user-facing changes
- Update inline comments for code changes
- Update CHANGELOG.md following Keep a Changelog format

### Internationalization

When adding new strings:

1. Add to both language objects in `lib/i18n.js`:
   ```javascript
   'zh-CN': {
     newKey: '中文翻译'
   },
   'en-US': {
     newKey: 'English translation'
   }
   ```

2. Use the translation in code:
   ```javascript
   i18n.t('newKey', { param: value })
   ```

## 🔧 Development Setup

### Prerequisites

- Node.js 16.x or higher
- npm 7.x or higher
- Git

### Local Development

```bash
# Clone repository
git clone https://github.com/yourusername/dnm.git
cd dnm

# Install dependencies
npm install

# Link for local testing
npm link

# Run tests
npm test

# Test CLI
dnm --help
```

### Project Structure

```
dnm/
├── bin/
│   └── cli.js              # CLI entry point
├── lib/
│   ├── i18n.js             # Internationalization
│   └── logger.js           # Logging system
├── .github/
│   └── workflows/
│       └── ci-cd.yml       # CI/CD configuration
├── .husky/
│   └── pre-commit          # Pre-commit hook
├── index.js                # Core logic
├── test.js                 # Test suite
├── package.json
└── README.md
```

## 🧪 Testing

### Running Tests

```bash
# Run all tests
npm test

# Test specific functionality
node test.js
```

### Adding Tests

Add new test cases to `test.js`:

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

## 🚀 Release Process

Releases are automated via GitHub Actions:

1. Update version in `package.json`
2. Update `CHANGELOG.md`
3. Commit changes
4. Push to `main` branch
5. GitHub Actions will:
   - Run tests
   - Create git tag
   - Create GitHub release
   - Publish to NPM

## 📋 Checklist Before Submitting PR

- [ ] Tests pass (`npm test`)
- [ ] Code follows project style
- [ ] Documentation updated
- [ ] CHANGELOG.md updated (if needed)
- [ ] Commit messages are clear
- [ ] PR description is detailed
- [ ] No merge conflicts

## 💡 Questions?

If you have questions:

- Check existing issues
- Create a new issue with the "question" label
- Reach out to maintainers

## 📄 License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to dnm! 🎉
