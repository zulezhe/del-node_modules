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

```bash
git clone https://github.com/yourusername/dnm.git
cd dnm
npm install
npm link
```

This links `dnm` to your global npm bin so you can run it from anywhere.

## Verify Installation

```bash
dnm --help
```

You should see the help message with all available options.

## Requirements

- Node.js >= 16.0.0
- npm or pnpm
