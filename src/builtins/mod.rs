//! Authoritative builtin definitions for v++.
//!
//! Type checker, interpreter, and native codegen consult this module.

use crate::types::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuiltinKind {
    Print,
    Len,
    Assert,
    AssertEq,
    ReadFile,
    WriteFile,
    FileExists,
    JsonParse,
    JsonStringify,
    ProcessRun,
    Some,
    Ok,
    Err,
}

#[derive(Debug, Clone)]
pub struct BuiltinSig {
    pub name: &'static str,
    pub kind: BuiltinKind,
    pub min_args: usize,
    pub max_args: Option<usize>,
}

impl BuiltinSig {
    pub const fn fixed(name: &'static str, kind: BuiltinKind, argc: usize) -> Self {
        Self {
            name,
            kind,
            min_args: argc,
            max_args: Some(argc),
        }
    }

    pub const fn variadic(name: &'static str, kind: BuiltinKind) -> Self {
        Self {
            name,
            kind,
            min_args: 0,
            max_args: None,
        }
    }
}

static BUILTINS: [BuiltinSig; 13] = [
    BuiltinSig::variadic("print", BuiltinKind::Print),
    BuiltinSig::fixed("len", BuiltinKind::Len, 1),
    BuiltinSig::fixed("assert", BuiltinKind::Assert, 1),
    BuiltinSig::fixed("assert_eq", BuiltinKind::AssertEq, 2),
    BuiltinSig::fixed("read_file", BuiltinKind::ReadFile, 1),
    BuiltinSig::fixed("write_file", BuiltinKind::WriteFile, 2),
    BuiltinSig::fixed("file_exists", BuiltinKind::FileExists, 1),
    BuiltinSig::fixed("json_parse", BuiltinKind::JsonParse, 1),
    BuiltinSig::fixed("json_stringify", BuiltinKind::JsonStringify, 1),
    BuiltinSig::fixed("process_run", BuiltinKind::ProcessRun, 1),
    BuiltinSig::fixed("Some", BuiltinKind::Some, 1),
    BuiltinSig::fixed("Ok", BuiltinKind::Ok, 1),
    BuiltinSig::fixed("Err", BuiltinKind::Err, 1),
];

pub fn all() -> &'static [BuiltinSig] {
    &BUILTINS
}

pub fn lookup(name: &str) -> Option<&'static BuiltinSig> {
    all().iter().find(|b| b.name == name)
}

pub fn is_printable(ty: &Type) -> bool {
    matches!(
        ty,
        Type::Int
            | Type::Float
            | Type::Bool
            | Type::String
            | Type::Struct { .. }
            | Type::Enum { .. }
            | Type::Option(_)
            | Type::Result { .. }
    )
}

pub fn is_builtin(name: &str) -> bool {
    lookup(name).is_some()
}
