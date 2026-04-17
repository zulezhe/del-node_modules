# dnm 使用方式说明

dnm 支持两种使用方式：**NPM 安装** 和 **下载二进制直接运行**。

---

## 方式一：NPM 安装（推荐）

### 1. 全局安装

```bash
# 全局安装，可以在任何地方使用 dnm 命令
npm install -g dnm
```

### 2. 使用

```bash
# 直接运行
dnm

# 指定目录
dnm /path/to/directory

# 查看帮助
dnm --help
```

### 3. 在 Node.js 项目中作为模块使用

```bash
# 在项目中安装
npm install dnm
```

```javascript
const { findAndDeleteNodeModules } = require('dnm');

// 调用
await findAndDeleteNodeModules('/path/to/directory', {
  safeMode: false,
  showSize: true
});
```

### 4. 无需安装的临时使用

```bash
# 使用 npx 临时运行
npx dnm /path/to/directory
```

---

## 方式二：下载二进制直接运行

### 1. 下载预编译二进制

从 [GitHub Releases](https://github.com/yourusername/dnm/releases) 下载对应平台的二进制文件：

- **Windows**: `dnm.exe`
- **macOS/Linux**: `dnm`

### 2. 使用预编译二进制

```bash
# 直接运行
./dnm

# 或添加到 PATH
mv dnm /usr/local/bin/
dnm --help
```

---

## 方式三：从源码构建

### 1. 克隆仓库

```bash
git clone https://github.com/yourusername/dnm.git
cd dnm
```

### 2. 编译 Rust 二进制

```bash
# 发布版本（性能最优）
cargo build --release

# 调试版本
cargo build
```

### 3. 运行

```bash
# 直接运行 Rust 二进制
./target/release/dnm

# 或通过 NPM wrapper 运行
npm run start
```

编译后的二进制文件位置：
- **Windows**: `target\release\dnm.exe`
- **macOS/Linux**: `target/release/dnm`

---

## 工作原理

```
┌─────────────────────────────────────────┐
│  用户运行 dnm 命令                        │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│  NPM Wrapper (js/cli.js)                │
│  - 查找 Rust 二进制文件                    │
│    1. bin/ 目录（已安装）                 │
│    2. target/release/（开发中）           │
│    3. PATH 中的 dnm                      │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│  Rust 二进制 (dnm/dnm.exe)               │
│  - 执行核心删除逻辑                      │
│  - 高性能扫描                            │
│  - 平台优化删除                          │
└─────────────────────────────────────────┘
```

---

## 平台差异

### Windows

- 二进制文件：`dnm.exe`
- 删除命令：`rd /s /q` (最优性能)
- 备用方案：PowerShell `Remove-Item`, Rust `fs::remove_dir_all`

### macOS/Linux

- 二进制文件：`dnm`
- 删除命令：`rm -rf`
- 备用方案：Rust `fs::remove_dir_all`

---

## 常见问题

### Q: 为什么 NPM 安装后还需要编译？

**A**: dnm 的核心功能使用 Rust 编写，NPM package 包含：
1. JS wrapper (`js/cli.js`)
2. 预编译的 Rust 二进制文件（`bin/` 目录）

如果预编译二进制不可用，需要自己编译。

### Q: 如何验证是否使用了 Rust 二进制？

**A**: 运行时会看到高性能的扫描和删除。你也可以检查进程：
- Windows: 会看到 `dnm.exe` 进程
- macOS/Linux: 会看到 `dnm` 进程

### Q: 可以直接删除 node_modules 吗？

**A**: 可以！运行 Rust 二进制文件不需要 Node.js：
```bash
./target/release/dnm /path/to/directory
```

### Q: 两种方式的性能有区别吗？

**A**: 没有区别。NPM wrapper 直接调用 Rust 二进制，性能完全一样。

---

## 推荐用法

| 场景 | 推荐方式 |
|------|----------|
| 经常使用 | `npm install -g dnm` |
| 一次性使用 | `npx dnm` 或下载二进制 |
| 开发贡献 | 从源码构建 |
| CI/CD 集成 | 下载二进制或全局安装 |

---

## 故障排查

### 问题：找不到 Rust 二进制

**解决**：
```bash
# 自行编译
cargo build --release

# 或确保 bin/ 目录有对应平台的二进制文件
```

### 问题：Windows 上删除很慢

**解决**：
- 确保使用的是最新版本（已优化 Windows 删除策略）
- 使用 `rd /s /q` 命令（Rust 版本会自动使用）
- 某些文件被占用时会自动重试（最多 8 次）

### 问题：权限错误

**解决**：
- macOS/Linux: 使用 `sudo dnm`
- Windows: 以管理员身份运行命令提示符
