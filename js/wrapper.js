#!/usr/bin/env node

/**
 * JS wrapper for Rust dnm binary
 * Allows using dnm as a Node.js module or CLI tool
 */

const { execFileSync, execSync } = require('child_process');
const path = require('path');
const fs = require('fs');

// Resolve the Rust binary path
function getBinaryPath() {
  const platform = process.platform;
  const arch = process.arch;
  
  // Try to find pre-built binary in bin directory
  const binDir = path.join(__dirname, 'bin');
  const binaryName = platform === 'win32' ? 'dnm.exe' : 'dnm';
  const binaryPath = path.join(binDir, binaryName);
  
  if (fs.existsSync(binaryPath)) {
    return binaryPath;
  }
  
  // Fallback to target/release for development
  const projectRoot = path.join(__dirname, '..');
  const devBinaryPath = path.join(projectRoot, 'target', 'release', binaryName);
  
  if (fs.existsSync(devBinaryPath)) {
    return devBinaryPath;
  }
  
  throw new Error(
    'Rust binary not found. Please run `cargo build --release` or install the package properly.'
  );
}

/**
 * Find and delete node_modules directories
 * @param {string} targetPath - Target directory path
 * @param {Object} options - Options object
 * @param {boolean} options.showProgress - Show progress bar
 * @param {boolean} options.showSize - Show directory sizes
 * @param {string} options.logLevel - Log level (debug, info, warn, error)
 * @param {string} options.logFile - Log file path
 * @param {boolean} options.silent - Suppress console output
 * @param {Array<string>} options.ignore - Directories to ignore
 * @param {boolean} options.safeMode - Enable safe mode
 * @param {string} options.language - Language (zh-CN, en-US)
 * @returns {Object} Result with deleted paths and totals
 */
function findAndDeleteNodeModules(targetPath, options = {}) {
  const binaryPath = getBinaryPath();
  
  const args = [targetPath];
  
  if (options.showSize) args.push('--size');
  if (options.showProgress === false) args.push('--no-progress');
  if (options.logLevel) args.push('--log-level', options.logLevel);
  if (options.logFile) args.push('--log-file', options.logFile);
  if (options.silent) args.push('--silent');
  if (options.language) args.push('--lang', options.language);
  if (options.ignore) {
    options.ignore.forEach(dir => {
      args.push('--ignore', dir);
    });
  }
  if (options.safeMode === false) args.push('--no-safe');
  
  try {
    const output = execFileSync(binaryPath, args, {
      encoding: 'utf-8',
      stdio: options.silent ? 'pipe' : 'inherit'
    });
    
    // Parse output to extract results (if not silent)
    // For now, return basic success
    return {
      success: true,
      message: 'Cleanup completed'
    };
  } catch (error) {
    return {
      success: false,
      error: error.message
    };
  }
}

// CLI mode - pass through to Rust binary
if (require.main === module) {
  const binaryPath = getBinaryPath();
  
  try {
    execFileSync(binaryPath, process.argv.slice(2), {
      stdio: 'inherit'
    });
  } catch (error) {
    process.exit(error.status || 1);
  }
}

module.exports = { findAndDeleteNodeModules };
