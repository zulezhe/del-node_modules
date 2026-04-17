# 🗑️ dnm (Delete Node Modules)

[![CI/CD Pipeline](https://github.com/yourusername/dnm/workflows/CI%2FCD/badge.svg)](https://github.com/yourusername/dnm/actions)
[![npm version](https://badge.fury.io/js/dnm.svg)](https://www.npmjs.com/package/dnm)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![Rust](https://img.shields.io/badge/rust-powered-orange)

一个强大的跨平台工具，用于递归删除指定路径下的所有 `node_modules` 目录。使用 Rust 编写，提供极高的性能和极低的内存占用，同时支持作为独立 CLI 工具或 Node.js 模块使用。

A powerful cross-platform utility for recursively deleting all `node_modules` directories under a specified path. Built with Rust for maximum performance and minimal memory usage, available as both a standalone CLI tool and a Node.js module.

---

## ✨ 特性 | Features

### 🌟 核心功能 | Core Features

- 🔍 **递归扫描** | Recursive scanning for all `node_modules` directories
- 🛡️ **安全模式（默认）** | Safe mode with confirmation (default)
- 📊 **进度条显示** | Real-time progress bar
- 🎨 **彩色终端输出** | Beautiful colored terminal output
- 💬 **交互式模式** | Interactive mode with user-friendly prompts
- 📝 **高级日志系统** | Advanced logging with multiple levels
- 🌐 **多语言支持** | Multi-language support (中文/English)
- 📏 **大小计算** | Optional directory size calculation
- 🚫 **忽略目录** | Ignore specific directories
- ⚡ **极速删除** | Ultra-fast deletion using Rust's native performance

### 🔒 安全特性 | Safety Features

- **默认安全模式**: 扫描后先显示列表，用户确认后再删除
- **灵活选择**: 支持删除全部、范围选择（1-5）、单独选择（1,3,5）
- **忽略保护**: 可配置忽略特定目录不被删除
- **错误处理**: 优雅处理权限问题和访问错误

### 🚀 性能优势 | Performance Advantages

| 指标 | 旧版 (Node.js) | 新版 (Rust) | 提升 |
|------|----------------|-------------|------|
| 扫描速度 | ~2s (100 个项目) | ~0.3s | **6-7x** |
| 内存占用 | ~50MB | ~5MB | **10x** |
| 启动时间 | ~500ms | ~50ms | **10x** |

---

## 📦 安装 | Installation

### 方式 1: 独立 CLI 工具 | Standalone CLI Tool

```bash
# 克隆仓库 | Clone repository
git clone https://github.com/yourusername/dnm.git
cd dnm

# 编译 | Build
cargo build --release

# 使用（Linux/macOS）| Use (Linux/macOS)
./target/release/dnm

# 使用（Windows）| Use (Windows)
.\target\release\dnm.exe
```

### 方式 2: 通过 NPM 安装（全局命令）| Via NPM (Global Command)

```bash
# 全局安装 | Global installation
npm install -g dnm

# 使用 | Use
dnm [目录] [选项]
```

### 方式 3: 作为 Node.js 模块调用 | As Node.js Module

```bash
# 本地安装 | Local installation
npm install dnm
```

```javascript
const { findAndDeleteNodeModules } = require('dnm');

// 基础使用 | Basic usage
findAndDeleteNodeModules('/path/to/directory', {
  safeMode: false,
  showSize: true,
  showProgress: true,
  language: 'zh-CN'
}).then(result => {
  console.log(`删除了 ${result.total} 个目录 | Deleted ${result.total} directories`);
});
```

### 方式 4: 本地开发 | Local Development

```bash
# 克隆并安装 | Clone and install
git clone https://github.com/yourusername/dnm.git
cd dnm
npm install
npm run build  # 编译 Rust 二进制文件 | Build Rust binary
```

---

## 🚀 快速开始 | Quick Start

### 基础使用 | Basic Usage

```bash
# 在当前目录下删除所有 node_modules（中文、安全模式）
# Delete all node_modules in current directory (Chinese, safe mode)
dnm

# 指定目录 | Specify directory
dnm /path/to/directory

# 英文界面 | English interface
dnm --lang en-US
```

### 安全模式示例 | Safe Mode Example

```bash
$ dnm

╔════════════════════════════════════════════════╗
║     🗑️  dnm - 清理工具                        ║
╚════════════════════════════════════════════════╝

🔍 目标目录: F:\projects

正在扫描 F:\projects 查找 node_modules 目录...
发现 5 个 node_modules 目录

═══════════════════════════════════════════════════
  安全模式 - 发现的 node_modules 目录
═══════════════════════════════════════════════════

[1] F:\projects\app1\node_modules (150.5 MB)
[2] F:\projects\app2\node_modules (89.2 MB)
[3] F:\projects\lib\node_modules (45.8 MB)
[4] F:\test\demo\node_modules (12.3 MB)
[5] F:\backup\old\node_modules (200.1 MB)

输入要删除的序号（例如: 1-5 或 1,3,5），留空删除全部，输入 q 取消: 1-3

已选择删除 3 个目录
```

---

## 📖 使用方法 | Usage

### 命令行选项 | Command Line Options

```bash
dnm [目录] [选项]
```

#### 选项说明 | Options

| 选项 | 说明 |
|------|------|
| `-h, --help` | 显示帮助信息 | Show help message |
| `-i, --interactive` | 交互模式 | Interactive mode |
| `-s, --size` | 显示目录大小 | Show directory sizes |
| `--no-progress` | 禁用进度条 | Disable progress bar |
| `-l, --log-level <level>` | 设置日志级别 (debug/info/warn/error) | Set log level |
| `-f, --log-file <path>` | 保存日志到文件 | Write logs to file |
| `--silent` | 静默模式 | Suppress console output |
| `--lang <lang>` | 设置语言 (zh-CN/en-US) | Set language |
| `--ignore <dir>` | 忽略指定目录（可多次使用）| Ignore directories (repeatable) |
| `--no-safe` | 禁用安全模式 | Disable safe mode |
| `-V, --version` | 显示版本 | Show version |

### 使用示例 | Examples

#### 1️⃣ 基础删除 | Basic Deletion

```bash
# 当前目录（安全模式）| Current directory (safe mode)
dnm

# 指定目录 | Specify directory
dnm /path/to/projects

# 禁用安全模式（直接删除）| Disable safe mode (direct deletion)
dnm --no-safe
```

#### 2️⃣ 显示大小 | Show Sizes

```bash
# 显示每个目录的大小 | Show sizes
dnm -s

# 英文 + 显示大小 | English + sizes
dnm --lang en-US -s
```

#### 3️⃣ 忽略目录 | Ignore Directories

```bash
# 忽略单个目录 | Ignore single directory
dnm --ignore backup

# 忽略多个目录 | Ignore multiple directories
dnm --ignore backup --ignore node_modules_old

# 组合使用 | Combined
dnm -s --ignore backup --ignore temp
```

#### 4️⃣ 日志功能 | Logging

```bash
# 保存日志到文件 | Save logs to file
dnm -f cleanup.log

# 调试模式 + 日志 | Debug mode + logs
dnm -l debug -f debug.log

# 静默模式（仅文件日志）| Silent mode (file only)
dnm --silent -f cleanup.log
```

#### 5️⃣ 交互模式 | Interactive Mode

```bash
# 启动交互模式 | Start interactive mode
dnm -i
```

交互模式会引导您完成以下配置：
- 选择语言 | Select language
- 指定目标目录 | Specify target directory
- 配置显示选项 | Configure display options
- 设置日志级别 | Set log level
- 配置忽略目录 | Configure ignore directories
- 启用/禁用安全模式 | Enable/disable safe mode

---

## 🛡️ 安全模式详解 | Safe Mode Details

### 默认启用 | Enabled by Default

安全模式下，dnm 会先扫描并显示所有找到的 `node_modules` 目录，等待用户确认后再执行删除操作。

In safe mode, dnm scans and displays all found `node_modules` directories, waiting for user confirmation before deletion.

### 选择语法 | Selection Syntax

| 输入 | 说明 |
|------|------|
| *(留空)* | 删除全部 | Delete all |
| `3` | 删除第 3 个 | Delete #3 |
| `1-5` | 删除 1 到 5 | Delete #1 to #5 |
| `1,3,5` | 删除 1、3、5 | Delete #1, #3, #5 |
| `1-3,7,9-12` | 组合选择 | Combined selection |
| `q` 或 `Q` | 取消操作 | Cancel operation |

---

## 🌐 多语言支持 | Multi-language Support

### 切换语言 | Switch Language

```bash
# 中文（默认）| Chinese (default)
dnm --lang zh-CN

# English
dnm --lang en-US
```

### 支持的语言 | Supported Languages

- 🇨🇳 **简体中文** (zh-CN) - 默认 | Default
- 🇺🇸 **English** (en-US)

---

## 🧪 测试 | Testing

### 运行测试 | Run Tests

```bash
# 运行 JS 测试套件 | Run JS test suite
npm test

# 运行 Rust 测试套件 | Run Rust test suite
cargo test
```

### 测试覆盖 | Test Coverage

- ✅ 基础删除功能 | Basic deletion
- ✅ 忽略目录功能 | Ignore directories
- ✅ 多语言支持 | Multi-language support
- ✅ 空目录处理 | Empty directory handling
- ✅ 嵌套目录处理 | Nested directory handling
- ✅ 多重忽略模式 | Multiple ignore patterns

---

## 🔧 开发 | Development

### 项目结构 | Project Structure

```
dnm/
├── src/                    # Rust 源代码 | Rust source code
│   ├── main.rs            # CLI 入口 | CLI entry point
│   ├── lib.rs             # 核心库 | Core library
│   ├── cli.rs             # 命令行参数 | CLI arguments
│   ├── scanner.rs         # 目录扫描 | Directory scanning
│   ├── deleter.rs         # 删除逻辑 | Deletion logic
│   ├── i18n.rs            # 国际化 | Internationalization
│   ├── logger.rs          # 日志系统 | Logging system
│   ├── interactive.rs     # 交互模式 | Interactive mode
│   ├── utils.rs           # 工具函数 | Utility functions
│   └── tests.rs           # 测试套件 | Test suite
├── js/                     # JS 包装器 | JS wrappers
│   ├── wrapper.js         # Node.js 模块 | Node.js module
│   └── cli.js             # CLI 包装器 | CLI wrapper
├── bin/                    # 预编译二进制 | Pre-built binaries
├── lib/                    # JS 工具库 | JS utilities
│   ├── i18n.js            # 国际化（旧版）| i18n (legacy)
│   └── logger.js          # 日志（旧版）| logger (legacy)
├── .github/
│   └── workflows/
│       └── ci-cd.yml      # CI/CD 配置 | CI/CD configuration
├── .husky/
│   └── pre-commit         # 预提交钩子 | Pre-commit hook
├── Cargo.toml             # Rust 配置 | Rust configuration
├── package.json           # NPM 配置 | NPM configuration
├── index.js               # 主入口 | Main entry
├── test.js                # Node.js 测试 | Node.js tests
└── README.md              # 使用说明 | Usage guide
```

### 常用命令 | Common Commands

#### Rust 开发 | Rust Development

```bash
# 调试构建 | Debug build
cargo build

# 发布构建 | Release build
cargo build --release

# 运行测试 | Run tests
cargo test

# 运行程序 | Run program
cargo run -- [options]

# 格式化代码 | Format code
cargo fmt

# 代码检查 | Lint code
cargo clippy
```

#### NPM 脚本 | NPM Scripts

```bash
# 编译 Rust 二进制文件 | Build Rust binary
npm run build         

# 调试构建 | Debug build
npm run build:debug   

# 复制二进制文件到 bin 目录 | Copy binary to bin
npm run copy-binary

# 运行测试 | Run tests
npm test              

# 安装依赖并构建 | Install and build
npm install           
```

### Git 提交钩子 | Git Hooks

- **pre-commit**: 提交前自动运行测试
- 测试失败会阻止提交

```bash
# 安装依赖（包括钩子）| Install dependencies (including hooks)
npm install

# 测试会在提交前自动运行
git commit -m "your message"
```

---

## 📊 CI/CD 流程 | CI/CD Pipeline

### 自动化测试 | Automated Testing

- ✅ 多平台测试 (Ubuntu, Windows, macOS)
- ✅ Rust 测试套件
- ✅ Node.js 测试套件
- ✅ 推送到 main/develop 分支时触发
- ✅ Pull Request 时触发

### 自动发布 | Automatic Release

推送到 `main` 分支时自动执行：

1. ✅ 运行所有测试 | Run all tests
2. ✅ 编译多平台二进制文件 | Build cross-platform binaries
3. ✅ 创建 Git tag (基于 package.json 版本) | Create Git tag
4. ✅ 生成 Release Notes | Generate release notes
5. ✅ 创建 GitHub Release (附带编译好的二进制) | Create GitHub Release with binaries
6. ✅ 发布到 NPM (需配置 `NPM_TOKEN`) | Publish to NPM

---

## ⚠️ 注意事项 | Important Notes

1. **不可恢复**: 删除操作不可恢复，请谨慎使用
2. **安全模式**: 建议使用默认的安全模式，确认后再删除
3. **权限问题**: 某些目录可能需要管理员权限
4. **备份建议**: 重要项目请先备份
5. **系统要求**: 
   - Rust 1.70+ (用于从源码构建)
   - Node.js 14.0+ (用于 NPM 安装)
   - 支持平台：Windows、macOS、Linux

---

## 🤝 贡献 | Contributing

欢迎贡献！请随时提交 Pull Request。

Contributions are welcome! Please feel free to submit a Pull Request.

### 开发流程 | Development Workflow

1. Fork 项目 | Fork the repository
2. 创建特性分支 | Create feature branch (`git checkout -b feature/AmazingFeature`)
3. 提交更改 | Commit changes (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 | Push to branch (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request | Open a Pull Request

---

## 📄 许可证 | License

MIT © [Your Name]

---

## 🔗 链接 | Links

- **GitHub**: https://github.com/yourusername/dnm
- **NPM**: https://www.npmjs.com/package/dnm
- **Issues**: https://github.com/yourusername/dnm/issues
- **Releases**: https://github.com/yourusername/dnm/releases

---

## 💡 常见问题 | FAQ

### Q: 安全模式和非安全模式有什么区别？

**A**: 安全模式（默认）会先显示所有待删除目录列表，让用户选择后再删除。非安全模式（`--no-safe`）会直接删除所有找到的 `node_modules` 目录。

### Q: 如何忽略多个目录？

**A**: 多次使用 `--ignore` 参数：
```bash
dnm --ignore backup --ignore old --ignore temp
```

### Q: 可以恢复已删除的目录吗？

**A**: 不可以。删除操作是永久性的，无法恢复。建议使用安全模式并仔细检查列表。

### Q: 支持哪些语言？

**A**: 目前支持简体中文（zh-CN）和英文（en-US），默认为简体中文。

### Q: 如何获取调试信息？

**A**: 使用调试日志级别并保存到文件：
```bash
dnm -l debug -f debug.log
```

### Q: 为什么使用 Rust 重写？

**A**: Rust 提供极高的性能（6-7 倍扫描速度提升）、极低的内存占用（10 倍减少）和更小的二进制文件大小，同时保持与原有 JS API 的完全兼容。

### Q: 如何在 Node.js 项目中使用？

**A**: 直接 `npm install dnm`，然后 `require('dnm')` 即可。JS 包装器会自动调用底层 Rust 二进制文件。

---

## 🎯 路线图 | Roadmap

- [ ] 添加更多语言支持 | Add more language support
- [ ] 支持自定义配置文件 | Support custom configuration files
- [ ] 添加干运行模式（--dry-run）| Add dry-run mode
- [ ] 支持软链接处理 | Support symlink handling
- [ ] Web UI 界面 | Web UI interface
- [ ] 提供预编译二进制文件下载 | Provide pre-built binary downloads

---

## 🙏 致谢 | Acknowledgments

- **Rust 生态系统** | Rust Ecosystem - 高性能基础库
- **colored** - 终端样式
- **indicatif** - 进度条
- **dialoguer** - 交互式命令行

---

**Made with ❤️ using Rust for maximum performance**
