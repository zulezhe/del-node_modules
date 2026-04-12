#!/usr/bin/env node

/**
 * JS CLI wrapper that delegates to the Rust binary
 */

const { execFileSync } = require('child_process');
const path = require('path');
const fs = require('fs');

function getBinaryPath() {
  const platform = process.platform;
  const binaryName = platform === 'win32' ? 'dnm.exe' : 'dnm';
  
  // Try bin directory first
  const binPath = path.join(__dirname, 'bin', binaryName);
  if (fs.existsSync(binPath)) {
    return binPath;
  }
  
  // Try target/release for development
  const devPath = path.join(__dirname, '..', 'target', 'release', binaryName);
  if (fs.existsSync(devPath)) {
    return devPath;
  }
  
  throw new Error('dnm binary not found. Run `cargo build --release` first.');
}

const binaryPath = getBinaryPath();

try {
  execFileSync(binaryPath, process.argv.slice(2), {
    stdio: 'inherit'
  });
} catch (error) {
  process.exit(error.status || 1);
}
