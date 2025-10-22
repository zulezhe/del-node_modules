# AGENTS.md

This file provides guidance to Qoder (qoder.com) when working with code in this repository.

## Project Overview

This is a Node.js utility for recursively deleting all `node_modules` directories under a specified path. The tool stops recursion when it encounters a `node_modules` directory, deletes it entirely, and continues scanning other directories at the same level.

## Commands

- **Run the tool**: `node index.js` (when used as a module)
- **CLI usage**: `node bin/cli.js <target-directory>`
- **Run tests**: `npm test` - Creates a temporary test structure, runs deletion, verifies results, and cleans up

## Architecture

### Core Module (`index.js`)
- Exports `findAndDeleteNodeModules(targetPath)` function
- Uses recursive directory scanning with immediate deletion upon finding `node_modules`
- Stops recursion at `node_modules` directories to avoid scanning their contents
- Uses `fs.rmSync()` with `recursive: true` and `force: true` for deletion
- Includes error handling for inaccessible files/directories

### CLI Entry Point (`bin/cli.js`)
- Command-line wrapper for the core module
- Accepts a single argument: target directory path
- Resolves relative paths to absolute paths

### Testing (`test.js`)
- Creates a temporary nested directory structure with multiple `node_modules` directories
- Verifies successful deletion
- Cleans up after itself

## Key Implementation Details

- Recursion stops at `node_modules` directories - does not traverse into them before deletion
- All paths are handled with Node.js `path` module for cross-platform compatibility
- Uses synchronous fs operations for simplicity
- Provides console feedback during scanning and deletion
- Graceful error handling with warnings for inaccessible paths
