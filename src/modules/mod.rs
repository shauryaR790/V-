use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::ast::{ImportDecl, Item, Program};
use crate::error::{VppError, VppResult};
use crate::lexer::Lexer;
use crate::parser::Parser;

#[derive(Debug, Clone, Default)]
pub struct LoadContext {
    pub std_paths: Vec<PathBuf>,
}

pub struct LoadedProgram {
    pub program: Program,
    pub source: String,
    pub entry_path: PathBuf,
}

pub fn load(entry_path: &Path) -> VppResult<LoadedProgram> {
    load_with_context(entry_path, LoadContext::default())
}

pub fn load_with_context(entry_path: &Path, ctx: LoadContext) -> VppResult<LoadedProgram> {
    let entry_path = entry_path.canonicalize().map_err(|e| VppError::Other {
        message: format!("cannot resolve path `{}`: {e}", entry_path.display()),
    })?;

    let mut visited = HashSet::new();
    let mut sources = HashMap::new();
    let program = load_recursive(&entry_path, &mut visited, &mut sources, &ctx)?;

    let source = sources
        .get(&entry_path)
        .cloned()
        .unwrap_or_default();

    Ok(LoadedProgram {
        program,
        source,
        entry_path,
    })
}

fn load_recursive(
    path: &Path,
    visited: &mut HashSet<PathBuf>,
    sources: &mut HashMap<PathBuf, String>,
    ctx: &LoadContext,
) -> VppResult<Program> {
    let path = path.canonicalize().map_err(|e| VppError::Other {
        message: format!("cannot read `{}`: {e}", path.display()),
    })?;

    if !visited.insert(path.clone()) {
        return Err(VppError::Other {
            message: format!("circular import detected at `{}`", path.display()),
        });
    }

    let text = std::fs::read_to_string(&path).map_err(|e| VppError::Other {
        message: format!("failed to read `{}`: {e}", path.display()),
    })?;
    sources.insert(path.clone(), text.clone());

    let tokens = Lexer::new(&text).tokenize()?;
    let mut program = Parser::new(text.clone(), tokens).parse_program()?;

    let imports: Vec<ImportDecl> = program
        .items
        .iter()
        .filter_map(|item| {
            if let Item::Import(import) = item {
                Some(import.clone())
            } else {
                None
            }
        })
        .collect();

    program.items.retain(|item| !matches!(item, Item::Import(_)));

    let base_dir = path.parent().unwrap_or(Path::new("."));
    for import in imports {
        let import_path = resolve_import(base_dir, &import.path, ctx)?;
        let imported = load_recursive(&import_path, visited, sources, ctx)?;
        merge_program(&mut program, imported);
    }

    visited.remove(&path);
    Ok(program)
}

fn resolve_import(base_dir: &Path, spec: &str, ctx: &LoadContext) -> VppResult<PathBuf> {
    let candidates = import_candidates(base_dir, spec, ctx);
    for path in candidates {
        if path.exists() {
            return Ok(path);
        }
    }

    Err(VppError::Other {
        message: format!(
            "import not found: `{spec}` (searched {} locations)",
            import_candidates(base_dir, spec, ctx).len()
        ),
    })
}

fn import_candidates(base_dir: &Path, spec: &str, ctx: &LoadContext) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    let file_name = if spec.ends_with(".vpp") {
        spec.to_string()
    } else {
        format!("{spec}.vpp")
    };

    paths.push(base_dir.join(&file_name));

    if spec.starts_with("std/") || spec.starts_with("std\\") {
        let rel = spec.strip_prefix("std/").or_else(|| spec.strip_prefix("std\\")).unwrap_or(spec);
        let rel_file = if rel.ends_with(".vpp") {
            rel.to_string()
        } else {
            format!("{rel}.vpp")
        };
        for std_root in &ctx.std_paths {
            paths.push(std_root.join(&rel_file));
        }
    }

    paths
}

fn merge_program(into: &mut Program, from: Program) {
    into.items.extend(from.items);
}
