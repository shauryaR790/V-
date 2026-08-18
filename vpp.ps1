# Run the local v++ compiler (no global install needed)
$ErrorActionPreference = "Stop"

$env:Path = "$env:USERPROFILE\.cargo\bin;" + $env:Path
$env:VPP_HOME = $PSScriptRoot
$release = Join-Path $PSScriptRoot "target\release\vpp.exe"
$debug = Join-Path $PSScriptRoot "target\debug\vpp.exe"
$bin = $null
if ((Test-Path $release) -and (Test-Path $debug)) {
    $bin = if ((Get-Item $release).LastWriteTime -ge (Get-Item $debug).LastWriteTime) { $release } else { $debug }
} elseif (Test-Path $release) {
    $bin = $release
} elseif (Test-Path $debug) {
    $bin = $debug
}

if (-not (Test-Path $bin)) {
    Write-Host "Building vpp..." -ForegroundColor Yellow
    Push-Location $PSScriptRoot
    cargo build
    Pop-Location
}

& $bin @args
