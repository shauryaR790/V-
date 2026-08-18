# One-time install: puts `vpp` on your PATH forever (like python or node)
$ErrorActionPreference = "Stop"
$env:Path = "$env:USERPROFILE\.cargo\bin;" + $env:Path

Write-Host "`n=== Installing v++ globally ===" -ForegroundColor Cyan

if (-not (Test-Path "$env:USERPROFILE\.cargo\bin\cargo.exe")) {
    Write-Host "Install Rust first: https://rustup.rs" -ForegroundColor Red
    exit 1
}

Push-Location $PSScriptRoot
cargo build --release --features codegen,lsp
cargo install --path . --features codegen,lsp --force
Pop-Location

Write-Host "`nDone. Close and reopen your terminal, then from the project folder:" -ForegroundColor Green
Write-Host "  vpp run stress.vpp" -ForegroundColor White
Write-Host "  vpp build stress.vpp -o stress.exe" -ForegroundColor White
Write-Host "  .\stress.ps1          # automatic interpreter vs native compare" -ForegroundColor White
Write-Host ""
Write-Host "Or always use the local wrapper (no install needed):" -ForegroundColor Green
Write-Host "  .\vpp.ps1 run stress.vpp" -ForegroundColor White
Write-Host ""

& (Join-Path $PSScriptRoot "stress.ps1")
