# 快速上手

## 基本用法

```bash
# 删除当前目录下所有 node_modules（安全模式）
dnm

# 指定目标目录
dnm /path/to/projects
```

## 首次运行

首次运行 `dnm` 时，它会：

1. **扫描** 目标目录，递归查找 `node_modules`
2. **显示** 找到的所有目录的编号列表
3. **提示** 你选择要删除哪些
4. **删除** 已选择的目录，并显示进度条

## 示例会话

```
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

## 常用命令

| 命令 | 说明 |
|------|------|
| `dnm` | 扫描当前目录 |
| `dnm ~/projects` | 扫描指定目录 |
| `dnm -s` | 显示目录大小 |
| `dnm --lang en-US` | 使用英文界面 |
| `dnm --no-safe` | 跳过确认（谨慎使用） |
| `dnm -i` | 交互模式 |

## 下一步

- [命令行选项](/zh-CN/options) — 所有可用参数
- [安全模式](/zh-CN/safe-mode) — 安全模式详解
- [交互模式](/zh-CN/interactive) — 引导向导模式
