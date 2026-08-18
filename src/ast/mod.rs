use crate::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Import(ImportDecl),
    Struct(StructDecl),
    Enum(EnumDecl),
    Function(FnDecl),
    Test(TestDecl),
    Statement(Stmt),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImportSpec {
    /// Legacy: `import "relative/path.vpp"`
    FilePath(String),
    /// Canonical module path: `import std.io`
    Module(Vec<String>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportDecl {
    pub spec: ImportSpec,
    pub span: Span,
}

impl ImportDecl {
    pub fn legacy_path(&self) -> Option<&str> {
        match &self.spec {
            ImportSpec::FilePath(p) => Some(p.as_str()),
            ImportSpec::Module(_) => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructDecl {
    pub name: String,
    pub fields: Vec<StructField>,
    pub public: bool,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructField {
    pub name: String,
    pub ty: TypeAnn,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumDecl {
    pub name: String,
    pub variants: Vec<EnumVariant>,
    pub public: bool,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumVariant {
    pub name: String,
    pub payload: Vec<TypeAnn>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnDecl {
    pub name: String,
    pub params: Vec<Param>,
    pub ret_type: TypeAnn,
    pub body: Block,
    pub public: bool,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TestDecl {
    pub name: String,
    pub body: Block,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub name: String,
    pub ty: TypeAnn,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeAnn {
    Int,
    Float,
    Bool,
    String,
    Array(Box<TypeAnn>),
    Named(String),
    Option(Box<TypeAnn>),
    Result {
        ok: Box<TypeAnn>,
        err: Box<TypeAnn>,
    },
}

impl TypeAnn {
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "int" => Some(TypeAnn::Int),
            "float" => Some(TypeAnn::Float),
            "bool" => Some(TypeAnn::Bool),
            "string" => Some(TypeAnn::String),
            _ => None,
        }
    }

    pub fn name(&self) -> String {
        match self {
            TypeAnn::Int => "int".to_string(),
            TypeAnn::Float => "float".to_string(),
            TypeAnn::Bool => "bool".to_string(),
            TypeAnn::String => "string".to_string(),
            TypeAnn::Array(inner) => format!("array[{}]", inner.name()),
            TypeAnn::Named(n) => n.clone(),
            TypeAnn::Option(inner) => format!("Option<{}>", inner.name()),
            TypeAnn::Result { ok, err } => format!("Result<{}, {}>", ok.name(), err.name()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Let {
        name: String,
        ty: Option<TypeAnn>,
        value: Expr,
        span: Span,
    },
    Expr(Expr),
    If {
        condition: Expr,
        then_block: Block,
        else_block: Option<Block>,
        span: Span,
    },
    While {
        condition: Expr,
        body: Block,
        span: Span,
    },
    For {
        var: String,
        iter: Expr,
        body: Block,
        span: Span,
    },
    Return {
        value: Option<Expr>,
        span: Span,
    },
    Break {
        span: Span,
    },
    Continue {
        span: Span,
    },
    Match {
        scrutinee: Expr,
        arms: Vec<MatchArm>,
        span: Span,
    },
    Block(Block),
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub body: Block,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Wildcard { span: Span },
    Literal(Expr),
    Variant {
        enum_name: Option<String>,
        variant: String,
        bindings: Vec<String>,
        span: Span,
    },
    Struct {
        struct_name: Option<String>,
        fields: Vec<(String, String)>,
        span: Span,
    },
}

impl Pattern {
    pub fn span(&self) -> Span {
        match self {
            Pattern::Wildcard { span } => *span,
            Pattern::Literal(expr) => expr.span(),
            Pattern::Variant { span, .. } => *span,
            Pattern::Struct { span, .. } => *span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Int {
        value: i64,
        span: Span,
    },
    Float {
        value: f64,
        span: Span,
    },
    Bool {
        value: bool,
        span: Span,
    },
    String {
        value: String,
        span: Span,
    },
    Ident {
        name: String,
        span: Span,
    },
    Binary {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
        span: Span,
    },
    Unary {
        op: UnOp,
        expr: Box<Expr>,
        span: Span,
    },
    Call {
        name: String,
        args: Vec<Expr>,
        span: Span,
    },
    Index {
        target: Box<Expr>,
        index: Box<Expr>,
        span: Span,
    },
    Field {
        target: Box<Expr>,
        field: String,
        span: Span,
    },
    Array {
        elements: Vec<Expr>,
        span: Span,
    },
    StructLit {
        name: Option<String>,
        fields: Vec<(String, Expr)>,
        span: Span,
    },
    Range {
        start: Box<Expr>,
        end: Box<Expr>,
        span: Span,
    },
    Assign {
        name: String,
        value: Box<Expr>,
        span: Span,
    },
    Match {
        scrutinee: Box<Expr>,
        arms: Vec<MatchArm>,
        span: Span,
    },
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::Int { span, .. }
            | Expr::Float { span, .. }
            | Expr::Bool { span, .. }
            | Expr::String { span, .. }
            | Expr::Ident { span, .. }
            | Expr::Binary { span, .. }
            | Expr::Unary { span, .. }
            | Expr::Call { span, .. }
            | Expr::Index { span, .. }
            | Expr::Field { span, .. }
            | Expr::Array { span, .. }
            | Expr::StructLit { span, .. }
            | Expr::Range { span, .. }
            | Expr::Assign { span, .. }
            | Expr::Match { span, .. } => *span,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    And,
    Or,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnOp {
    Not,
    Neg,
}

impl BinOp {
    pub fn from_token(kind: &crate::lexer::TokenKind) -> Option<Self> {
        use crate::lexer::TokenKind;
        match kind {
            TokenKind::Plus => Some(BinOp::Add),
            TokenKind::Minus => Some(BinOp::Sub),
            TokenKind::Star => Some(BinOp::Mul),
            TokenKind::Slash => Some(BinOp::Div),
            TokenKind::Percent => Some(BinOp::Mod),
            TokenKind::EqEq => Some(BinOp::Eq),
            TokenKind::BangEq => Some(BinOp::NotEq),
            TokenKind::Lt => Some(BinOp::Lt),
            TokenKind::LtEq => Some(BinOp::LtEq),
            TokenKind::Gt => Some(BinOp::Gt),
            TokenKind::GtEq => Some(BinOp::GtEq),
            TokenKind::AndAnd => Some(BinOp::And),
            TokenKind::OrOr => Some(BinOp::Or),
            _ => None,
        }
    }
}

impl UnOp {
    pub fn from_token(kind: &crate::lexer::TokenKind) -> Option<Self> {
        use crate::lexer::TokenKind;
        match kind {
            TokenKind::Bang => Some(UnOp::Not),
            TokenKind::Minus => Some(UnOp::Neg),
            _ => None,
        }
    }
}
