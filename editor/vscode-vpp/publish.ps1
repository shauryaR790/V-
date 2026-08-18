# Package and optionally publish the v++ VS Code extension to the Marketplace.
# Usage:
#   .\publish.ps1              # creates vpp-0.4.0.vsix (install locally)
#   .\publish.ps1 -Publish       # uploads to Marketplace (requires vsce login first)

param(
    [switch]$Publish
)

$ErrorActionPreference = "Stop"
$here = $PSScriptRoot

Write-Host "`n=== v++ VS Code extension ===" -ForegroundColor Cyan

if (-not (Get-Command npm -ErrorAction SilentlyContinue)) {
    Write-Host "Node.js/npm not found. Install from https://nodejs.org" -ForegroundColor Red
    exit 1
}

Push-Location $here
try {
    if (-not (Test-Path "node_modules")) {
        Write-Host "Installing extension dependencies..." -ForegroundColor Yellow
        npm install
    }

    if (-not (Get-Command vsce -ErrorAction SilentlyContinue)) {
        Write-Host "Installing @vscode/vsce globally..." -ForegroundColor Yellow
        npm install -g @vscode/vsce
    }

    if ($Publish) {
        Write-Host "Publishing to VS Code Marketplace..." -ForegroundColor Yellow
        Write-Host "(You must run 'vsce login vpp-lang' once before this works.)" -ForegroundColor Gray
        vsce publish
        Write-Host "`nPublished! Users can install with:" -ForegroundColor Green
        Write-Host "  code --install-extension vpp-lang.vpp" -ForegroundColor White
    } else {
        Write-Host "Packaging .vsix..." -ForegroundColor Yellow
        vsce package
        $vsix = Get-ChildItem -Filter "*.vsix" | Sort-Object LastWriteTime -Descending | Select-Object -First 1
        Write-Host "`nCreated: $($vsix.Name)" -ForegroundColor Green
        Write-Host "`nInstall locally in VS Code:" -ForegroundColor Green
        Write-Host "  code --install-extension $($vsix.FullName)" -ForegroundColor White
        Write-Host "`nOr: Extensions sidebar -> ... -> Install from VSIX" -ForegroundColor Gray
    }
} finally {
    Pop-Location
}
