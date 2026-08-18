# Hello, v++

Create `hello.vpp`:

```vpp
fn main() -> int {
    print("Hello from v++!")
    return 0
}
```

Run it:

```powershell
vpp run hello.vpp
```

Type-check without running:

```powershell
vpp check hello.vpp
```

Compile to a native executable:

```powershell
vpp build hello.vpp -o hello.exe
.\hello.exe
```

## What you just used

- **`vpp run`** — interpreter (fast iteration, no compile step)
- **`vpp check`** — type checker only
- **`vpp build`** — LLVM + clang → `.exe`

Next: [Your first project](first-project.md).
