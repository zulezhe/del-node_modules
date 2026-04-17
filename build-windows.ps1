# Build script for Windows platform
# Run this on Windows to build the Rust binary and prepare for release

$ErrorActionPreference = "Stop"

Write-Host "================================================" -ForegroundColor Cyan
Write-Host "  dnm Windows Build Script" -ForegroundColor Cyan
Write-Host "================================================" -ForegroundColor Cyan
Write-Host ""

# Check prerequisites
Write-Host "Checking prerequisites..." -ForegroundColor Yellow

# Check Rust
$rustVersion = rustc --version 2>$null
if ($rustVersion) {
    Write-Host "  ✓ Rust: $rustVersion" -ForegroundColor Green
} else {
    Write-Host "  ✗ Rust not found. Please install Rust from https://rustup.rs/" -ForegroundColor Red
    exit 1
}

# Check Node.js
$nodeVersion = node --version 2>$null
if ($nodeVersion) {
    Write-Host "  ✓ Node.js: $nodeVersion" -ForegroundColor Green
} else {
    Write-Host "  ✗ Node.js not found. Please install Node.js from https://nodejs.org/" -ForegroundColor Red
    exit 1
}

Write-Host ""

# Clean previous builds
Write-Host "Cleaning previous builds..." -ForegroundColor Yellow
if (Test-Path "target\release\dnm.exe") {
    Remove-Item "target\release\dnm.exe" -Force -ErrorAction SilentlyContinue
}
if (Test-Path "bin\dnm.exe") {
    Remove-Item "bin\dnm.exe" -Force -ErrorAction SilentlyContinue
}
Write-Host "  ✓ Cleaned" -ForegroundColor Green
Write-Host ""

# Build Rust binary
Write-Host "Building Rust binary (release mode)..." -ForegroundColor Yellow
Write-Host "  This may take a few minutes on first build..." -ForegroundColor Gray

$stopwatch = [System.Diagnostics.Stopwatch]::StartNew()
cargo build --release --target x86_64-pc-windows-msvc

$stopwatch.Stop()
$buildTime = [math]::Round($stopwatch.Elapsed.TotalSeconds, 2)

if (Test-Path "target\x86_64-pc-windows-msvc\release\dnm.exe") {
    Write-Host "  ✓ Build completed in ${buildTime}s" -ForegroundColor Green
} else {
    Write-Host "  ✗ Build failed!" -ForegroundColor Red
    exit 1
}
Write-Host ""

# Copy binary to bin directory
Write-Host "Copying binary to bin/ directory..." -ForegroundColor Yellow
if (-not (Test-Path "bin")) {
    New-Item -ItemType Directory -Path "bin" -Force | Out-Null
}
Copy-Item "target\x86_64-pc-windows-msvc\release\dnm.exe" "bin\dnm.exe" -Force
Write-Host "  ✓ Copied to bin/dnm.exe" -ForegroundColor Green
Write-Host ""

# Get binary info
$binaryPath = "bin\dnm.exe"
$binaryInfo = Get-Item $binaryPath
$binarySize = [math]::Round($binaryInfo.Length / 1MB, 2)

Write-Host "Build Summary:" -ForegroundColor Cyan
Write-Host "  Binary: bin/dnm.exe" -ForegroundColor White
Write-Host "  Size: ${binarySize} MB" -ForegroundColor White
Write-Host "  Location: $([System.IO.Path]::GetFullPath($binaryPath))" -ForegroundColor White
Write-Host ""

# Test the binary
Write-Host "Testing binary..." -ForegroundColor Yellow
Write-Host ""

& $binaryPath --version

Write-Host ""
Write-Host "================================================" -ForegroundColor Green
Write-Host "  Build Successful!" -ForegroundColor Green
Write-Host "================================================" -ForegroundColor Green
Write-Host ""
Write-Host "Usage:" -ForegroundColor Yellow
Write-Host "  .\bin\dnm.exe --help           # Show help" -ForegroundColor Gray
Write-Host "  .\bin\dnm.exe /path/to/dir     # Delete node_modules" -ForegroundColor Gray
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Yellow
Write-Host "  - Test the binary: .\bin\dnm.exe" -ForegroundColor Gray
Write-Host "  - Upload to GitHub Releases" -ForegroundColor Gray
Write-Host "  - Publish to NPM (includes Windows binary)" -ForegroundColor Gray
Write-Host ""
