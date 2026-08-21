# Prevent Cursor from appearing as a GitHub contributor on this repo.
# Applies to commits made from this workspace (local agent, CLI, or manual).

$ErrorActionPreference = "Stop"
$repoRoot = Split-Path $PSScriptRoot -Parent
$hooksDir = Join-Path $repoRoot ".git\hooks"
$srcHook = Join-Path $PSScriptRoot "git-hooks\prepare-commit-msg"

if (-not (Test-Path (Join-Path $repoRoot ".git"))) {
    Write-Host "Not a git repo: $repoRoot" -ForegroundColor Red
    exit 1
}

New-Item -ItemType Directory -Force -Path $hooksDir | Out-Null
Copy-Item $srcHook (Join-Path $hooksDir "prepare-commit-msg") -Force

Write-Host "Installed prepare-commit-msg hook -> strips Cursor co-author trailers" -ForegroundColor Green
Write-Host "Repo: $repoRoot" -ForegroundColor Gray
