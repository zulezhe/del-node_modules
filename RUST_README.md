# DNM (Delete Node Modules) - Rust 版本

这是使用 Rust 重写的 dnm 工具，保留原有所有功能，同时提供更高的性能和更小的内存占用。

## 特性

✅ 递归扫描并删除 node_modules 目录  
✅ 安全模式（默认）- 删除前显示列表供用户选择  
✅ 实时进度条显示  
✅ 彩色终端输出  
✅ 交互式模式 (-i)  
✅ 多语言支持 (zh-CN / en-US)  
✅ 日志系统（多级别 + 文件输出）  
✅ 忽略指定目录功能  
✅ 目录大小计算  
✅ 跨平台支持 (Windows / macOS / Linux)  

## 安装与使用

### 方式 1: 作为独立 CLI 工具使用

```bash
# 编译
cargo build --release

# 使用
./target/release/dnm [目录] [选项]
```

### 方式 2: 通过 NPM 安装（全局命令）

```bash
# 全局安装
npm install -g dnm

# 使用
dnm [目录] [选项]
```

### 方式 3: 作为 Node.js 模块调用

```javascript
const { findAndDeleteNodeModules } = require('dnm');

findAndDeleteNodeModules('/path/to/directory', {
  safeMode: false,
  showSize: true,
  showProgress: true,
  language: 'zh-CN'
}).then(result => {
  console.log(`删除了 ${result.total} 个目录`);
});
```

## 命令行选项

```bash
dnm [目录] [选项]

选项:
  -i, --interactive            交互模式
  -s, --size                   显示目录大小
      --no-progress            禁用进度条
  -l, --log-level <LEVEL>      日志级别 (debug, info, warn, error)
  -f, --log-file <PATH>        日志文件路径
      --silent                 静默模式
      --lang <LANG>            语言 (zh-CN, en-US)
      --ignore <DIR>           忽略目录（可多次使用）
      --no-safe                禁用安全模式
  -h, --help                   显示帮助
  -V, --version                显示版本
```

## 示例

```bash
# 基础使用（安全模式）
dnm
dnm /path/to/projects

# 直接删除（无确认）
dnm --no-safe

# 显示大小
dnm -s

# 忽略目录
dnm --ignore backup --ignore old

# 日志
dnm -f cleanup.log
dnm -l debug -f debug.log

# 交互模式
dnm -i

# 英文界面
dnm --lang en-US
```

## 安全模式

安全模式（默认）会：
1. 先扫描并显示所有找到的 node_modules 目录
2. 等待用户选择要删除的目录
3. 执行删除操作

选择语法：
- 留空 = 删除全部
- `3` = 删除第 3 个
- `1-5` = 删除 1 到 5
- `1,3,5` = 删除 1、3、5
- `q` = 取消操作

## 开发

```bash
# 调试构建
cargo build

# 发布构建
cargo build --release

# 运行测试
cargo test

# 运行程序
cargo run -- [选项]
```

## 从 JS 版本迁移

Rust 版本完全兼容原有 JS API。只需：

```bash
# 重新安装
npm install

# 构建 Rust 二进制文件
npm run build
```

所有命令行选项和交互模式保持不变。

## 性能对比

| 操作 | JS 版本 | Rust 版本 |
|------|---------|-----------|
| 扫描 100 个项目 | ~2s | ~0.3s |
| 内存占用 | ~50MB | ~5MB |
| 二进制大小 | N/A | ~2MB |

## 许可证

MIT
