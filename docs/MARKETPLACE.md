# Publish v++ to VS Code Marketplace

Do this once. Updates later are just bump version + publish.

**Your extension ID:** `vpp-lang.vplusplus`  
**Marketplace URL (after publish):** https://marketplace.visualstudio.com/items?itemName=vpp-lang.vplusplus

---

## 1. Create a Microsoft account (if needed)

**Do not start at dev.azure.com** — that signup form often rejects Gmail.

1. Open an **Incognito/Private** window
2. Go to [account.microsoft.com](https://account.microsoft.com) → **Create an account**
3. Use your Gmail (or **Sign in with GitHub** — works well since your repo is on GitHub)
4. Finish Microsoft account setup there first

---

## 2. Create publisher (5 min, one time)

1. Same incognito window → [Create Publisher](https://marketplace.visualstudio.com/manage/createpublisher)
2. Sign in with the Microsoft account you just made
3. **ID:** `vpp-lang` (must match `package.json` → `"publisher"`)
4. **Name:** `v++ Language`
5. Create

---

## 3. Get access token (5 min, one time)

**Option A — from Marketplace (try this first)**

1. [Manage publishers](https://marketplace.visualstudio.com/manage/publishers/)
2. Click your publisher **vpp-lang**
3. Look for **Security**, **Personal access tokens**, or **Access tokens**
4. New token → scope **Marketplace → Manage**
5. Copy the token

**Option B — Azure DevOps (only if Option A has no token page)**

1. [dev.azure.com](https://dev.azure.com/) → sign in with the **same** account
2. If it asks “few more details” and email goes red → your Microsoft account isn’t ready; go back to step 1 or use GitHub sign-in
3. Create an organization (any name, e.g. `vpp-lang`)
4. Profile → **Personal access tokens** → **+ New Token**
5. **Organization:** All accessible organizations  
   **Scopes:** Marketplace → **Manage**
6. Copy the token

---

## 4. Install tools

```powershell
# Node.js from https://nodejs.org if needed
npm install -g @vscode/vsce
cd C:\Users\shaur\v++\editor\vscode-vpp
npm install
vsce login vpp-lang
# paste token when prompted
```

---

## 5. Test package locally

```powershell
cd C:\Users\shaur\v++\editor\vscode-vpp
.\publish.ps1
code --install-extension vpp-0.4.0.vsix
```

Reload VS Code → open `examples\hello.vpp` → **F5**.

---

## 6. Publish

```powershell
.\publish.ps1 -Publish
```

Or paste token directly (no login saved):

```powershell
vsce publish -p YOUR_TOKEN_HERE
```

First publish can take 5–15 minutes to show on the Marketplace.

---

## 7. Tell users how to install

**From VS Code:** Extensions → search **v++** → Install **v++ Language**

**From terminal:**

```powershell
code --install-extension vpp-lang.vplusplus
```

Users still need the **vpp compiler** separately (GitHub Release or `setup.ps1`). The extension does not bundle it.

---

## Updates

1. Bump `"version"` in `editor/vscode-vpp/package.json`
2. `.\publish.ps1 -Publish`

---

## Troubleshooting

| Problem | Fix |
|---------|-----|
| Azure DevOps email red / won’t continue | Skip Azure. Create account at account.microsoft.com first, or use **Sign in with GitHub** |
| Redirect loop when getting PAT | Incognito window, sign out all Microsoft accounts, use [Manage publishers](https://marketplace.visualstudio.com/manage/publishers/) |
| `vsce login` fails | Use `vsce publish -p YOUR_TOKEN` instead |
| Publisher ID taken | Pick another ID (e.g. `shaurya-vpp`) and change `"publisher"` in `package.json` |

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
