# 常见问题

## 通用问题

### 删除的目录能恢复吗？

不能。删除是永久性的。建议始终使用安全模式（默认），并在确认前仔细检查列表。

### 安全模式和非安全模式有什么区别？

**安全模式**（默认）会先显示所有待删除目录列表，让你确认后再删除。**非安全模式**（`--no-safe`）会直接删除所有找到的 `node_modules` 目录。

### 如何忽略某些目录？

多次使用 `--ignore` 参数：

```bash
dnm --ignore backup --ignore old --ignore temp
```

### 支持哪些语言？

目前支持：
- 简体中文（zh-CN）— 默认
- English（en-US）

### 支持 Windows 吗？

支持。`dnm` 会检测平台并使用相应的删除命令（Windows 使用 `rd /s /q`，Unix 使用 `rm -rf`）。

## 故障排除

### 权限被拒绝

某些目录可能需要更高的权限。建议：

- Windows：以管理员身份运行终端
- macOS/Linux：使用 `sudo`（不推荐 — 建议修复权限）

### 扫描速度慢

使用 `--no-progress` 跳过进度条，或缩小扫描范围：

```bash
dnm /specific/project --no-progress
```

### 大小计算太慢

大小计算（`-s`）需要遍历每个文件。对于大型目录，可能会很慢。可以跳过：

```bash
dnm  # 不加 -s 参数
```

### 如何获取调试信息？

```bash
dnm -l debug -f debug.log
```

这会将详细的调试信息保存到 `debug.log`。
