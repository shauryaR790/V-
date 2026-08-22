# Build one GitHub Release bundle locally (Windows).
# Usage: .\scripts\publish-release.ps1 -Version 1.0.4
# Output: manual-releases/v1.0.4/  (upload to GitHub Releases if CI has not run)

param(
    [Parameter(Mandatory = $true)]
    [string]$Version
)

$ErrorActionPreference = "Stop"
$Root = Split-Path $PSScriptRoot -Parent
Set-Location $Root

$tag = "v$Version"
$OutDir = Join-Path $Root "manual-releases/v$Version"
New-Item -ItemType Directory -Force -Path $OutDir | Out-Null

Write-Host "Building vpp $tag..." -ForegroundColor Cyan
cargo build --release --features codegen,lsp --bin vpp --bin vppls
if ($LASTEXITCODE -ne 0) { throw "cargo build failed" }

Write-Host "Staging..." -ForegroundColor Cyan
$staging = Join-Path $Root "staging"
if (Test-Path $staging) { Remove-Item $staging -Recurse -Force }
New-Item -ItemType Directory -Force -Path "$staging/examples", "$staging/llvm/bin" | Out-Null
Copy-Item target/release/vpp.exe, target/release/vppls.exe $staging/
Copy-Item -Recurse std, registry, runtime, cmake $staging/
Copy-Item examples/hello.vpp $staging/examples/
Copy-Item LICENSE $staging/

$llvmBin = "C:\LLVM\bin"
if (-not (Test-Path $llvmBin)) { $llvmBin = "C:\Program Files\LLVM\bin" }
if (Test-Path $llvmBin) {
    Copy-Item "$llvmBin\clang*.exe", "$llvmBin\lld*.exe", "$llvmBin\LLVM*.dll", "$llvmBin\lib*.dll" `
        -ErrorAction SilentlyContinue $staging/llvm/bin/
}

function Write-Sha256($filePath) {
    $hash = (Get-FileHash $filePath -Algorithm SHA256).Hash
    $name = Split-Path $filePath -Leaf
    "$hash  $name" | Set-Content "$filePath.sha256" -Encoding ascii
}

# Portable zip
$dirName = "vpp-v$Version-windows-x64"
$work = Join-Path $env:TEMP $dirName
if (Test-Path $work) { Remove-Item $work -Recurse -Force }
Copy-Item -Recurse $staging $work
Copy-Item GO.bat, RELEASE.txt, START-HERE.txt $work/
$zipPath = Join-Path $OutDir "vpp-v$Version-windows-x64.zip"
if (Test-Path $zipPath) { Remove-Item $zipPath -Force }
Compress-Archive -Path $work -DestinationPath $zipPath -Force
Write-Sha256 $zipPath
Remove-Item $work -Recurse -Force

# Installer
$iscc = $null
foreach ($p in @("C:\Inno Setup 6\ISCC.exe", "C:\InnoSetup6\ISCC.exe")) {
    if (Test-Path $p) { $iscc = $p; break }
}
if (-not $iscc) {
    Write-Host "Downloading Inno Setup..." -ForegroundColor Yellow
    $issInstaller = Join-Path $env:TEMP "innosetup-6.7.3.exe"
    curl.exe -L -o $issInstaller "https://github.com/jrsoftware/issrc/releases/download/is-6_7_3/innosetup-6.7.3.exe"
    Start-Process -FilePath $issInstaller -ArgumentList "/VERYSILENT", "/SUPPRESSMSGBOXES", "/DIR=C:\InnoSetup6" -Wait
    $iscc = "C:\InnoSetup6\ISCC.exe"
}

New-Item -ItemType Directory -Force -Path (Join-Path $Root "output") | Out-Null
Push-Location (Join-Path $Root "installer")
& $iscc "/DMyAppVersion=$Version" vpp-setup.iss
if ($LASTEXITCODE -ne 0) { throw "ISCC failed" }
Pop-Location

$setup = Join-Path $Root "output/vpp-$Version-setup.exe"
Copy-Item $setup $OutDir/ -Force
Write-Sha256 (Join-Path $OutDir "vpp-$Version-setup.exe")

# VSIX (extension version from package.json — independent from compiler)
$pkgJson = Join-Path $Root "editor/vscode-vpp/package.json"
$extVer = (Get-Content $pkgJson -Raw | ConvertFrom-Json).version
$vsix = Join-Path $OutDir "vplusplus-$extVer.vsix"
Push-Location (Join-Path $Root "editor/vscode-vpp")
if (-not (Get-Command vsce -ErrorAction SilentlyContinue)) {
    npm install -g @vscode/vsce 2>&1 | Out-Null
}
vsce package -o $vsix
Pop-Location

Write-Host "`nBuilt $tag -> $OutDir" -ForegroundColor Green
Get-ChildItem $OutDir | ForEach-Object { Write-Host "  $($_.Name)  ($([math]::Round($_.Length/1MB, 1)) MB)" }
Write-Host @"

Upload to GitHub Releases (tag $tag):
  https://github.com/shauryaR790/VPP/releases/new?tag=$tag

Or wait for CI: pushing tag $tag triggers .github/workflows/release.yml

VSIX -> Marketplace (extension $extVer, compiler $tag):
  https://marketplace.visualstudio.com/manage/publishers/vpp-lang
"@
