use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::span::Span;

#[derive(Debug, Error, Diagnostic)]
pub enum VppError {
    #[error("invalid character `{ch}`")]
    #[diagnostic(code(vpp::E0001))]
    InvalidCharacter {
        ch: char,
        #[label("unexpected character")]
        span: SourceSpan,
    },

    #[error("unterminated string literal")]
    #[diagnostic(code(vpp::E0002))]
    UnterminatedString {
        #[label("string starts here")]
        span: SourceSpan,
    },

    #[error("unexpected token `{found}`")]
    #[diagnostic(code(vpp::E0003))]
    UnexpectedToken {
        found: String,
        expected: String,
        #[label("unexpected token")]
        span: SourceSpan,
        #[help("{expected}")]
        expected_help: String,
    },

    #[error("unexpected end of file")]
    #[diagnostic(code(vpp::E0004))]
    UnexpectedEof {
        expected: String,
        #[label("expected {expected} here")]
        span: SourceSpan,
    },

    #[error("type mismatch")]
    #[diagnostic(code(vpp::E0100))]
    TypeMismatch {
        expected: String,
        found: String,
        #[label("expected {expected}, found {found}")]
        span: SourceSpan,
        #[help("{help}")]
        help: String,
    },

    #[error("undefined variable `{name}`")]
    #[diagnostic(code(vpp::E0200))]
    UndefinedVariable {
        name: String,
        #[label("not found in this scope")]
        span: SourceSpan,
        #[help("did you mean to declare it with `let {name} = ...`?")]
        help: String,
    },

    #[error("undefined function `{name}`")]
    #[diagnostic(code(vpp::E0201))]
    UndefinedFunction {
        name: String,
        #[label("function not defined")]
        span: SourceSpan,
    },

    #[error("wrong number of arguments")]
    #[diagnostic(code(vpp::E0101))]
    WrongArgCount {
        name: String,
        expected: usize,
        found: usize,
        #[label("called with {found} argument(s), expected {expected}")]
        span: SourceSpan,
    },

    #[error("cannot infer type of empty array")]
    #[diagnostic(code(vpp::E0102))]
    EmptyArrayNoType {
        #[label("provide at least one element or an explicit type annotation")]
        span: SourceSpan,
    },

    #[error("array element type mismatch")]
    #[diagnostic(code(vpp::E0103))]
    ArrayElementMismatch {
        expected: String,
        found: String,
        #[label("expected {expected}, found {found}")]
        span: SourceSpan,
    },

    #[error("cannot assign to `{name}`")]
    #[diagnostic(code(vpp::E0104))]
    ImmutableAssign {
        name: String,
        #[label("`{name}` is not declared with `let` in this scope")]
        span: SourceSpan,
    },

    #[error("missing return value")]
    #[diagnostic(code(vpp::E0105))]
    MissingReturn {
        expected: String,
        #[label("function must return {expected}")]
        span: SourceSpan,
    },

    #[error("for-in loop requires an array or range")]
    #[diagnostic(code(vpp::E0106))]
    InvalidForIter {
        found: String,
        #[label("expected array or range, found {found}")]
        span: SourceSpan,
    },

    #[error("import not found: `{spec}`")]
    #[diagnostic(code(vpp::E0400))]
    ImportNotFound {
        spec: String,
        #[help("{hint}")]
        hint: String,
    },

    #[error("circular import at `{path}`")]
    #[diagnostic(code(vpp::E0401))]
    ImportCycle { path: String },

    #[error("duplicate import of module `{module}`")]
    #[diagnostic(code(vpp::E0402))]
    DuplicateImport {
        module: String,
        #[label("already imported")]
        span: SourceSpan,
    },

    #[error("unknown module `{module}`")]
    #[diagnostic(code(vpp::E0403))]
    UnknownModule {
        module: String,
        #[label("no such imported module")]
        span: SourceSpan,
    },

    #[error("module `{module}` has no export `{name}`")]
    #[diagnostic(code(vpp::E0404))]
    UnknownModuleMember {
        module: String,
        name: String,
        #[label("not exported from `{module}`")]
        span: SourceSpan,
        #[help("{help}")]
        help: String,
    },

    #[error("{message}")]
    #[diagnostic(code(vpp::E0300))]
    Codegen {
        message: String,
        #[label("codegen error")]
        span: SourceSpan,
    },

    #[error("I/O error")]
    Io {
        #[source]
        source: std::io::Error,
    },

    #[error("{message}")]
    Other {
        message: String,
    },
}

pub type VppResult<T> = Result<T, VppError>;

pub fn span_to_source(_source: &str, span: Span) -> SourceSpan {
    let len = span.end.saturating_sub(span.start).max(1);
    SourceSpan::new(span.start.into(), len)
}

impl VppError {
    pub fn with_source(self, source: impl Into<String>) -> miette::Report {
        miette::Report::new(self).with_source_code(source.into())
    }

    pub fn source_span(&self) -> Option<SourceSpan> {
        match self {
            Self::InvalidCharacter { span, .. }
            | Self::UnterminatedString { span, .. }
            | Self::UnexpectedToken { span, .. }
            | Self::UnexpectedEof { span, .. }
            | Self::TypeMismatch { span, .. }
            | Self::UndefinedVariable { span, .. }
            | Self::UndefinedFunction { span, .. }
            | Self::WrongArgCount { span, .. }
            | Self::EmptyArrayNoType { span, .. }
            | Self::ArrayElementMismatch { span, .. }
            | Self::ImmutableAssign { span, .. }
            | Self::MissingReturn { span, .. }
            | Self::InvalidForIter { span, .. }
            | Self::DuplicateImport { span, .. }
            | Self::UnknownModule { span, .. }
            | Self::UnknownModuleMember { span, .. }
            | Self::Codegen { span, .. } => Some(*span),
            _ => None,
        }
    }
}

pub fn type_mismatch(
    source: &str,
    span: Span,
    expected: &str,
    found: &str,
    help: impl Into<String>,
) -> VppError {
    VppError::TypeMismatch {
        expected: expected.to_string(),
        found: found.to_string(),
        span: span_to_source(source, span),
        help: help.into(),
    }
}
