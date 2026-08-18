# Interpreter vs native parity test for stress.vpp
# Usage: .\stress.ps1

$ErrorActionPreference = "Stop"

$root = $PSScriptRoot
$vpp = Join-Path $root "vpp.ps1"
$src = Join-Path $root "stress.vpp"
$exe = Join-Path $root "stress.exe"
$interpOut = Join-Path $root "interpreter.txt"
$nativeOut = Join-Path $root "native.txt"

if (-not (Test-Path $src)) {
    Write-Host "stress.vpp not found in $root" -ForegroundColor Red
    exit 1
}

# LLVM needed for native build
if ($env:LLVM_SYS_221_PREFIX) {
    $env:PATH = "$env:LLVM_SYS_221_PREFIX\bin;$env:PATH"
} elseif (Test-Path "C:\Program Files\LLVM\bin") {
    $env:LLVM_SYS_221_PREFIX = "C:\Program Files\LLVM"
    $env:PATH = "C:\Program Files\LLVM\bin;$env:PATH"
}

$env:VPP_HOME = $root

Write-Host "`n=== v++ stress parity test ===" -ForegroundColor Cyan
Write-Host "Source: stress.vpp`n" -ForegroundColor Gray

Write-Host "[1/3] Interpreter..." -ForegroundColor Yellow
& $vpp run $src 2>&1 | Tee-Object -FilePath $interpOut
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host "`n[2/3] Native build..." -ForegroundColor Yellow
& $vpp build $src -o $exe 2>&1
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host "`n[3/3] Native run..." -ForegroundColor Yellow
& $exe 2>&1 | Tee-Object -FilePath $nativeOut
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

$diff = Compare-Object (Get-Content $interpOut) (Get-Content $nativeOut)
Write-Host ""
if (-not $diff) {
    Write-Host "PASS: interpreter and native output are identical." -ForegroundColor Green
    exit 0
}

Write-Host "FAIL: output mismatch" -ForegroundColor Red
Write-Host "  interpreter.txt vs native.txt"
$diff | Format-Table -AutoSize
exit 1
