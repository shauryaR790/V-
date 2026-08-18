# GitHub Releases

Prebuilt binaries are published automatically when you push a version tag.

## Create a release

```powershell
# 1. Bump version in Cargo.toml + CHANGELOG.md
# 2. Commit
git add Cargo.toml CHANGELOG.md
git commit -m "Release v0.4.1"

# 3. Tag and push
git tag v0.4.1
git push origin main
git push origin v0.4.1
```

GitHub Actions (`.github/workflows/release.yml`) builds Windows, Linux, and macOS bundles and attaches them to the Release page.

## What users download

| File | Platform |
|------|----------|
| `vpp-v0.4.3-setup.exe` | **Windows (recommended)** — installer, adds PATH |
| `vpp-v0.4.3-windows-x64.zip` | Windows portable |
| `vpp-v0.4.3-linux-x64.tar.gz` | Linux |
| `vpp-v0.4.3-macos-arm64.tar.gz` | macOS |

Code signing (no SmartScreen like Python): see [docs/SIGNING.md](SIGNING.md).

## User install (Windows)

1. [GitHub Releases](https://github.com/shauryaR790/V-/releases) → download **`vpp-*-setup.exe`**
2. Run installer
3. Open a new terminal: `vpp run examples\hello.vpp`
4. VS Code extension: **v++ Language** (vpp-lang)

No Rust. No `cargo build`. LLVM only needed for `vpp build` (native `.exe` output).

## Check workflow

After pushing a tag: **GitHub repo → Actions → Release** — wait for green, then **Releases** tab.
