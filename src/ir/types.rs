use std::collections::HashMap;

use crate::types::Type;

#[derive(Debug, Clone, PartialEq)]
pub enum IrType {
    Int,
    Float,
    Bool,
    String,
    Array(Box<IrType>),
    Struct {
        name: String,
    },
    Enum {
        name: String,
    },
    Void,
    Unknown,
}

impl IrType {
    pub fn from_type(ty: &Type) -> Self {
        match ty {
            Type::Int => IrType::Int,
            Type::Float => IrType::Float,
            Type::Bool => IrType::Bool,
            Type::String => IrType::String,
            Type::Array(inner) => IrType::Array(Box::new(IrType::from_type(inner))),
            Type::Struct { name, .. } => IrType::Struct { name: name.clone() },
            Type::Enum { name, .. } => IrType::Enum { name: name.clone() },
            Type::Option(inner) => IrType::Enum {
                name: format!("Option<{}>", inner.name()),
            },
            Type::Result { ok, err } => IrType::Enum {
                name: format!("Result<{}, {}>", ok.name(), err.name()),
            },
            Type::Void => IrType::Void,
            _ => IrType::Unknown,
        }
    }

    pub fn enum_key(&self) -> Option<&str> {
        match self {
            IrType::Enum { name } => Some(name),
            _ => None,
        }
    }

    pub fn struct_name(&self) -> Option<&str> {
        match self {
            IrType::Struct { name } => Some(name),
            _ => None,
        }
    }

    pub fn name(&self) -> String {
        match self {
            IrType::Int => "int".to_string(),
            IrType::Float => "float".to_string(),
            IrType::Bool => "bool".to_string(),
            IrType::String => "string".to_string(),
            IrType::Array(inner) => format!("array<{}>", inner.name()),
            IrType::Struct { name } => name.clone(),
            IrType::Enum { name } => name.clone(),
            IrType::Void => "void".to_string(),
            IrType::Unknown => "unknown".to_string(),
        }
    }

    pub fn is_heap(&self) -> bool {
        matches!(self, IrType::String | IrType::Array(_))
    }

    pub fn elem_type(&self) -> &IrType {
        match self {
            IrType::Array(inner) => inner.as_ref(),
            other => other,
        }
    }

    pub fn is_array(&self) -> bool {
        matches!(self, IrType::Array(_))
    }

    pub fn is_struct(&self) -> bool {
        matches!(self, IrType::Struct { .. })
    }

    pub fn is_enum(&self) -> bool {
        matches!(self, IrType::Enum { .. })
    }
}

#[derive(Debug, Clone)]
pub struct IrModule {
    pub functions: Vec<IrFunction>,
    pub top_level: Vec<IrStmt>,
    pub struct_defs: HashMap<String, Vec<(String, IrType)>>,
    pub enum_defs: HashMap<String, Vec<(String, Vec<IrType>)>>,
}

#[derive(Debug, Clone)]
pub struct IrFunction {
    pub name: String,
    pub params: Vec<(String, IrType)>,
    pub ret: IrType,
    pub body: Vec<IrStmt>,
}

#[derive(Debug, Clone)]
pub enum IrStmt {
    Let {
        name: String,
        ty: IrType,
        value: IrValue,
    },
    Expr(IrValue),
    If {
        cond: IrValue,
        then_body: Vec<IrStmt>,
        else_body: Option<Vec<IrStmt>>,
    },
    While {
        cond: IrValue,
        body: Vec<IrStmt>,
    },
    ForInt {
        var: String,
        start: i64,
        end: i64,
        body: Vec<IrStmt>,
    },
    ForArray {
        var: String,
        array: IrValue,
        elem_ty: IrType,
        body: Vec<IrStmt>,
    },
    Return {
        value: Option<IrValue>,
    },
    Block(Vec<IrStmt>),
    Break,
    Continue,
    Match {
        scrutinee: IrValue,
        arms: Vec<IrMatchArm>,
        ty: IrType,
    },
}

#[derive(Debug, Clone)]
pub struct IrMatchArm {
    pub pattern: IrPattern,
    pub body: Vec<IrStmt>,
}

#[derive(Debug, Clone)]
pub enum IrPattern {
    Wildcard,
    Literal(IrValue),
    Variant {
        enum_name: String,
        variant: String,
        bindings: Vec<String>,
        payload_types: Vec<IrType>,
    },
    Struct {
        struct_name: String,
        fields: Vec<(String, String, IrType)>,
    },
}

#[derive(Debug, Clone)]
pub enum IrValue {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Local {
        name: String,
        ty: IrType,
    },
    Binary {
        op: crate::ast::BinOp,
        left: Box<IrValue>,
        right: Box<IrValue>,
        ty: IrType,
    },
    Unary {
        op: crate::ast::UnOp,
        expr: Box<IrValue>,
        ty: IrType,
    },
    Call {
        name: String,
        args: Vec<IrValue>,
        ty: IrType,
    },
    Index {
        target: Box<IrValue>,
        index: Box<IrValue>,
        ty: IrType,
    },
    Field {
        target: Box<IrValue>,
        field: String,
        ty: IrType,
    },
    Array {
        elements: Vec<IrValue>,
        ty: IrType,
    },
    StructLit {
        name: String,
        fields: Vec<(String, IrValue)>,
        ty: IrType,
    },
    Variant {
        enum_name: String,
        variant: String,
        payload: Vec<IrValue>,
        ty: IrType,
    },
    Assign {
        name: String,
        value: Box<IrValue>,
        ty: IrType,
    },
    Match {
        scrutinee: Box<IrValue>,
        arms: Vec<IrMatchArm>,
        ty: IrType,
    },
}

impl IrValue {
    pub fn ty(&self) -> IrType {
        match self {
            IrValue::Int(_) => IrType::Int,
            IrValue::Float(_) => IrType::Float,
            IrValue::Bool(_) => IrType::Bool,
            IrValue::String(_) => IrType::String,
            IrValue::Local { ty, .. } => ty.clone(),
            IrValue::Binary { ty, .. }
            | IrValue::Unary { ty, .. }
            | IrValue::Call { ty, .. }
            | IrValue::Index { ty, .. }
            | IrValue::Field { ty, .. }
            | IrValue::Array { ty, .. }
            | IrValue::StructLit { ty, .. }
            | IrValue::Variant { ty, .. }
            | IrValue::Assign { ty, .. }
            | IrValue::Match { ty, .. } => ty.clone(),
        }
    }
}
