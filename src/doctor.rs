//! Environment diagnostics for v++ tooling.

use std::path::Path;
use std::process::Command;

use crate::error::{VppError, VppResult};

pub fn run_doctor(project_root: Option<&Path>) -> VppResult<()> {
    println!("v++ doctor");
    println!("==========");
    println!();

    check_rust()?;
    check_llvm()?;
    check_git()?;
    if let Some(root) = project_root {
        check_project(root)?;
    }

    println!();
    println!("Doctor checks complete.");
    Ok(())
}

fn check_rust() -> VppResult<()> {
    print!("Rust toolchain ... ");
    match Command::new("rustc").arg("--version").output() {
        Ok(out) if out.status.success() => {
            println!("ok ({})", String::from_utf8_lossy(&out.stdout).trim());
            Ok(())
        }
        _ => {
            println!("MISSING");
            Err(VppError::Other {
                message: "rustc not found; install Rust from https://rustup.rs".to_string(),
            })
        }
    }
}

fn check_llvm() -> VppResult<()> {
    print!("LLVM (clang) ... ");
    match Command::new("clang").arg("--version").output() {
        Ok(out) if out.status.success() => {
            let line = String::from_utf8_lossy(&out.stdout)
                .lines()
                .next()
                .unwrap_or("clang")
                .to_string();
            println!("ok ({line})");
            Ok(())
        }
        _ => {
            println!("MISSING (native build disabled)");
            println!("  Install LLVM/Clang and set LLVM_SYS_221_PREFIX for codegen.");
            Ok(())
        }
    }
}

fn check_git() -> VppResult<()> {
    print!("git ... ");
    match Command::new("git").arg("--version").output() {
        Ok(out) if out.status.success() => {
            println!("ok ({})", String::from_utf8_lossy(&out.stdout).trim());
            Ok(())
        }
        _ => {
            println!("MISSING (git dependencies unavailable)");
            Ok(())
        }
    }
}

fn check_project(root: &Path) -> VppResult<()> {
    print!("vpp.toml ... ");
    let manifest = crate::project::load_manifest(root)?;
    println!("ok ({}, v{})", manifest.name, manifest.version);

    print!("entry `{}` ... ", manifest.entry.display());
    let entry = root.join(&manifest.entry);
    if entry.exists() {
        println!("ok");
    } else {
        println!("MISSING");
        return Err(VppError::Other {
            message: format!("entry file `{}` not found", entry.display()),
        });
    }

    if root.join("vpp.lock").exists() {
        println!("vpp.lock ... ok");
    } else if !manifest.dependencies.is_empty() {
        println!("vpp.lock ... missing (run `vpp update`)");
    }

    Ok(())
}
