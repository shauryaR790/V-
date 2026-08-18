# v++ in VS Code — full setup guide

This is the step-by-step guide for **using v++ in VS Code** and **publishing the extension** to the Marketplace so anyone can install it with one click.

---

## Part 1: Use v++ in VS Code (for you or your users)

### What you need installed first

| Tool | Why | Install |
|------|-----|---------|
| **VS Code** | The editor | [code.visualstudio.com](https://code.visualstudio.com/) |
| **Rust** | Build `vpp` from source (or skip if using prebuilt release) | [rustup.rs](https://rustup.rs/) |
| **LLVM** (optional) | Only for `vpp build` (native `.exe`) | `winget install LLVM.LLVM` |

---

### Step 1 — Get the v++ compiler

**Option A — Clone and build (developers)**

```powershell
git clone https://github.com/shauryaR790/V-.git vpp
cd vpp
.\setup.ps1
```

This builds `vpp` and copies the extension into VS Code’s extensions folder.

**Option B — Prebuilt binary (end users)**

1. Go to [GitHub Releases](https://github.com/shauryaR790/V-/releases)
2. Download the Windows zip for `v0.4.0` (or latest)
3. Extract to e.g. `C:\vpp`
4. Add to PATH and set home:

```powershell
$env:VPP_HOME = "C:\vpp"
$env:PATH = "C:\vpp;$env:PATH"
vpp doctor
```

**Option C — Global install (after Option A)**

```powershell
cd vpp
.\install.ps1
```

Close and reopen the terminal — then `vpp` works everywhere.

---

### Step 2 — Install the VS Code extension

**Option A — From Marketplace (after you publish — Part 2 below)**

1. Open VS Code
2. `Ctrl+Shift+X` → Extensions
3. Search **v++**
4. Install **v++ Language** by **vpp-lang**

**Option B — Local install (works today, no Marketplace needed)**

After running `.\setup.ps1` from the repo:

1. `Ctrl+Shift+P` → **Developer: Reload Window**

Or install the packaged file:

```powershell
cd editor\vscode-vpp
.\publish.ps1
code --install-extension vpp-0.4.0.vsix
```

**Option C — Manual copy**

```powershell
Copy-Item -Recurse editor\vscode-vpp "$env:USERPROFILE\.vscode\extensions\vpp-lang.vplusplus-0.4.0"
```

Then reload VS Code.

---

### Step 3 — Open a project

1. VS Code → **File → Open Folder**
2. Select the `vpp` repo (or any folder with `.vpp` files)

---

### Step 4 — Run your first program

1. Open `examples\hello.vpp` or `stress.vpp`
2. Check bottom-right corner says **v++** (not Plain Text)
3. Press **F5** (or click ▶ top-right)

Output shows in the **v++** output panel.

Other shortcuts:

| Action | How |
|--------|-----|
| Run file | **F5** or **Ctrl+Shift+R** |
| Type-check only | Right-click → **v++: Check File** |
| Run tests | Command palette → **v++: Run Tests** |

---

### Step 5 — Fix common problems

| Problem | Fix |
|---------|-----|
| Plain Text instead of v++ | Click language mode (bottom-right) → select **v++** |
| “compiler not found” | Run `.\setup.ps1` or set `vpp.compilerPath` in settings |
| No red squiggles | Build with LSP: `cargo build --features lsp,codegen` |
| F5 does nothing | Reload window; confirm extension is enabled |

---

## Part 2: Publish to VS Code Marketplace

**→ See [MARKETPLACE.md](MARKETPLACE.md)** — short step-by-step (publisher, token, publish).

<details>
<summary>Full reference (same steps, expanded)</summary>

1. Open [Create Publisher](https://marketplace.visualstudio.com/manage/createpublisher)
2. Sign in with Microsoft
3. Fill in:
   - **Name:** `vpp-lang` (display name, e.g. “v++ Language”)
   - **ID:** `vpp-lang` — **must match** `publisher` in `editor/vscode-vpp/package.json`
4. Click **Create**

You only do this once.

---

### Step 3 — Create a Personal Access Token (PAT)

1. Open [Azure DevOps Tokens](https://dev.azure.com/) — sign in with the **same** Microsoft account
2. If asked to create an organization, create one (any name is fine)
3. Click your profile (top right) → **Personal access tokens**
4. **+ New Token**
   - Name: `vscode-marketplace`
   - Organization: **All accessible organizations**
   - Expiration: 90 days or custom
   - Scopes: **Custom defined** → check **Marketplace → Manage**
5. **Create** → **copy the token** (you won’t see it again)

Store it somewhere safe (password manager).

---

### Step 4 — Install publishing tools

```powershell
# Node.js from https://nodejs.org if needed
npm install -g @vscode/vsce
cd C:\Users\shaur\v++\editor\vscode-vpp
npm install
```

Log in to your publisher (paste the PAT when prompted):

```powershell
vsce login vpp-lang
```

---

### Step 5 — Package (test locally first)

```powershell
cd C:\Users\shaur\v++\editor\vscode-vpp
.\publish.ps1
```

This creates `vpp-0.4.0.vsix`. Install it to verify:

```powershell
code --install-extension vpp-0.4.0.vsix
```

Reload VS Code, open a `.vpp` file, press F5.

---

### Step 6 — Publish to Marketplace

```powershell
cd C:\Users\shaur\v++\editor\vscode-vpp
.\publish.ps1 -Publish
```

Or manually:

```powershell
vsce publish
```

First publish can take a few minutes to appear on the Marketplace.

Your extension URL will be:

`https://marketplace.visualstudio.com/items?itemName=vpp-lang.vplusplus`

Users install with:

```powershell
code --install-extension vpp-lang.vplusplus
```

Or search **v++** in the Extensions sidebar.

---

### Step 7 — Publish updates later

1. Bump `"version"` in `editor/vscode-vpp/package.json` (e.g. `0.4.0` → `0.4.1`)
2. Run:

```powershell
cd editor\vscode-vpp
.\publish.ps1 -Publish
```

---

## Checklist before first publish

- [ ] Publisher `vpp-lang` created on Marketplace
- [ ] `vsce login vpp-lang` succeeded
- [ ] `.\publish.ps1` creates `.vsix` without errors
- [ ] Installed `.vsix` locally — F5 runs `hello.vpp`
- [ ] `.\publish.ps1 -Publish` succeeds
- [ ] Extension visible at marketplace.visualstudio.com

---

## What users see after publish

1. Install VS Code
2. Install v++ compiler (release zip or `install.ps1`)
3. Extensions → search **v++** → Install
4. Open `.vpp` file → **F5**

That’s the same flow as Python or Rust — legit and familiar.
