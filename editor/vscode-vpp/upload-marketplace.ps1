# Build the VSIX and open the Marketplace page where you upload it (same flow as before).
# Usage: .\upload-marketplace.ps1

$ErrorActionPreference = "Stop"
$here = $PSScriptRoot
Push-Location $here

if (-not (Get-Command npm -ErrorAction SilentlyContinue)) {
    Write-Host "Install Node.js from https://nodejs.org first." -ForegroundColor Red
    exit 1
}

if (-not (Get-Command vsce -ErrorAction SilentlyContinue)) {
    npm install -g @vscode/vsce | Out-Null
}

$pkg = Get-Content "package.json" -Raw | ConvertFrom-Json
$vsixName = "$($pkg.name)-$($pkg.version).vsix"
$repoRoot = Split-Path (Split-Path $here -Parent) -Parent
$outPath = Join-Path $repoRoot $vsixName

Write-Host "`nPackaging $vsixName ..." -ForegroundColor Cyan
vsce package -o $outPath

$manageUrl = "https://marketplace.visualstudio.com/manage/publishers/$($pkg.publisher)/hubs/extensions/detail/$($pkg.publisher).$($pkg.name)/update"
Write-Host "`nVSIX ready:" -ForegroundColor Green
Write-Host "  $outPath" -ForegroundColor White
Write-Host "`nOpening Marketplace upload page..." -ForegroundColor Cyan
Write-Host "  1. Sign in if asked" -ForegroundColor Yellow
Write-Host "  2. Click 'Upload' / drag the VSIX file" -ForegroundColor Yellow
Write-Host "  3. Submit - new version goes live in a few minutes`n" -ForegroundColor Yellow

Start-Process $manageUrl
Invoke-Item (Split-Path $outPath -Parent)
Write-Host "Select file: $(Split-Path $outPath -Leaf)" -ForegroundColor White

Pop-Location
