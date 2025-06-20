# Solana GPU Trading Bot - OpenSSL Setup Script
# This script automates the installation and configuration of OpenSSL on Windows
# Required for building Solana SDK dependencies

param(
    [switch]$SkipDownload,
    [switch]$Help
)

if ($Help) {
    Write-Host @"
OpenSSL Setup Script for Solana GPU Trading Bot

This script will:
1. Download OpenSSL for Windows (if not already installed)
2. Install it to the default location
3. Set the required environment variables
4. Verify the installation

Usage: .\setup-openssl.ps1 [options]

Options:
    -SkipDownload   Skip downloading if OpenSSL is already installed
    -Help           Show this help message

Requirements:
    - Administrator privileges (for setting system environment variables)
    - Internet connection (for downloading OpenSSL)
"@
    exit 0
}

Write-Host "OpenSSL Setup for Solana GPU Trading Bot" -ForegroundColor Green
Write-Host "==========================================" -ForegroundColor Green

# Check if running as Administrator
$currentPrincipal = New-Object Security.Principal.WindowsPrincipal([Security.Principal.WindowsIdentity]::GetCurrent())
$isAdmin = $currentPrincipal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)

if (-not $isAdmin) {
    Write-Error "This script requires Administrator privileges to set system environment variables."
    Write-Host "Please run PowerShell as Administrator and try again." -ForegroundColor Yellow
    exit 1
}

# Define OpenSSL paths
$opensslDir = "C:\Program Files\OpenSSL-Win64"
$opensslBin = "$opensslDir\bin"
$opensslLib = "$opensslDir\lib\VC\x64\MD"
$opensslInclude = "$opensslDir\include"

# Check if OpenSSL is already installed
Write-Host "Checking for existing OpenSSL installation..." -ForegroundColor Yellow
$opensslExists = Test-Path "$opensslBin\openssl.exe"

if ($opensslExists -and -not $SkipDownload) {
    Write-Host "OpenSSL is already installed at $opensslDir" -ForegroundColor Green
    
    # Check version
    try {
        $version = & "$opensslBin\openssl.exe" version 2>$null
        Write-Host "   Version: $version" -ForegroundColor Cyan
    } catch {
        Write-Warning "Could not determine OpenSSL version"
    }
    
    $response = Read-Host "Do you want to skip reinstallation? (Y/n)"
    if ($response -ne 'n' -and $response -ne 'N') {
        $SkipDownload = $true
    }
}

