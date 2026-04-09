# Safe Mode

## Overview

Safe mode is **enabled by default**. It ensures you always see what will be deleted before any action is taken.

## How It Works

1. `dnm` scans the target directory for all `node_modules`
2. It displays a numbered list of found directories
3. You choose which ones to delete
4. Only the selected directories are removed

## Selection Syntax

| Input | Action |
|-------|--------|
| *(press Enter)* | Delete all |
| `3` | Delete only #3 |
| `1-5` | Delete #1 through #5 |
| `1,3,5` | Delete #1, #3, #5 |
| `1-3,7,9-12` | Combined selection |
| `q` or `Q` | Cancel operation |

## Example

```bash
$ dnm -s

═══════════════════════════════════════════════════
  Safe Mode - Found node_modules Directories
═══════════════════════════════════════════════════

[1] /projects/app1/node_modules (150.5 MB)
[2] /projects/app2/node_modules (89.2 MB)
[3] /projects/lib/node_modules (45.8 MB)
[4] /projects/demo/node_modules (12.3 MB)

Enter numbers to delete: 1,3

Selected 2 directories for deletion
```

## Disable Safe Mode

If you want to skip the confirmation step:

```bash
dnm --no-safe
```

> ⚠️ This will immediately delete all found `node_modules` directories. Make sure you have backups of important projects.
