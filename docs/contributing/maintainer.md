# Maintainer scripts (optional)

## Package VSIX + Windows zip locally

```powershell
.\scripts\package-manual-releases.ps1
```

Output goes to `manual-releases/` (gitignored).

## Clean old GitHub Actions runs

GitHub does not expose this via git. As repo owner:

1. Open https://github.com/shauryaR790/V-/actions
2. Click **Release** (or **CI**) in the left sidebar
3. For each **cancelled** or **failed** run you want gone: open it → **⋯** (top right) → **Delete workflow run**
4. Optional: **Settings → Actions → General** → set **Artifact and log retention** to 7 days to limit future clutter

Bulk delete: filter by status, select runs (checkboxes), **Delete** (requires admin).
