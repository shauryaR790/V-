# Code signing (Windows)

Python installs without security warnings because **python.org ships a signed installer** from a verified publisher. v++ needs the same.

## Free signing for open source — SignPath

SignPath Foundation provides **free Authenticode signing** for qualifying OSS projects on GitHub.

1. Apply: https://signpath.io/product/open-source  
2. Install the [SignPath GitHub App](https://github.com/apps/signpath) on `shauryaR790/V-`  
3. After approval, add these **repository secrets**:
   - `SIGNPATH_API_TOKEN`
   - `SIGNPATH_ORGANIZATION_ID`
   - `SIGNPATH_PROJECT_SLUG` (e.g. `vpp`)
   - `SIGNPATH_SIGNING_POLICY_SLUG` (e.g. `release-signing`)
4. Set repository variable `SIGNPATH_ENABLED` = `true`
5. Push a new tag — Release workflow signs `vpp-*-setup.exe` automatically

Until signing is enabled, the installer is **unsigned** and Windows SmartScreen may show one prompt (same as many indie tools). After SignPath is wired up, users see **vpp-lang** (or SignPath Foundation) as publisher — like Python.

## What we ship

| Artifact | Purpose |
|----------|---------|
| `vpp-x.y.z-setup.exe` | **Primary** — Windows installer (adds to PATH, no `.bat`) |
| `vpp-x.y.z-windows-x64.zip` | Portable / advanced users |

## Alternative (paid)

- Buy an EV/OV code signing certificate (~$200–400/yr) and sign in CI with `signtool.exe`
- Azure Artifact Signing (Microsoft cloud signing) — requires Azure subscription

## winget (future)

Once the installer is signed, we can publish a winget manifest so users run:

```powershell
winget install vpp-lang.vpp
```

That path also avoids zip/unblock friction.
