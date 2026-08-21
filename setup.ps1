# One-time setup + build for v++
# Right-click -> Run with PowerShell, OR in terminal: .\setup.ps1

$ErrorActionPreference = "Stop"

$env:Path = "$env:USERPROFILE\.cargo\bin;" + $env:Path

Write-Host "`n=== v++ setup ===" -ForegroundColor Cyan

if (-not (Test-Path "$env:USERPROFILE\.cargo\bin\cargo.exe")) {
    Write-Host "Rust not found. Install from https://rustup.rs then run this again." -ForegroundColor Red
    exit 1
}

Write-Host "Building v++ compiler (interpreter + native codegen + LSP)..." -ForegroundColor Yellow
Push-Location $PSScriptRoot

$extSrc = Join-Path $PSScriptRoot "editor\vscode-vpp"
if ($env:LLVM_SYS_221_PREFIX) {
    $env:PATH = "$env:LLVM_SYS_221_PREFIX\bin;$env:PATH"
} elseif (Test-Path "C:\Program Files\LLVM\bin") {
    $env:LLVM_SYS_221_PREFIX = "C:\Program Files\LLVM"
    $env:PATH = "C:\Program Files\LLVM\bin;$env:PATH"
}

$hasLlvm = $null -ne (Get-Command clang -ErrorAction SilentlyContinue)
if (-not $hasLlvm) {
    Write-Host "Note: LLVM/clang not found - interpreter works; native build needs:" -ForegroundColor Yellow
    Write-Host "  winget install LLVM.LLVM" -ForegroundColor White
    Write-Host "  then re-run .\setup.ps1" -ForegroundColor White
}

cargo build --release --features codegen,lsp
Pop-Location

$pkg = Get-Content (Join-Path $extSrc "package.json") -Raw | ConvertFrom-Json
$extVersion = $pkg.version
$extId = "$($pkg.publisher).$($pkg.name)"
$extTargets = @(
    (Join-Path $env:USERPROFILE ".vscode\extensions\$extId-$extVersion"),
    (Join-Path $env:USERPROFILE ".cursor\extensions\$extId-$extVersion")
)
Write-Host "Installing v++ editor extension ($extId@$extVersion)..." -ForegroundColor Yellow

# Remove legacy manually-copied extension folders that shadow the current package.
$legacyRoots = @(
    (Join-Path $env:USERPROFILE ".vscode\extensions"),
    (Join-Path $env:USERPROFILE ".cursor\extensions")
)
foreach ($legacyRoot in $legacyRoots) {
    if (-not (Test-Path $legacyRoot)) { continue }
    Get-ChildItem $legacyRoot -Directory -Filter "vpp-lang.vpp-*" -ErrorAction SilentlyContinue | ForEach-Object {
        Write-Host "  removing legacy $($_.Name)" -ForegroundColor DarkYellow
        Remove-Item -Recurse -Force $_.FullName
    }
}

foreach ($extDest in $extTargets) {
    $parent = Split-Path $extDest -Parent
    if (-not (Test-Path $parent)) {
        Write-Host "  skip (not installed): $parent" -ForegroundColor DarkGray
        continue
    }
    if (Test-Path $extDest) { Remove-Item -Recurse -Force $extDest }
    New-Item -ItemType Directory -Path $extDest -Force | Out-Null
    robocopy $extSrc $extDest /E /XD node_modules .vscode /XF *.vsix /NFL /NDL /NJH /NJS | Out-Null
    Write-Host "  installed -> $extDest" -ForegroundColor Gray
}

Write-Host "`nDone! In VS Code or Cursor:" -ForegroundColor Green
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
if ($hasLlvm) {
    & (Join-Path $PSScriptRoot "stress.ps1")
} else {
    Write-Host "Skipping native parity test (install LLVM to enable)." -ForegroundColor Yellow
    & (Join-Path $PSScriptRoot "vpp.ps1") run $stress
}
