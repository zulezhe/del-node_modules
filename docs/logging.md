# Logging

## Log Levels

`dnm` supports four log levels, from most to least verbose:

| Level | Description |
|-------|-------------|
| `debug` | Detailed diagnostic information |
| `info` | General operational messages (default) |
| `warn` | Warning messages for potential issues |
| `error` | Error messages only |

## Console Logging

By default, logs are printed to the console with colors:

```bash
# Default (info level)
dnm

# Debug level — see everything
dnm -l debug

# Only errors
dnm -l error
```

## File Logging

Save logs to a file for later review:

```bash
# Save to file
dnm -f cleanup.log

# Debug + file
dnm -l debug -f debug.log

# Silent console + file logging
dnm --silent -f cleanup.log
```

## Log Format

Each log line follows this format:

```
[2025-04-09T12:00:00.000Z] [INFO] Found node_modules: /path/to/dir
```

## Use Cases

### Debugging Issues

```bash
dnm -l debug -f debug.log --no-progress
```

### CI/CD Integration

```bash
dnm --silent -f cleanup.log
```

### Audit Trail

```bash
dnm -l info -f "cleanup-$(date +%Y%m%d).log"
```
