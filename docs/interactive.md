# Interactive Mode

Interactive mode provides a step-by-step wizard for configuring and running `dnm`. It's recommended for first-time users.

## Start Interactive Mode

```bash
dnm -i
# or
dnm --interactive
```

## Wizard Steps

### 1. Language Selection

```
? Select language / 选择语言:
  ❯ 简体中文 (Simplified Chinese)
    English (US)
```

### 2. Target Directory

```
? Enter the directory path to scan: /home/user/projects
```

### 3. Ignore Directories

```
? Enter directories to ignore (comma-separated, leave empty for none):
```

### 4. Safe Mode

```
? Enable safe mode (show list and confirm before deleting)? (Y/n)
```

### 5. Show Sizes

```
? Calculate and show directory sizes? (y/N)
```

### 6. Progress Bar

```
? Show progress bar? (Y/n)
```

### 7. Log Level

```
? Select log level:
  debug
  ❯ info
    warn
    error
```

### 8. Log File

```
? Save logs to a file? (y/N)
```

If yes:
```
? Enter log file path: cleanup.log
```

### 9. Confirmation

```
⚠️  This will delete all node_modules directories. Continue? (y/N)
```

## When to Use

- First time using `dnm`
- Unsure about command-line options
- Want a guided experience
