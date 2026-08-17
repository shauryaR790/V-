use std::collections::HashMap;
use std::path::PathBuf;

use crate::span::Span;

#[derive(Debug, Clone)]
pub struct SymbolIndex {
    pub defs: HashMap<String, SymbolDef>,
}

#[derive(Debug, Clone)]
pub struct SymbolDef {
    pub kind: SymbolKind,
    pub file: PathBuf,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolKind {
    Function,
    Struct,
    Enum,
    Variant,
    Field,
    Variable,
    Param,
}

impl SymbolIndex {
    pub fn new() -> Self {
        Self {
            defs: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: impl Into<String>, def: SymbolDef) {
        self.defs.insert(name.into(), def);
    }

    pub fn lookup(&self, name: &str) -> Option<&SymbolDef> {
        self.defs.get(name)
    }
}

impl Default for SymbolIndex {
    fn default() -> Self {
        Self::new()
    }
}
