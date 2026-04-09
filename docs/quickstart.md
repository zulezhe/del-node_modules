# Quick Start

## Basic Usage

```bash
# Delete all node_modules in current directory (safe mode)
dnm

# Specify a target directory
dnm /path/to/projects
```

## First Run

When you run `dnm` for the first time, it will:

1. **Scan** the target directory recursively for `node_modules`
2. **Display** a numbered list of all found directories
3. **Prompt** you to select which ones to delete
4. **Delete** the selected directories with a progress bar

## Example Session

```
$ dnm

╔════════════════════════════════════════════════╗
║     🗑️  dnm - Cleanup Tool                     ║
╚════════════════════════════════════════════════╝

🔍 Target directory: /home/user/projects

Scanning /home/user/projects for node_modules directories...
Found 5 node_modules directories

═══════════════════════════════════════════════════
  Safe Mode - Found node_modules Directories
═══════════════════════════════════════════════════

[1] /home/user/projects/app1/node_modules (150.5 MB)
[2] /home/user/projects/app2/node_modules (89.2 MB)
[3] /home/user/projects/lib/node_modules (45.8 MB)
[4] /home/user/projects/demo/node_modules (12.3 MB)
[5] /home/user/projects/old/node_modules (200.1 MB)

Enter numbers to delete (e.g., 1-5 or 1,3,5), leave empty for all, or q to cancel: 1-3

Selected 3 directories for deletion
```

## Common Commands

| Command | Description |
|---------|-------------|
| `dnm` | Scan current directory |
| `dnm ~/projects` | Scan specific directory |
| `dnm -s` | Show directory sizes |
| `dnm --lang en-US` | Use English interface |
| `dnm --no-safe` | Skip confirmation (use with caution) |
| `dnm -i` | Interactive mode |

## Next Steps

- [Command Line Options](/options) — All available flags
- [Safe Mode](/safe-mode) — Detailed safe mode explanation
- [Interactive Mode](/interactive) — Guided wizard mode
