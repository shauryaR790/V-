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
    Write-Host "Building vpp (codegen + lsp)..." -ForegroundColor Yellow
    Push-Location $PSScriptRoot
    if (Test-Path "C:\Program Files\LLVM\bin") {
        $env:LLVM_SYS_221_PREFIX = "C:\Program Files\LLVM"
        $env:PATH = "C:\Program Files\LLVM\bin;$env:PATH"
    }
    cargo build --features codegen,lsp
    Pop-Location
}

& $bin @args
