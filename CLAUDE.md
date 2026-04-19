# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**dnm** (Delete Node Modules) — a cross-platform CLI tool that recursively finds and deletes `node_modules` directories. Originally written in Node.js, now rewritten in Rust for 6-7x faster scanning and 10x lower memory usage. Published as an npm package (`dnm`) that wraps the Rust binary.

The project maintains dual implementations: a legacy JS version (`index.js`) and the current Rust version (`src/`), with the JS wrapper layer (`js/wrapper.js`, `js/cli.js`) delegating to the compiled Rust binary.

## Build & Development Commands

```bash
# Rust release build (production binary)
cargo build --release

# Rust debug build
cargo build

# Run Rust binary directly
cargo run -- [options]

# Run Rust test suite (uses temp dirs, auto-cleanup)
cargo test

# Run Rust tests with verbose output
cargo test -- --nocapture

# Build + copy binary to bin/ for npm usage
npm run build

# Run JS test suite (requires compiled Rust binary)
npm test

# Run the tool via npm
npm start -- [options]
```

## Architecture

### Dual-Language Structure

The Rust core is the authoritative implementation. The JS layer exists solely for npm distribution and backwards-compatible `require('dnm')` usage.

**Rust core** (`src/`):
- `main.rs` — thin entry point, calls `run_cli()` from lib
- `lib.rs` — orchestrates the full pipeline: scan → safe-mode selection → delete → summary. Exposes `find_and_delete_node_modules()` and `run_cli()` as the public API
- `cli.rs` — clap-based argument parsing (`CliArgs` struct)
- `scanner.rs` — recursive directory walker that stops at `node_modules` (doesn't traverse into them)
- `deleter.rs` — deletes directories with platform-native commands (`rd /s /q` on Windows, `rm -rf` on Unix) and falls back to `fs::remove_dir_all`
- `interactive.rs` — interactive mode prompts and safe-mode selection list with range/individual parsing
- `i18n.rs` — inline `HashMap`-based translations (zh-CN default, en-US). Supports `{param}` template substitution
- `logger.rs` — multi-level logger (debug/info/warn/error/success) with optional file output
- `utils.rs` — `format_bytes()`, `get_directory_size()`, `should_ignore()`
- `tests.rs` — integration tests using temp directories with PID-based naming

**JS wrapper** (`js/`):
- `cli.js` — npm bin entry, locates and execs the Rust binary with `stdio: 'inherit'`
- `wrapper.js` — provides `findAndDeleteNodeModules()` for programmatic use, translates JS options to CLI args

**Legacy JS** (still present but not the active code path):
- `index.js` — original pure-JS implementation, used by `test.js`
- `lib/i18n.js`, `lib/logger.js` — supporting modules for the legacy version

### Key Design Decisions

- **Safe mode is default**: scanning always precedes deletion; user must explicitly opt into `--no-safe`
- **Recursion stops at `node_modules`**: the scanner adds the path to results and does NOT descend into `node_modules` contents
- **Two-phase deletion**: tries native OS command first, falls back to Rust's `fs::remove_dir_all`
- **Binary lookup order**: `bin/dnm(.exe)` → `target/release/dnm(.exe)` — the wrapper searches these in order

### Rust Dependencies

- `clap` (derive) — CLI args
- `colored` — terminal colors
- `indicatif` — progress bar
- `dialoguer` — interactive prompts (declared but interactive.rs uses manual stdin reading)
- `chrono` — log timestamps
- `anyhow` / `thiserror` — error handling (declared but errors use `Result<_, String>`)

## CI/CD

GitHub Actions (`.github/workflows/ci-cd.yml`) runs on push/PR to `main`/`develop`:
- Tests on Ubuntu, Windows, macOS
- On `main` push: cross-compile binaries (Linux x86_64, Windows x86_64, macOS x86_64 + aarch64), create GitHub Release, publish to npm

Pre-commit hook (`.husky/pre-commit`): runs `npm test` before allowing commits.

## Language

This project uses bilingual (Chinese/English) documentation and UI. Default language is `zh-CN`. Keep this convention when modifying i18n strings — always add both translations in `src/i18n.rs`.
