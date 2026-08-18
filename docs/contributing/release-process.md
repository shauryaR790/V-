# Release process

1. Bump `Cargo.toml` and `editor/vscode-vpp/package.json`
2. Update `CHANGELOG.md`
3. Commit, tag, push:

```powershell
git tag v0.5.0
git push origin main
git push origin v0.5.0
```

4. Wait for [Release workflow](https://github.com/shauryaR790/V-/actions) — green
5. Verify [GitHub Releases](https://github.com/shauryaR790/V-/releases)
6. Upload VSIX to Marketplace if extension changed

Details: [RELEASE.md](../RELEASE.md), signing: [SIGNING.md](../SIGNING.md).
