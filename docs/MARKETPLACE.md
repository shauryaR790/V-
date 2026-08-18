# Publish v++ to VS Code Marketplace

Do this once. Updates later are just bump version + publish.

**Your extension ID:** `vpp-lang.vpp`  
**Marketplace URL (after publish):** https://marketplace.visualstudio.com/items?itemName=vpp-lang.vpp

---

## 1. Create publisher (5 min, one time)

1. Sign in at [Create Publisher](https://marketplace.visualstudio.com/manage/createpublisher)
2. **ID:** `vpp-lang` (must match `package.json` → `"publisher"`)
3. **Name:** `v++ Language` (or similar)
4. Create

---

## 2. Get access token (5 min, one time)

1. Open [Azure DevOps](https://dev.azure.com/) with the **same Microsoft account**
2. Create an organization if prompted (any name)
3. Profile icon → **Personal access tokens** → **+ New Token**
4. Scope: **Marketplace → Manage**
5. Copy the token — you won't see it again

---

## 3. Install tools

```powershell
# Node.js from https://nodejs.org if needed
npm install -g @vscode/vsce
cd C:\Users\shaur\v++\editor\vscode-vpp
npm install
vsce login vpp-lang
# paste token when prompted
```

---

## 4. Test package locally

```powershell
cd C:\Users\shaur\v++\editor\vscode-vpp
.\publish.ps1
code --install-extension vpp-0.4.0.vsix
```

Reload VS Code → open `examples\hello.vpp` → **F5**.

---

## 5. Publish

```powershell
.\publish.ps1 -Publish
```

Or: `vsce publish`

First publish can take 5–15 minutes to show on the Marketplace.

---

## 6. Tell users how to install

**From VS Code:** Extensions → search **v++** → Install **v++ Language**

**From terminal:**

```powershell
code --install-extension vpp-lang.vpp
```

Users still need the **vpp compiler** separately (GitHub Release or `setup.ps1`). The extension does not bundle it.

---

## Updates

1. Bump `"version"` in `editor/vscode-vpp/package.json`
2. `.\publish.ps1 -Publish`

---

## Checklist

- [ ] Publisher `vpp-lang` created
- [ ] `vsce login vpp-lang` works
- [ ] `.\publish.ps1` builds `.vsix` without errors
- [ ] Local `.vsix` install — F5 runs a `.vpp` file
- [ ] `.\publish.ps1 -Publish` succeeds
- [ ] Extension live at marketplace.visualstudio.com

---

## Roadmap (your plan)

1. **Marketplace extension** ← you are here  
2. **GitHub Releases** — prebuilt `vpp.exe` zip (no Rust needed)  
3. **Website** — docs + one-click install links to Marketplace + Releases
