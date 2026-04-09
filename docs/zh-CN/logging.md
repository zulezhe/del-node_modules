# 日志系统

## 日志级别

`dnm` 支持四个日志级别，从详细到简洁：

| 级别 | 说明 |
|------|------|
| `debug` | 详细的诊断信息 |
| `info` | 常规操作消息（默认） |
| `warn` | 潜在问题的警告 |
| `error` | 仅错误消息 |

## 控制台日志

默认情况下，日志会以彩色输出到控制台：

```bash
# 默认（info 级别）
dnm

# Debug 级别 — 查看所有信息
dnm -l debug

# 仅错误
dnm -l error
```

## 文件日志

将日志保存到文件以便后续查看：

```bash
# 保存到文件
dnm -f cleanup.log

# Debug + 文件
dnm -l debug -f debug.log

# 静默控制台 + 文件日志
dnm --silent -f cleanup.log
```

## 日志格式

每条日志遵循以下格式：

```
[2025-04-09T12:00:00.000Z] [INFO] 发现 node_modules: /path/to/dir
```

## 使用场景

### 调试问题

```bash
dnm -l debug -f debug.log --no-progress
```

### CI/CD 集成

```bash
dnm --silent -f cleanup.log
```

### 审计追踪

```bash
dnm -l info -f "cleanup-$(date +%Y%m%d).log"
```
