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
| `vpp-v0.4.1-windows-x64.zip` | Windows — unzip, double-click `GO.bat` |
| `vpp-v0.4.1-linux-x64.tar.gz` | Linux |
| `vpp-v0.4.1-macos-x64.tar.gz` | macOS |

Each bundle includes:

- `vpp` / `vpp.exe` — compiler + interpreter
- `vppls` / `vppls.exe` — language server
- `std/`, `registry/`, `runtime/`
- `examples/hello.vpp`
- `GO.bat` (Windows) or `run.sh` (Unix)

## User install (Windows)

1. [GitHub Releases](https://github.com/shauryaR790/V-/releases) → download Windows zip
2. Unzip anywhere (e.g. `C:\vpp`)
3. Double-click **GO.bat**
4. Install VS Code extension: search **v++**

No Rust. No `cargo build`. LLVM only needed for `vpp build` (native `.exe` output).

## Check workflow

After pushing a tag: **GitHub repo → Actions → Release** — wait for green, then **Releases** tab.
