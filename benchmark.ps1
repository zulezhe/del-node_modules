# PowerShell Benchmark Script for dnm Performance Testing
# Run this on Windows to compare JS vs Rust implementation

$ErrorActionPreference = "Stop"

Write-Host "================================================" -ForegroundColor Cyan
Write-Host "  dnm Performance Benchmark - JS vs Rust" -ForegroundColor Cyan
Write-Host "================================================" -ForegroundColor Cyan
Write-Host ""

# Configuration
$TestBase = "C:\dnm-benchmark"
$NodeExe = "node"
$RustExe = ".\target\release\dnm.exe"
$JSCli = ".\bin\cli.js"

# Test scenarios
$Scenarios = @(
    @{ Name = "Small"; Count = 5; ExpectedFiles = 20000 },
    @{ Name = "Medium"; Count = 20; ExpectedFiles = 100000 },
    @{ Name = "Large"; Count = 100; ExpectedFiles = 500000 }
)

function New-TestNodeModules {
    param(
        [string]$BasePath,
        [int]$PackageCount
    )
    
    Write-Host "Creating $PackageCount node_modules directories..." -ForegroundColor Yellow
    
    if (Test-Path $BasePath) {
        Remove-Item $BasePath -Recurse -Force -ErrorAction SilentlyContinue
    }
    New-Item -ItemType Directory -Path $BasePath -Force | Out-Null
    
    $Packages = @("lodash", "express", "react", "axios", "moment", "webpack", "babel-core", "typescript", "eslint", "prettier")
    
    for ($i = 1; $i -le $PackageCount; $i++) {
        $PackageName = $Packages[$i % $Packages.Length]
        $PackageDir = Join-Path $BasePath "package-$i"
        $NodeModulesDir = Join-Path $PackageDir "node_modules"
        
        # Create nested structure typical of node_modules
        $Depth = Get-Random -Minimum 3 -Maximum 8
        $CurrentPath = $NodeModulesDir
        
        for ($d = 0; $d -lt $Depth; $d++) {
            $SubDir = Join-Path $CurrentPath "@scope"
            $CurrentPath = Join-Path $SubDir "package-$d"
        }
        
        # Create some files
        New-Item -ItemType Directory -Path $CurrentPath -Force | Out-Null
        for ($f = 0; $f -lt 50; $f++) {
            "dummy content $f" | Out-File -FilePath (Join-Path $CurrentPath "file-$f.js") -Encoding UTF8
        }
        
        if ($i % 10 -eq 0) {
            Write-Host "  Created $i/$PackageCount packages" -ForegroundColor Gray
        }
    }
    
    Write-Host "✓ Test structure created at $BasePath" -ForegroundColor Green
    Write-Host ""
}

function Measure-DeletionTime {
    param(
        [string]$Command,
        [string]$Args,
        [string]$TestName
    )
    
    Write-Host "Testing: $TestName" -ForegroundColor Cyan
    
    # Warm up
    & $Command $Args | Out-Null
    
    # Create fresh test data
    $Scenario = $Scenarios[1] # Medium test
    New-TestNodeModules -BasePath "$TestBase\$($Scenario.Name)" -PackageCount $Scenario.Count
    
    # Measure deletion
    $Stopwatch = [System.Diagnostics.Stopwatch]::StartNew()
    
    & $Command $Args | Out-Null
    
    $Stopwatch.Stop()
    
    $Elapsed = $Stopwatch.Elapsed.TotalSeconds
    Write-Host "  Time: ${Elapsed}s" -ForegroundColor $(if ($Elapsed -lt 30) { "Green" } elseif ($Elapsed -lt 60) { "Yellow" } else { "Red" })
    Write-Host ""
    
    return $Elapsed
}

# Main execution
try {
    Write-Host "Prerequisites Check:" -ForegroundColor White
    Write-Host "  Node.js: $($NodeExe --version)" -ForegroundColor Green
    Write-Host "  Rust Binary: $(if (Test-Path $RustExe) { 'Found' } else { 'NOT FOUND' })" -ForegroundColor $(if (Test-Path $RustExe) { "Green" } else { "Red" })
    Write-Host ""
    
    if (-not (Test-Path $RustExe)) {
        Write-Host "Building Rust binary..." -ForegroundColor Yellow
        cargo build --release
        Write-Host ""
    }
    
    $Results = @{}
    
    foreach ($Scenario in $Scenarios) {
        Write-Host "================================================" -ForegroundColor Cyan
        Write-Host "  Scenario: $($Scenario.Name) ($($Scenario.Count) packages, ~$($Scenario.ExpectedFiles) files)" -ForegroundColor Cyan
        Write-Host "================================================" -ForegroundColor Cyan
        Write-Host ""
        
        $TestPath = "$TestBase\$($Scenario.Name)"
        
        # JS Version
        $JSTime = Measure-DeletionTime `
            -Command $NodeExe `
            -Args "$JSCli --no-safe $TestPath" `
            -TestName "JS Implementation"
        
        # Recreate for Rust test
        New-TestNodeModules -BasePath $TestPath -PackageCount $Scenario.Count
        
        # Rust Version
        $RustTime = Measure-DeletionTime `
            -Command $RustExe `
            -Args "--no-safe $TestPath" `
            -TestName "Rust Implementation"
        
        $Speedup = [math]::Round($JSTime / $RustTime, 2)
        $Results[$Scenario.Name] = @{ JS = $JSTime; Rust = $RustTime; Speedup = $Speedup }
    }
    
    # Summary
    Write-Host ""
    Write-Host "================================================" -ForegroundColor Cyan
    Write-Host "  RESULTS SUMMARY" -ForegroundColor Cyan
    Write-Host "================================================" -ForegroundColor Cyan
    Write-Host ""
    
    Write-Host ("{0,-15} {1,-12} {2,-12} {3,-10}" -f "Scenario", "JS (s)", "Rust (s)", "Speedup")
    Write-Host ("{0,-15} {1,-12} {2,-12} {3,-10}" -f "--------", "-----", "-------", "-------")
    
    foreach ($Scenario in $Scenarios) {
        $Result = $Results[$Scenario.Name]
        Write-Host ("{0,-15} {1,-12.2f} {2,-12.2f} {3,-10.2f}x" -f $Scenario.Name, $Result.JS, $Result.Rust, $Result.Speedup)
    }
    
    Write-Host ""
    Write-Host "Cleanup..." -ForegroundColor Yellow
    if (Test-Path $TestBase) {
        Remove-Item $TestBase -Recurse -Force -ErrorAction SilentlyContinue
    }
    Write-Host "✓ Done" -ForegroundColor Green
    
} catch {
    Write-Host "Error: $_" -ForegroundColor Red
    exit 1
}
