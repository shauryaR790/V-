# One-time install: puts `vpp` on your PATH forever (like python or node)
$ErrorActionPreference = "Stop"
$env:Path = "$env:USERPROFILE\.cargo\bin;" + $env:Path

Write-Host "`n=== Installing v++ globally ===" -ForegroundColor Cyan

if (-not (Test-Path "$env:USERPROFILE\.cargo\bin\cargo.exe")) {
    Write-Host "Install Rust first: https://rustup.rs" -ForegroundColor Red
    exit 1
}

Push-Location $PSScriptRoot
cargo build --release
cargo install --path . --force
Pop-Location

Write-Host "`nDone. Close and reopen Cursor, then from ANY folder:" -ForegroundColor Green
Write-Host "  vpp run examples\hello.vpp" -ForegroundColor White
Write-Host "  vpp init myapp" -ForegroundColor White
Write-Host ""
Write-Host "In Cursor: open a .vpp file and press F5 or click the Run button." -ForegroundColor Green
Write-Host ""

vpp run "$PSScriptRoot\examples\hello.vpp"
