# 命令行选项

## 用法

```bash
dnm [目录] [选项]
```

## 选项说明

### `-h, --help`
显示帮助信息并退出。

```bash
dnm --help
```

### `-i, --interactive`
以交互模式运行，提供引导式向导。

```bash
dnm -i
```

详见 [交互模式](/zh-CN/interactive)。

### `-s, --size`
计算并显示每个 `node_modules` 目录的大小。

```bash
dnm -s
```

> 注意：大小计算会增加扫描阶段的开销。

### `--no-progress`
禁用删除过程中的进度条。

```bash
dnm --no-progress
```

### `-l, --log-level <级别>`
设置日志详细程度。可用级别：`debug`、`info`、`warn`、`error`。

```bash
dnm -l debug
```

### `-f, --log-file <路径>`
将日志输出写入文件。

```bash
dnm -f cleanup.log
```

### `--silent`
静默模式，不输出到控制台。与 `-f` 配合使用效果更佳。

```bash
dnm --silent -f cleanup.log
```

### `--lang, --language <语言>`
设置界面语言。

```bash
# 中文（默认）
dnm --lang zh-CN

# 英文
dnm --lang en-US
```

### `--ignore <目录>`
扫描时忽略指定目录。可以多次使用。

```bash
dnm --ignore backup --ignore old-projects
```

### `--no-safe`
禁用安全模式 — 直接删除所有找到的 `node_modules`，无需确认。

```bash
dnm --no-safe
```

> ⚠️ **警告**：谨慎使用 `--no-safe`。删除的目录无法恢复。

## 组合使用

```bash
# 英文 + 显示大小 + 日志到文件
dnm --lang en-US -s -f cleanup.log

# 调试模式 + 忽略目录
dnm -l debug --ignore backup --ignore temp

# 静默模式，仅文件日志
dnm --silent -f cleanup.log
```