# Download and install OpenSSL if needed
if (-not $opensslExists -or -not $SkipDownload) {
    Write-Host "Downloading OpenSSL for Windows..." -ForegroundColor Yellow
    
    # Create temp directory
    $tempDir = "$env:TEMP\openssl_setup"
    New-Item -ItemType Directory -Force -Path $tempDir | Out-Null
    
    # Download URL for OpenSSL 3.x EXE installer
    $downloadUrl = "https://slproweb.com/download/Win64OpenSSL-3_5_0.exe"
    
    $installerPath = "$tempDir\openssl_installer.exe"
    
    try {
        Write-Host "   Downloading from: $downloadUrl" -ForegroundColor Gray
        Invoke-WebRequest -Uri $downloadUrl -OutFile $installerPath -UseBasicParsing
    } catch {
        Write-Error "Failed to download OpenSSL installer from $downloadUrl"
        Write-Host @"

Please download the Win64 OpenSSL v3.x.x EXE installer manually from:
https://slproweb.com/products/Win32OpenSSL.html

Place the downloaded .exe file in this directory and run the script again.
The script will detect the local installer and proceed.
"@ -ForegroundColor Yellow
        exit 1
    }
    
    Write-Host "Installing OpenSSL..." -ForegroundColor Yellow
    Write-Host "   This may take a few minutes..." -ForegroundColor Gray
    
    # Install OpenSSL silently using Inno Setup flags
    $arguments = "/VERYSILENT /SUPPRESSMSGBOXES /NORESTART /DIR=`"$opensslDir`""
    $process = Start-Process -FilePath $installerPath -ArgumentList $arguments -Wait -PassThru
    
    if ($process.ExitCode -ne 0) {
        Write-Error "OpenSSL installation failed with exit code: $($process.ExitCode)"
        exit 1
    }
    
    # Clean up
    Remove-Item -Path $tempDir -Recurse -Force -ErrorAction SilentlyContinue
    
    Write-Host "OpenSSL installed successfully!" -ForegroundColor Green
}

# Set environment variables
Write-Host "`nSetting environment variables..." -ForegroundColor Yellow

# Function to set system environment variable
function Set-SystemEnvironmentVariable {
    param(
        [string]$Name,
        [string]$Value
    )
    
    try {
        [Environment]::SetEnvironmentVariable($Name, $Value, [EnvironmentVariableTarget]::Machine)
        Write-Host "   Set $Name = $Value" -ForegroundColor Green
    } catch {
        Write-Error "   Failed to set $Name"
        throw
    }
}

try {
    Set-SystemEnvironmentVariable -Name "OPENSSL_DIR" -Value $opensslDir
    Set-SystemEnvironmentVariable -Name "OPENSSL_LIB_DIR" -Value $opensslLib
    Set-SystemEnvironmentVariable -Name "OPENSSL_INCLUDE_DIR" -Value $opensslInclude
} catch {
    Write-Error "Failed to set environment variables"
    exit 1
}

# Add OpenSSL to PATH if not already there
Write-Host "`nUpdating PATH..." -ForegroundColor Yellow
$currentPath = [Environment]::GetEnvironmentVariable("PATH", [EnvironmentVariableTarget]::Machine)

if ($currentPath -notlike "*$opensslBin*") {
    $newPath = "$currentPath;$opensslBin"
    [Environment]::SetEnvironmentVariable("PATH", $newPath, [EnvironmentVariableTarget]::Machine)
    Write-Host "   Added OpenSSL to system PATH" -ForegroundColor Green
} else {
    Write-Host "   OpenSSL already in PATH" -ForegroundColor Green
}

# Set environment variables for current session too
Write-Host "`nSetting variables for current session..." -ForegroundColor Yellow
$env:OPENSSL_DIR = $opensslDir
$env:OPENSSL_LIB_DIR = $opensslLib
$env:OPENSSL_INCLUDE_DIR = $opensslInclude
$env:PATH = "$env:PATH;$opensslBin"

# Verify installation
Write-Host "`nVerifying installation..." -ForegroundColor Yellow

# Check if OpenSSL executable exists
if (Test-Path "$opensslBin\openssl.exe") {
    Write-Host "   OpenSSL executable found" -ForegroundColor Green
    
    # Test OpenSSL command
    try {
        $version = & "$opensslBin\openssl.exe" version 2>&1
        Write-Host "   OpenSSL version: $version" -ForegroundColor Green
    } catch {
        Write-Warning "   Could not run OpenSSL command"
    }
} else {
    Write-Error "   OpenSSL executable not found at expected location"
}

# Check if library files exist
$libFiles = @(
    "$opensslLib\libcrypto.lib",
    "$opensslLib\libssl.lib"
)

$allLibsFound = $true
foreach ($libFile in $libFiles) {
    if (Test-Path $libFile) {
        Write-Host "   Found: $(Split-Path -Leaf $libFile)" -ForegroundColor Green
    } else {
        Write-Warning "   Missing: $libFile"
        $allLibsFound = $false
    }
}

# Check include directory
if (Test-Path "$opensslInclude\openssl") {
    Write-Host "   Include directory found" -ForegroundColor Green
} else {
    Write-Warning "   Include directory not found"
}

# Final summary
Write-Host "`nSetup Summary" -ForegroundColor Green
Write-Host "===============" -ForegroundColor Green
Write-Host "OpenSSL Directory: $opensslDir" -ForegroundColor White
Write-Host "Library Directory: $opensslLib" -ForegroundColor White
Write-Host "Include Directory: $opensslInclude" -ForegroundColor White
Write-Host "Binary Directory: $opensslBin" -ForegroundColor White

if ($allLibsFound) {
    Write-Host "`nOpenSSL setup completed successfully!" -ForegroundColor Green
    Write-Host @"

IMPORTANT: You need to restart your terminal or IDE for the environment variables to take effect.

To build the Solana GPU Trading Bot:
1. Open a NEW PowerShell window
2. Navigate to the project directory
3. Run: cargo build --release

"@ -ForegroundColor Yellow
} else {
    Write-Warning "`nOpenSSL setup completed with warnings. Some files may be missing."
    Write-Host "You may need to reinstall OpenSSL or adjust the library paths." -ForegroundColor Yellow
}

# Offer to test in new shell
$testNow = Read-Host "`nWould you like to open a new PowerShell window to test the setup? (Y/n)"
if ($testNow -ne 'n' -and $testNow -ne 'N') {
    Start-Process powershell -ArgumentList "-NoExit", "-Command", @"
Write-Host 'Testing OpenSSL setup...' -ForegroundColor Cyan
Write-Host 'OPENSSL_DIR: ' -NoNewline; Write-Host `$env:OPENSSL_DIR -ForegroundColor Green
Write-Host 'OPENSSL_LIB_DIR: ' -NoNewline; Write-Host `$env:OPENSSL_LIB_DIR -ForegroundColor Green
Write-Host 'OPENSSL_INCLUDE_DIR: ' -NoNewline; Write-Host `$env:OPENSSL_INCLUDE_DIR -ForegroundColor Green
Write-Host ''
openssl version
Write-Host ''
Write-Host 'If you see the OpenSSL version above, the setup was successful!' -ForegroundColor Green
Write-Host 'You can now run: cargo build --release' -ForegroundColor Yellow
"@
}

Write-Host "`nSetup script completed!" -ForegroundColor Green 