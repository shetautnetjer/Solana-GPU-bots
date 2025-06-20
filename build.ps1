# Solana GPU Trading Bot Build Script
# This script automates the build process and handles common issues

param(
    [switch]$Clean,
    [switch]$Release,
    [switch]$SkipCuda,
    [switch]$Test,
    [switch]$Help
)

if ($Help) {
    Write-Host @"
Solana GPU Trading Bot Build Script

Usage: .\build.ps1 [options]

Options:
    -Clean      Clean build artifacts before building
    -Release    Build in release mode (default: debug)
    -SkipCuda   Skip CUDA compilation (for development)
    -Test       Run tests after building
    -Help       Show this help message

Examples:
    .\build.ps1                    # Debug build
    .\build.ps1 -Release           # Release build
    .\build.ps1 -Clean -Release    # Clean release build
    .\build.ps1 -SkipCuda -Test    # Build without CUDA and run tests
"@
    exit 0
}

Write-Host "🚀 Solana GPU Trading Bot Build Script" -ForegroundColor Green
Write-Host "=====================================" -ForegroundColor Green

# Check if we're in the right directory
if (-not (Test-Path "Cargo.toml")) {
    Write-Error "❌ Cargo.toml not found. Please run this script from the project root."
    exit 1
}

# Check Rust installation
Write-Host "🔍 Checking Rust installation..." -ForegroundColor Yellow
try {
    $rustVersion = rustc --version 2>$null
    Write-Host "✅ Rust found: $rustVersion" -ForegroundColor Green
} catch {
    Write-Error "❌ Rust not found. Please install Rust from https://rustup.rs/"
    exit 1
}

# Check CUDA installation
Write-Host "🔍 Checking CUDA installation..." -ForegroundColor Yellow
try {
    $cudaVersion = nvcc --version 2>$null
    if ($cudaVersion -match "release (\d+\.\d+)") {
        $version = $matches[1]
        Write-Host "✅ CUDA found: version $version" -ForegroundColor Green
    } else {
        Write-Host "⚠️  CUDA found but version unclear" -ForegroundColor Yellow
    }
} catch {
    Write-Warning "⚠️  CUDA not found. GPU features will be disabled."
    $SkipCuda = $true
}

# Check Visual Studio Build Tools (Windows)
if ($IsWindows -or $env:OS -eq "Windows_NT") {
    Write-Host "🔍 Checking Visual Studio Build Tools..." -ForegroundColor Yellow
    try {
        $clPath = where.exe cl.exe 2>$null
        if ($clPath) {
            Write-Host "✅ Visual Studio Build Tools found" -ForegroundColor Green
        } else {
            Write-Warning "⚠️  Visual Studio Build Tools not found. CUDA compilation may fail."
        }
    } catch {
        Write-Warning "⚠️  Could not check for Visual Studio Build Tools"
    }
}

# Set environment variables
if ($SkipCuda) {
    Write-Host "🔧 Setting SKIP_CUDA=1" -ForegroundColor Yellow
    $env:SKIP_CUDA = "1"
}

# Clean if requested
if ($Clean) {
    Write-Host "🧹 Cleaning build artifacts..." -ForegroundColor Yellow
    cargo clean
    if ($LASTEXITCODE -ne 0) {
        Write-Error "❌ Clean failed"
        exit 1
    }
    Write-Host "✅ Clean completed" -ForegroundColor Green
}

# Build the project
Write-Host "🔨 Building project..." -ForegroundColor Yellow
$buildArgs = @("build")
if ($Release) {
    $buildArgs += "--release"
    Write-Host "📦 Building in RELEASE mode" -ForegroundColor Cyan
} else {
    Write-Host "🐛 Building in DEBUG mode" -ForegroundColor Cyan
}

cargo @buildArgs
if ($LASTEXITCODE -ne 0) {
    Write-Error "❌ Build failed"
    exit 1
}
Write-Host "✅ Build completed successfully!" -ForegroundColor Green

# Run tests if requested
if ($Test) {
    Write-Host "🧪 Running tests..." -ForegroundColor Yellow
    $testArgs = @("test")
    if ($Release) {
        $testArgs += "--release"
    }
    
    cargo @testArgs
    if ($LASTEXITCODE -ne 0) {
        Write-Error "❌ Tests failed"
        exit 1
    }
    Write-Host "✅ All tests passed!" -ForegroundColor Green
}

# Show build summary
Write-Host "`n📊 Build Summary" -ForegroundColor Green
Write-Host "===============" -ForegroundColor Green
Write-Host "Build mode: $(if ($Release) { 'Release' } else { 'Debug' })" -ForegroundColor White
Write-Host "CUDA: $(if ($SkipCuda) { 'Disabled' } else { 'Enabled' })" -ForegroundColor White
Write-Host "Tests: $(if ($Test) { 'Run' } else { 'Skipped' })" -ForegroundColor White

Write-Host "`n🎯 Next Steps:" -ForegroundColor Cyan
Write-Host "1. Test GPU functionality: cargo run -p pool_indexer -- gpu-test" -ForegroundColor White
Write-Host "2. Monitor pools: cargo run -p pool_indexer -- monitor --pair SOL/USDC --gpu" -ForegroundColor White
Write-Host "3. Get quotes: cargo run -p pool_indexer -- quote --pair SOL/USDC" -ForegroundColor White

Write-Host "`n✅ Build script completed successfully!" -ForegroundColor Green 