# 🗑️ dnm (Delete Node Modules)

[![CI/CD Pipeline](https://github.com/yourusername/dnm/workflows/CI/CD%20Pipeline/badge.svg)](https://github.com/yourusername/dnm/actions)
[![npm version](https://badge.fury.io/js/dnm.svg)](https://www.npmjs.com/package/dnm)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

一个强大的 Node.js 工具，用于递归删除指定路径下的所有 `node_modules` 目录，支持多语言、进度显示、交互式操作等高级功能。

A powerful Node.js utility for recursively deleting all `node_modules` directories under a specified path, with multi-language support, progress tracking, interactive mode, and more.

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
- ⚡ **快速删除** | Fast deletion using platform-specific commands

### 🔒 安全特性 | Safety Features

- **默认安全模式**: 扫描后先显示列表，用户确认后再删除
- **灵活选择**: 支持删除全部、范围选择（1-5）、单独选择（1,3,5）
- **忽略保护**: 可配置忽略特定目录不被删除
- **错误处理**: 优雅处理权限问题和访问错误

---

## 📦 安装 | Installation

### 全局安装 | Global Installation

```bash
npm install -g dnm
```

### 本地开发 | Local Development

```bash
git clone https://github.com/yourusername/dnm.git
cd dnm
npm install
npm link
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
dnm [directory] [options]
```

#### 选项说明 | Options

| 选项 | 说明 | Option | Description |
|------|------|--------|-------------|
| `-h, --help` | 显示帮助信息 | Show help message |
| `-i, --interactive` | 交互模式 | Interactive mode |
| `-s, --size` | 显示目录大小 | Show directory sizes |
| `--no-progress` | 禁用进度条 | Disable progress bar |
| `-l, --log-level <level>` | 设置日志级别 | Set log level (debug/info/warn/error) |
| `-f, --log-file <path>` | 保存日志到文件 | Write logs to file |
| `--silent` | 静默模式 | Suppress console output |
| `--lang, --language <lang>` | 设置语言 | Set language (zh-CN/en-US) |
| `--ignore <dir>` | 忽略指定目录 | Ignore specified directories |
| `--no-safe` | 禁用安全模式 | Disable safe mode |

### 使用示例 | Examples

#### 1️⃣ 基础删除 | Basic Deletion

```bash
# 当前目录（安全模式）
dnm

# 指定目录
dnm /path/to/projects

# 禁用安全模式（直接删除）
dnm --no-safe
```

#### 2️⃣ 显示大小 | Show Sizes

```bash
# 显示每个目录的大小
dnm -s

# 英文 + 显示大小
dnm --lang en-US -s
```

#### 3️⃣ 忽略目录 | Ignore Directories

```bash
# 忽略单个目录
dnm --ignore backup

# 忽略多个目录
dnm --ignore backup --ignore node_modules_old

# 组合使用
dnm -s --ignore backup --ignore temp
```

#### 4️⃣ 日志功能 | Logging

```bash
# 保存日志到文件
dnm -f cleanup.log

# 调试模式 + 日志
dnm -l debug -f debug.log

# 静默模式（仅文件日志）
dnm --silent -f cleanup.log
```

#### 5️⃣ 交互模式 | Interactive Mode

```bash
# 启动交互模式（推荐新手）
dnm -i

# 或
dnm --interactive
```

交互模式会引导您完成以下配置：
- 选择语言
- 指定目标目录
- 配置显示选项
- 设置日志级别
- 配置忽略目录
- 启用/禁用安全模式

---

## 🛡️ 安全模式详解 | Safe Mode Details

### 默认启用 | Enabled by Default

安全模式下，dnm 会先扫描并显示所有找到的 `node_modules` 目录，等待用户确认后再执行删除操作。

In safe mode, dnm scans and displays all found `node_modules` directories, waiting for user confirmation before deletion.

### 选择语法 | Selection Syntax

| 输入 | 说明 | Input | Description |
|------|------|-------|-------------|
| *(留空)* | 删除全部 | Delete all |
| `3` | 删除第3个 | Delete #3 |
| `1-5` | 删除1到5 | Delete #1 to #5 |
| `1,3,5` | 删除1、3、5 | Delete #1, #3, #5 |
| `1-3,7,9-12` | 组合选择 | Combined selection |
| `q` 或 `Q` | 取消操作 | Cancel operation |

### 禁用安全模式 | Disable Safe Mode

```bash
dnm --no-safe
```

---

## 🌐 多语言支持 | Multi-language Support

### 切换语言 | Switch Language

```bash
# 中文（默认）
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
npm test
```

### 测试覆盖 | Test Coverage

- ✅ 基础删除功能
- ✅ 忽略目录功能
- ✅ 多语言支持
- ✅ 空目录处理
- ✅ 嵌套目录处理
- ✅ 多重忽略模式

---

## 🔧 开发 | Development

### 项目结构 | Project Structure

```
dnm/
├── bin/
│   └── cli.js              # CLI 入口 | CLI entry point
├── lib/
│   ├── i18n.js             # 国际化 | Internationalization
│   └── logger.js           # 日志系统 | Logging system
├── .github/
│   └── workflows/
│       └── ci-cd.yml       # CI/CD 配置 | CI/CD configuration
├── .husky/
│   └── pre-commit          # Git 提交钩子 | Git pre-commit hook
├── index.js                # 核心逻辑 | Core logic
├── test.js                 # 测试文件 | Test file
├── package.json
└── README.md
```

### Git 提交钩子 | Git Hooks

项目使用 [Husky](https://github.com/typicode/husky) 实现 Git 提交钩子：

- **pre-commit**: 提交前自动运行测试
- 测试失败会阻止提交

```bash
# 安装钩子
npm install

# 测试会在提交前自动运行
git commit -m "your message"
```

### CI/CD 流程 | CI/CD Pipeline

#### 自动化测试 | Automated Testing

- ✅ 多平台测试 (Ubuntu, Windows, macOS)
- ✅ 多 Node.js 版本 (16.x, 18.x, 20.x)
- ✅ 推送到 main/develop 分支时触发
- ✅ Pull Request 时触发

#### 自动发布 | Automatic Release

推送到 `main` 分支时自动执行：

1. ✅ 运行所有测试
2. ✅ 创建 Git tag (基于 package.json 版本)
3. ✅ 生成 Release Notes
4. ✅ 创建 GitHub Release
5. ✅ 发布到 NPM (需配置 `NPM_TOKEN`)

---

## 📊 性能 | Performance

- **快速扫描**: 高效的递归目录遍历
- **智能删除**: 使用平台特定命令优化删除速度
- **停止递归**: 发现 `node_modules` 后立即停止深入，避免不必要的扫描

---

## ⚠️ 注意事项 | Important Notes

1. **不可恢复**: 删除操作不可恢复，请谨慎使用
2. **安全模式**: 建议使用默认的安全模式，确认后再删除
3. **权限问题**: 某些目录可能需要管理员权限
4. **备份建议**: 重要项目请先备份

---

## 🤝 贡献 | Contributing

欢迎贡献！请随时提交 Pull Request。

Contributions are welcome! Please feel free to submit a Pull Request.

### 开发流程 | Development Workflow

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

---

## 📄 许可证 | License

MIT © [Your Name]

---

## 🔗 链接 | Links

- **GitHub**: https://github.com/yourusername/dnm
- **NPM**: https://www.npmjs.com/package/dnm
- **Issues**: https://github.com/yourusername/dnm/issues
- **Changelog**: https://github.com/yourusername/dnm/releases

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

---

## 🎯 路线图 | Roadmap

- [ ] 添加更多语言支持
- [ ] 支持自定义配置文件
- [ ] 添加干运行模式（--dry-run）
- [ ] 支持软链接处理
- [ ] Web UI 界面

---

## 🙏 致谢 | Acknowledgments

- [chalk](https://github.com/chalk/chalk) - 终端样式
- [inquirer](https://github.com/SBoudrias/Inquirer.js) - 交互式命令行
- [cli-progress](https://github.com/npkgz/cli-progress) - 进度条
- [husky](https://github.com/typicode/husky) - Git 钩子

---

**Made with ❤️ for the Node.js community**
