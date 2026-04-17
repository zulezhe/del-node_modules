# Command Line Options

## Usage

```bash
dnm [directory] [options]
```

## Options

### `-h, --help`
Show the help message and exit.

```bash
dnm --help
```

### `-i, --interactive`
Run in interactive mode with a guided wizard.

```bash
dnm -i
```

See [Interactive Mode](/interactive) for details.

### `-s, --size`
Calculate and display the size of each `node_modules` directory.

```bash
dnm -s
```

> Note: Size calculation adds some overhead to the scanning phase.

### `--no-progress`
Disable the progress bar during deletion.

```bash
dnm --no-progress
```

### `-l, --log-level <level>`
Set the log verbosity level. Available levels: `debug`, `info`, `warn`, `error`.

```bash
dnm -l debug
```

### `-f, --log-file <path>`
Write log output to a file.

```bash
dnm -f cleanup.log
```

### `--silent`
Suppress all console output. Useful when combined with `-f`.

```bash
dnm --silent -f cleanup.log
```

### `--lang, --language <lang>`
Set the interface language.

```bash
# Chinese (default)
dnm --lang zh-CN

# English
dnm --lang en-US
```

### `--ignore <dir>`
Ignore specific directories during scanning. Can be used multiple times.

```bash
dnm --ignore backup --ignore old-projects
```

### `--no-safe`
Disable safe mode — delete all found `node_modules` without confirmation.

```bash
dnm --no-safe
```

> ⚠️ **Warning**: Use `--no-safe` with caution. Deleted directories cannot be recovered.

## Combining Options

```bash
# English + sizes + log to file
dnm --lang en-US -s -f cleanup.log

# Debug mode + ignore directories
dnm -l debug --ignore backup --ignore temp

# Silent mode with file logging only
dnm --silent -f cleanup.log
```

## Architecture Notes

The CLI interface is powered by:
- **Rust core**: All deletion and scanning logic
- **Node.js wrapper**: Provides seamless NPM integration

Both interfaces support the same options and maintain compatibility.
