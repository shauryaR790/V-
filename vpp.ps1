# Run the local v++ compiler (no global install needed)
$ErrorActionPreference = "Stop"

$env:Path = "$env:USERPROFILE\.cargo\bin;" + $env:Path
$bin = Join-Path $PSScriptRoot "target\release\vpp.exe"
if (-not (Test-Path $bin)) {
    $bin = Join-Path $PSScriptRoot "target\debug\vpp.exe"
}

if (-not (Test-Path $bin)) {
    Write-Host "Building vpp..." -ForegroundColor Yellow
    Push-Location $PSScriptRoot
    cargo build
    Pop-Location
}

& $bin @args
