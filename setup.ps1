# One-time setup + build for v++
# Right-click -> Run with PowerShell, OR in terminal: .\setup.ps1

$ErrorActionPreference = "Stop"

$env:Path = "$env:USERPROFILE\.cargo\bin;" + $env:Path

Write-Host "`n=== v++ setup ===" -ForegroundColor Cyan

if (-not (Test-Path "$env:USERPROFILE\.cargo\bin\cargo.exe")) {
    Write-Host "Rust not found. Install from https://rustup.rs then run this again." -ForegroundColor Red
    exit 1
}

Write-Host "Building v++ compiler..." -ForegroundColor Yellow
Push-Location $PSScriptRoot
cargo build --release
Pop-Location

$extDest = Join-Path $env:USERPROFILE ".cursor\extensions\vpp-lang.vpp-0.3.0"
Write-Host "Installing v++ editor extension..." -ForegroundColor Yellow
if (Test-Path $extDest) { Remove-Item -Recurse -Force $extDest }
Copy-Item -Recurse (Join-Path $PSScriptRoot "editor\vscode-vpp") $extDest

Write-Host "`nDone! In Cursor:" -ForegroundColor Green
Write-Host '  1. Reload window (Ctrl+Shift+P, then Developer: Reload Window)' -ForegroundColor White
Write-Host '  2. Open examples\hello.vpp' -ForegroundColor White
Write-Host '  3. Press F5 or click the Run button (top right)' -ForegroundColor White
Write-Host ""
Write-Host 'Optional: run .\install.ps1 to put vpp on PATH forever' -ForegroundColor Yellow
Write-Host ""
Write-Host 'Or from terminal:' -ForegroundColor Green
Write-Host '  .\vpp.ps1 run stress.vpp' -ForegroundColor White
Write-Host '  .\stress.ps1                 # interpreter vs native parity test' -ForegroundColor White
Write-Host ""

$stress = Join-Path $PSScriptRoot "stress.vpp"
& (Join-Path $PSScriptRoot "stress.ps1")
