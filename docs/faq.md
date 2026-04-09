# FAQ

## General

### Is there a way to recover deleted directories?

No. Deletion is permanent. Always use safe mode (default) and review the list carefully before confirming.

### What's the difference between safe mode and non-safe mode?

**Safe mode** (default) shows a list of found `node_modules` directories and asks you to confirm. **Non-safe mode** (`--no-safe`) deletes everything immediately.

### How do I ignore certain directories?

Use `--ignore` multiple times:

```bash
dnm --ignore backup --ignore old --ignore temp
```

### What languages are supported?

Currently:
- 简体中文 (zh-CN) — default
- English (en-US)

### Does it work on Windows?

Yes. `dnm` detects the platform and uses the appropriate deletion command (`rd /s /q` on Windows, `rm -rf` on Unix).

## Troubleshooting

### Permission denied errors

Some directories may require elevated permissions. Try:

- On Windows: Run terminal as Administrator
- On macOS/Linux: Use `sudo` (not recommended — prefer fixing permissions)

### Scanning is slow

Use `--no-progress` to skip the progress bar, or narrow the scan path:

```bash
dnm /specific/project --no-progress
```

### Size calculation takes too long

Size calculation (`-s`) walks every file. For large directories, this can be slow. You can skip it:

```bash
dnm  # without -s flag
```

### How to get debug info?

```bash
dnm -l debug -f debug.log
```

This saves detailed debug information to `debug.log`.
