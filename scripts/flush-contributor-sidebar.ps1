# Flush GitHub repo homepage "Contributors" sidebar cache
#
# GitHub maintains TWO contributor lists:
#   - Insights > Contributors  (recalculated from git — yours is already clean)
#   - Repo homepage sidebar      (separate cache — can stay stale for days)
#
# Toggling the default branch forces the sidebar to rebuild (~60–90 seconds).

$ErrorActionPreference = "Stop"
$repoRoot = Split-Path $PSScriptRoot -Parent
Push-Location $repoRoot

$flushBranch = "chore/flush-contributor-sidebar"
$defaultBranch = "main"

Write-Host "`n=== GitHub contributor sidebar cache flush ===" -ForegroundColor Cyan
Write-Host "Insights already shows only you. This fixes the OUTSIDE sidebar.`n" -ForegroundColor Gray

# Create/push a harmless temp branch (GitHub needs a second branch to toggle to)
git fetch origin 2>$null
$exists = git show-ref --verify --quiet "refs/heads/$flushBranch"; if ($LASTEXITCODE -eq 0) { $hasLocal = $true } else { $hasLocal = $false }

if (-not $hasLocal) {
    git checkout -b $flushBranch origin/$defaultBranch 2>$null
    if ($LASTEXITCODE -ne 0) { git checkout -b $flushBranch $defaultBranch }
    git commit --allow-empty -m "chore: flush GitHub contributor sidebar cache"
}

Write-Host "Pushing branch '$flushBranch'..." -ForegroundColor Yellow
git push -u origin $flushBranch

Write-Host @"

NEXT — do this on GitHub (requires repo Admin):

  1. Open: https://github.com/shauryaR790/V-/settings/branches
  2. Under "Default branch", click the switch/edit icon
  3. Select:  $flushBranch
  4. Confirm the change
  5. Wait 90 seconds (seriously — the sidebar cache is slow)
  6. Switch default branch BACK to:  $defaultBranch
  7. Wait another 90 seconds
  8. Hard-refresh the repo homepage (Ctrl+Shift+R)

The cursoragent avatar should disappear from the right sidebar.
Insights > Contributors should still show only you.

If it STILL shows cursoragent after 24h:
  - GitHub Support → "Remove data from a repository I own"
  - Ask them to recompute the homepage contributor sidebar cache
  - Mention: Insights is clean, sidebar still lists cursoragent after history rewrite

Optional cleanup after it works:
  git push origin --delete $flushBranch

"@ -ForegroundColor Green

Pop-Location
