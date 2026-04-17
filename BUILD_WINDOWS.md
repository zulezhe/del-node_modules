# Windows 构建和发布说明

## 本地 Windows 构建

### 使用 PowerShell 脚本

在 Windows 上运行:

```powershell
# 1. 克隆仓库
git clone https://github.com/zulezhe/del-node_modules.git
cd del-node_modules

# 2. 运行构建脚本
.\build-windows.ps1
```

构建脚本会:
- ✅ 检查 Rust 和 Node.js 依赖
- ✅ 清理之前的构建
- ✅ 编译 Rust 二进制 (release 模式)
- ✅ 复制二进制到 `bin/dnm.exe`
- ✅ 显示构建信息和测试结果

### 手动构建

```powershell
# 编译 Rust 二进制
cargo build --release --target x86_64-pc-windows-msvc

# 复制二进制
Copy-Item "target\x86_64-pc-windows-msvc\release\dnm.exe" "bin\dnm.exe"

# 测试
.\bin\dnm.exe --version
```

---

## GitHub Actions 自动构建

### 触发条件

以下情况会自动构建 Windows 二进制:

1. **推送到 main 分支**
   - 触发文件：`src/**`, `Cargo.toml`, `package.json`

2. **推送到 develop 分支**
   - 触发文件：同上

3. **手动触发**
   - 在 GitHub Actions 页面点击 "Run workflow"

### 自动化流程

1. **编译 Rust 二进制**
   - 平台：Windows Server (GitHub Actions)
   - 目标：`x86_64-pc-windows-msvc`
   - 模式：Release (优化性能)

2. **上传 Artifact**
   - 保留期限：30 天
   - 名称：`dnm-windows-x86_64`

3. **发布到 GitHub Releases** (仅 main 分支)
   - 标签：`v{package.json version}`
   - 附件：`dnm.exe`
   - 自动生成发布说明

### 工作流文件

`.github/workflows/build-windows.yml`

---

## 构建产物

### 二进制文件位置

- **开发构建**: `target\x86_64-pc-windows-msvc\release\dnm.exe`
- **发布构建**: `bin\dnm.exe`
- **GitHub Release**: 附件下载

### 文件大小

典型大小：1.5-2.5 MB (取决于 Rust 依赖)

---

## 发布流程

### 1. 更新版本号

```bash
# 编辑 package.json
{
  "name": "dnm",
  "version": "2.0.1",  # 更新这里
  ...
}
```

### 2. 更新 CHANGELOG

```md
## [2.0.1] - 2025-04-17

### Fixed
- 修复 xxx 问题
### Added
- 添加 xxx 功能
```

### 3. 提交并推送到 main

```bash
git add .
git commit -m "chore: 发布 v2.0.1"
git push origin main
```

### 4. 自动发布

GitHub Actions 会自动:
- ✅ 运行测试
- ✅ 构建 Windows 二进制
- ✅ 创建 GitHub Release
- ✅ 上传 `dnm.exe` 作为附件
- ✅ (可选) 发布到 NPM

---

## 使用方式

### 方式 1: 下载预编译二进制

1. 访问 [GitHub Releases](https://github.com/zulezhe/del-node_modules/releases)
2. 下载最新版本的 `dnm.exe`
3. 直接运行：`.\dnm.exe --help`

### 方式 2: 通过 NPM 安装

```bash
npm install -g dnm
dnm --help
```

NPM 包会自动包含预编译的 Windows 二进制。

---

## 故障排查

### 问题：构建脚本失败

**解决**:
```powershell
# 检查 Rust 版本
rustc --version

# 更新 Rust
rustup update

# 手动编译
cargo build --release
```

### 问题：Binary 找不到

**解决**:
```powershell
# 检查 bin/ 目录
ls bin

# 如果没有，手动复制
Copy-Item "target\x86_64-pc-windows-msvc\release\dnm.exe" "bin\dnm.exe"
```

### 问题: GitHub Actions 失败

**解决**:
- 检查 `.github/workflows/build-windows.yml` 语法
- 查看 Actions 日志
- 确保 Rust 版本 >= 1.70

---

## 性能优化

### 构建时间

- **首次构建**: 5-10 分钟 (下载依赖)
- **后续构建**: 30-60 秒 (使用缓存)

### 二进制优化

Cargo.toml 已配置优化选项:

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true  # 减小文件大小
```

这些选项可以让二进制文件更小、更快。

---

## 下一步

1. **测试构建的二进制**
   ```powershell
   .\bin\dnm.exe C:\Your\Test\Path
   ```

2. **上传到 GitHub Releases**
   - 手动上传或等待自动发布

3. **发布到 NPM**
   - 包含 Windows 二进制
   - 用户 `npm install -g dnm` 即可使用
