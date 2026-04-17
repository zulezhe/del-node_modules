# Installation

## Global Install (Recommended)

```bash
npm install -g dnm
```

After installation, the `dnm` command is available everywhere.

## Using npx (No Install)

```bash
npx dnm /path/to/directory
```

## From Source

### Option 1: Build from Rust source

```bash
git clone https://github.com/yourusername/dnm.git
cd dnm
cargo build --release
```

The binary will be at `target/release/dnm`.

### Option 2: Build with NPM

```bash
git clone https://github.com/yourusername/dnm.git
cd dnm
npm install
```

This will automatically build the Rust binary during installation.

### Option 3: Link to global NPM bin

```bash
cd dnm
npm link
```

This links `dnm` to your global npm bin so you can run it from anywhere.

## Verify Installation

```bash
dnm --help
```

You should see the help message with all available options.

## Requirements

- **For NPM installation**: Node.js >= 14.0.0, npm or pnpm
- **For building from source**: Rust 1.70+ (edition 2021)
- **Supported platforms**: Windows, macOS, Linux

## Pre-built Binaries

Pre-built binaries are available for:
- Linux (x86_64)
- macOS (x86_64, arm64)
- Windows (x86_64)

The Node.js wrapper will automatically detect and use the appropriate binary.
