# dnm (Delete Node Modules)

> A powerful Node.js CLI tool for recursively finding and deleting all `node_modules` directories.

[![CI/CD](https://github.com/yourusername/dnm/workflows/CI/CD%20Pipeline/badge.svg)](https://github.com/yourusername/dnm/actions)
[![npm version](https://badge.fury.io/js/dnm.svg)](https://www.npmjs.com/package/dnm)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Why dnm?

When working on multiple Node.js projects, `node_modules` directories can consume gigabytes of disk space. `dnm` helps you reclaim that space quickly and safely.

## Key Features

- **Recursive Scanning** — Finds all `node_modules` in nested directories
- **Safe Mode (default)** — Shows a list before deleting anything
- **Interactive Mode** — Guided wizard for first-time users
- **Progress Bar** — Real-time visual feedback during deletion
- **Colored Output** — Beautiful terminal output with chalk
- **Multi-language** — Supports Chinese (zh-CN) and English (en-US)
- **Directory Sizes** — Optional size calculation and display
- **Ignore Patterns** — Skip specific directories
- **Logging** — Write detailed logs to file with configurable levels
- **Cross-Platform** — Works on Windows, macOS, and Linux

## Quick Example

```bash
# Scan and delete node_modules in current directory
dnm

# With directory sizes shown
dnm -s

# English interface
dnm --lang en-US
```

## Safety First

By default, `dnm` runs in **safe mode** — it scans and displays all found `node_modules` directories, then asks you to confirm before deleting. You can select specific directories by number (e.g., `1-3,7`) or press Enter to delete all.

## Installation

```bash
npm install -g dnm
```

See the [Installation Guide](/install) for more details.
