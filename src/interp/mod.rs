use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::{BinOp, UnOp};
use crate::error::{VppError, VppResult};
use crate::types::{
    FunctionInfo, TypedExpr, TypedPattern, TypedProgram, TypedStmt, Type,
};

#[derive(Debug, Clone)]
enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(Rc<String>),
    Array(Rc<Vec<Value>>),
    Struct {
        name: String,
        fields: HashMap<String, Value>,
    },
    Variant {
        enum_name: String,
        variant: String,
        payload: Vec<Value>,
    },
    Void,
}

impl Value {
    fn as_int(&self) -> VppResult<i64> {
        match self {
            Value::Int(n) => Ok(*n),
            other => Err(VppError::Other {
                message: format!("expected int at runtime, found {other:?}"),
            }),
        }
    }

    fn as_bool(&self) -> VppResult<bool> {
        match self {
            Value::Bool(b) => Ok(*b),
            other => Err(VppError::Other {
                message: format!("expected bool at runtime, found {other:?}"),
            }),
        }
    }

    fn display_string(&self) -> String {
        match self {
            Value::Int(n) => n.to_string(),
            Value::Float(n) => n.to_string(),
            Value::Bool(b) => if *b { "true".to_string() } else { "false".to_string() },
            Value::String(s) => s.to_string(),
            Value::Struct { name, fields } => {
                let inner: Vec<String> = fields
                    .iter()
                    .map(|(k, v)| format!("{k}: {}", v.display_string()))
                    .collect();
                format!("{name} {{ {} }}", inner.join(", "))
            }
            Value::Variant {
                enum_name,
                variant,
                payload,
            } => {
                if payload.is_empty() {
                    if enum_name == "Option" || enum_name == "Result" {
                        variant.clone()
                    } else {
                        format!("{enum_name}.{variant}")
                    }
                } else if payload.len() == 1 {
                    format!("{variant}({})", payload[0].display_string())
                } else {
                    format!(
                        "{variant}({})",
                        payload
                            .iter()
                            .map(Value::display_string)
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                }
            }
            Value::Array(items) => {
                let inner: Vec<String> = items.iter().map(Value::display_string).collect();
                format!("[{}]", inner.join(", "))
            }
            Value::Void => "void".to_string(),
        }
    }
}

struct Interpreter {
    functions: HashMap<String, FunctionInfo>,
    scopes: Vec<HashMap<String, Value>>,
    return_value: Option<Value>,
    returning: bool,
    breaking: bool,
    continuing: bool,
}

pub fn run(program: &TypedProgram) -> VppResult<()> {
    let mut interp = Interpreter::new(program.functions.clone());

    for stmt in &program.top_level {
        interp.exec_stmt(stmt)?;
        if interp.returning {
            break;
        }
    }
    Ok(())
}

pub fn run_tests(program: &TypedProgram) -> VppResult<usize> {
    if program.tests.is_empty() {
        return Err(VppError::Other {
            message: "no `test` blocks found in this file".to_string(),
        });
    }

    let mut passed = 0usize;
    for test in &program.tests {
        print!("  {} ... ", test.name);
        let mut interp = Interpreter::new(program.functions.clone());
        match interp.exec_block(&test.body) {
            Ok(()) => {
                println!("ok");
                passed += 1;
            }
            Err(e) => {
                println!("FAILED");
                return Err(VppError::Other {
                    message: format!("test `{}`: {e}", test.name),
                });
            }
        }
    }
    Ok(passed)
}

impl Interpreter {
    fn new(functions: HashMap<String, FunctionInfo>) -> Self {
        Self {
            functions,
            scopes: vec![HashMap::new()],
            return_value: None,
            returning: false,
            breaking: false,
            continuing: false,
        }
    }
    fn exec_stmt(&mut self, stmt: &TypedStmt) -> VppResult<()> {
        match stmt {
            TypedStmt::Let { name, value, .. } => {
                let val = self.eval_expr(value)?;
                self.define(name, val);
            }
            TypedStmt::Expr(expr) => {
                self.eval_expr(expr)?;
            }
            TypedStmt::If {
                condition,
                then_block,
                else_block,
            } => {
                if self.eval_expr(condition)?.as_bool()? {
                    self.exec_block(then_block)?;
                } else if let Some(block) = else_block {
                    self.exec_block(block)?;
                }
            }
            TypedStmt::While { condition, body } => {
                while self.eval_expr(condition)?.as_bool()? {
                    self.exec_block(body)?;
                    if self.returning {
                        break;
                    }
                    if self.breaking {
                        self.breaking = false;
                        break;
                    }
                    if self.continuing {
                        self.continuing = false;
                        continue;
                    }
                }
            }
            TypedStmt::ForInt { var, start, end, body } => {
                let mut i = *start;
                while i < *end {
                    self.push_scope();
                    self.define(var, Value::Int(i));
                    self.exec_block(body)?;
                    self.pop_scope();
                    if self.returning {
                        break;
                    }
                    if self.breaking {
                        self.breaking = false;
                        break;
                    }
                    if self.continuing {
                        self.continuing = false;
                        i += 1;
                        continue;
                    }
                    i += 1;
                }
            }
            TypedStmt::ForArray { var, array, body, .. } => {
                let arr = self.eval_expr(array)?;
                if let Value::Array(items) = arr {
                    for item in items.iter() {
                        self.push_scope();
                        self.define(var, item.clone());
                        self.exec_block(body)?;
                        self.pop_scope();
                        if self.returning {
                            break;
                        }
                        if self.breaking {
                            self.breaking = false;
                            break;
                        }
                        if self.continuing {
                            self.continuing = false;
                            continue;
                        }
                    }
                }
            }
            TypedStmt::Return { value } => {
                self.return_value = if let Some(expr) = value {
                    Some(self.eval_expr(expr)?)
                } else {
                    Some(Value::Void)
                };
                self.returning = true;
            }
            TypedStmt::Match { scrutinee, arms, .. } => {
                let val = self.eval_expr(scrutinee)?;
                for arm in arms {
                    if let Some(bindings) = self.match_pattern(&arm.pattern, &val)? {
                        self.push_scope();
                        for (name, bound) in bindings {
                            self.define(&name, bound);
                        }
                        self.exec_block(&arm.body)?;
                        self.pop_scope();
                        if self.returning {
                            break;
                        }
                        break;
                    }
                }
            }
            TypedStmt::Block(stmts) => {
                self.exec_block(stmts)?;
            }
            TypedStmt::Break => {
                self.breaking = true;
            }
            TypedStmt::Continue => {
                self.continuing = true;
            }
        }
        Ok(())
    }

    fn exec_block(&mut self, stmts: &[TypedStmt]) -> VppResult<()> {
        self.push_scope();
        for stmt in stmts {
            self.exec_stmt(stmt)?;
            if self.returning || self.breaking || self.continuing {
                break;
            }
        }
        self.pop_scope();
        Ok(())
    }

    fn match_pattern(
        &self,
        pattern: &TypedPattern,
        value: &Value,
    ) -> VppResult<Option<Vec<(String, Value)>>> {
        match pattern {
            TypedPattern::Wildcard => Ok(Some(Vec::new())),
            TypedPattern::Literal(expr) => {
                let lit = self.eval_expr_standalone(expr)?;
                Ok((lit == *value).then_some(Vec::new()))
            }
            TypedPattern::Variant {
                enum_name,
                variant,
                bindings,
                ..
            } => {
                if let Value::Variant {
                    enum_name: en,
                    variant: vn,
                    payload,
                } = value
                {
                    if en == enum_name && vn == variant && payload.len() == bindings.len() {
                        let mut out = Vec::new();
                        for (name, val) in bindings.iter().zip(payload.iter()) {
                            out.push((name.clone(), val.clone()));
                        }
                        return Ok(Some(out));
                    }
                }
                Ok(None)
            }
            TypedPattern::Struct {
                struct_name,
                fields,
            } => {
                if let Value::Struct { name, fields: vals } = value {
                    if name != struct_name {
                        return Ok(None);
                    }
                    let mut out = Vec::new();
                    for (field, binding, _) in fields {
                        if let Some(val) = vals.get(field) {
                            out.push((binding.clone(), val.clone()));
                        } else {
                            return Ok(None);
                        }
                    }
                    Ok(Some(out))
                } else {
                    Ok(None)
                }
            }
        }
    }

    fn eval_expr_standalone(&self, expr: &TypedExpr) -> VppResult<Value> {
        match expr {
            TypedExpr::Int(n) => Ok(Value::Int(*n)),
            TypedExpr::Float(n) => Ok(Value::Float(*n)),
            TypedExpr::Bool(b) => Ok(Value::Bool(*b)),
            TypedExpr::String(s) => Ok(Value::String(Rc::new(s.clone()))),
            _ => Err(VppError::Other {
                message: "complex literal patterns not supported yet".to_string(),
            }),
        }
    }

    fn call_function(&mut self, name: &str, args: &[TypedExpr]) -> VppResult<Value> {
        if name == "print" {
            for arg in args {
                let val = self.eval_expr(arg)?;
                println!("{}", val.display_string());
            }
            return Ok(Value::Void);
        }

        if name == "len" {
            let val = self.eval_expr(&args[0])?;
            let len = match val {
                Value::String(s) => s.len() as i64,
                Value::Array(items) => items.len() as i64,
                other => {
                    return Err(VppError::Other {
                        message: format!("len() expects array or string, found {other:?}"),
                    });
                }
            };
            return Ok(Value::Int(len));
        }

        if name == "assert" {
            let cond = self.eval_expr(&args[0])?.as_bool()?;
            if !cond {
                return Err(VppError::Other {
                    message: "assertion failed".to_string(),
                });
            }
            return Ok(Value::Void);
        }

        if name == "assert_eq" {
            let left = self.eval_expr(&args[0])?;
            let right = self.eval_expr(&args[1])?;
            if left != right {
                return Err(VppError::Other {
                    message: format!(
                        "assertion failed: {} != {}",
                        left.display_string(),
                        right.display_string()
                    ),
                });
            }
            return Ok(Value::Void);
        }

        let func = self
            .functions
            .get(name)
            .cloned()
            .ok_or_else(|| VppError::Other {
                message: format!("undefined function `{name}`"),
            })?;

        let mut arg_values = Vec::new();
        for arg in args {
            arg_values.push(self.eval_expr(arg)?);
        }

        self.push_scope();
        for ((param, _), value) in func.params.iter().zip(arg_values) {
            self.define(param, value);
        }

        let saved_returning = self.returning;
        let saved_return = self.return_value.take();
        self.returning = false;

        for stmt in &func.body {
            self.exec_stmt(stmt)?;
            if self.returning {
                break;
            }
        }

        let result = if self.returning {
            self.return_value.take().unwrap_or(Value::Void)
        } else if func.ret == Type::Void {
            Value::Void
        } else {
            Value::Int(0)
        };

        self.returning = saved_returning;
        self.return_value = saved_return;
        self.pop_scope();

        Ok(result)
    }

    fn eval_expr(&mut self, expr: &TypedExpr) -> VppResult<Value> {
        match expr {
            TypedExpr::Int(n) => Ok(Value::Int(*n)),
            TypedExpr::Float(n) => Ok(Value::Float(*n)),
            TypedExpr::Bool(b) => Ok(Value::Bool(*b)),
            TypedExpr::String(s) => Ok(Value::String(Rc::new(s.clone()))),
            TypedExpr::Ident { name, .. } => self.lookup(name).ok_or_else(|| VppError::Other {
                message: format!("undefined variable `{name}` at runtime"),
            }),
            TypedExpr::Binary { op, left, right, .. } => {
                let l = self.eval_expr(left)?;
                let r = self.eval_expr(right)?;
                self.eval_binary(*op, l, r)
            }
            TypedExpr::Unary { op, expr, .. } => {
                let val = self.eval_expr(expr)?;
                self.eval_unary(*op, val)
            }
            TypedExpr::Call { name, args, .. } => self.call_function(name, args),
            TypedExpr::Index { target, index, .. } => {
                let target_val = self.eval_expr(target)?;
                let idx = self.eval_expr(index)?.as_int()? as usize;
                match target_val {
                    Value::Array(items) => items.get(idx).cloned().ok_or_else(|| VppError::Other {
                        message: format!("array index out of bounds: {idx}"),
                    }),
                    other => Err(VppError::Other {
                        message: format!("cannot index non-array value {other:?}"),
                    }),
                }
            }
            TypedExpr::Field { target, field, .. } => {
                let target_val = self.eval_expr(target)?;
                match target_val {
                    Value::Struct { fields, .. } => fields.get(field).cloned().ok_or_else(|| {
                        VppError::Other {
                            message: format!("struct has no field `{field}`"),
                        }
                    }),
                    other => Err(VppError::Other {
                        message: format!("field access on non-struct value {other:?}"),
                    }),
                }
            }
            TypedExpr::Array { elements, .. } => {
                let mut items = Vec::new();
                for elem in elements {
                    items.push(self.eval_expr(elem)?);
                }
                Ok(Value::Array(Rc::new(items)))
            }
            TypedExpr::StructLit { name, fields, .. } => {
                let mut map = HashMap::new();
                for (field, expr) in fields {
                    map.insert(field.clone(), self.eval_expr(expr)?);
                }
                Ok(Value::Struct {
                    name: name.clone(),
                    fields: map,
                })
            }
            TypedExpr::Variant {
                enum_name,
                variant,
                payload,
                ..
            } => {
                let mut vals = Vec::new();
                for expr in payload {
                    vals.push(self.eval_expr(expr)?);
                }
                Ok(Value::Variant {
                    enum_name: enum_name.clone(),
                    variant: variant.clone(),
                    payload: vals,
                })
            }
            TypedExpr::Assign { name, value, .. } => {
                let val = self.eval_expr(value)?;
                if self.assign(name, val.clone()) {
                    Ok(val)
                } else {
                    Err(VppError::Other {
                        message: format!("undefined variable `{name}` for assignment"),
                    })
                }
            }
            TypedExpr::Match { scrutinee, arms, .. } => {
                let val = self.eval_expr(scrutinee)?;
                for arm in arms {
                    if let Some(bindings) = self.match_pattern(&arm.pattern, &val)? {
                        self.push_scope();
                        for (name, bound) in bindings {
                            self.define(&name, bound);
                        }
                        let mut last = Value::Void;
                        for stmt in &arm.body {
                            self.exec_stmt(stmt)?;
                            if let TypedStmt::Expr(expr) = stmt {
                                last = self.eval_expr(expr)?;
                            }
                            if self.returning {
                                break;
                            }
                        }
                        self.pop_scope();
                        return Ok(last);
                    }
                }
                Err(VppError::Other {
                    message: "non-exhaustive match at runtime".to_string(),
                })
            }
        }
    }

    fn eval_binary(&self, op: BinOp, left: Value, right: Value) -> VppResult<Value> {
        match op {
            BinOp::Add => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
                (Value::String(a), Value::String(b)) => {
                    Ok(Value::String(Rc::new(format!("{}{}", a, b))))
                }
                (a, b) => Err(VppError::Other {
                    message: format!("invalid operands for +: {a:?} and {b:?}"),
                }),
            },
            BinOp::Sub => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
                (a, b) => Err(VppError::Other {
                    message: format!("invalid operands for -: {a:?} and {b:?}"),
                }),
            },
            BinOp::Mul => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
                (a, b) => Err(VppError::Other {
                    message: format!("invalid operands for *: {a:?} and {b:?}"),
                }),
            },
            BinOp::Div => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a / b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),
                (a, b) => Err(VppError::Other {
                    message: format!("invalid operands for /: {a:?} and {b:?}"),
                }),
            },
            BinOp::Mod => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a % b)),
                (a, b) => Err(VppError::Other {
                    message: format!("invalid operands for %: {a:?} and {b:?}"),
                }),
            },
            BinOp::Eq => Ok(Value::Bool(left == right)),
            BinOp::NotEq => Ok(Value::Bool(left != right)),
            BinOp::Lt => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a < b)),
                (a, b) => Err(VppError::Other {
                    message: format!("invalid comparison operands: {a:?} and {b:?}"),
                }),
            },
            BinOp::LtEq => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a <= b)),
                (a, b) => Err(VppError::Other {
                    message: format!("invalid comparison operands: {a:?} and {b:?}"),
                }),
            },
            BinOp::Gt => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a > b)),
                (a, b) => Err(VppError::Other {
                    message: format!("invalid comparison operands: {a:?} and {b:?}"),
                }),
            },
            BinOp::GtEq => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a >= b)),
                (a, b) => Err(VppError::Other {
                    message: format!("invalid comparison operands: {a:?} and {b:?}"),
                }),
            },
            BinOp::And => Ok(Value::Bool(left.as_bool()? && right.as_bool()?)),
            BinOp::Or => Ok(Value::Bool(left.as_bool()? || right.as_bool()?)),
        }
    }

    fn eval_unary(&self, op: UnOp, val: Value) -> VppResult<Value> {
        match op {
            UnOp::Not => Ok(Value::Bool(!val.as_bool()?)),
            UnOp::Neg => match val {
                Value::Int(n) => Ok(Value::Int(-n)),
                Value::Float(n) => Ok(Value::Float(-n)),
                other => Err(VppError::Other {
                    message: format!("cannot negate {other:?}"),
                }),
            },
        }
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn define(&mut self, name: &str, value: Value) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), value);
        }
    }

    fn assign(&mut self, name: &str, value: Value) -> bool {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), value);
                return true;
            }
        }
        false
    }

    fn lookup(&self, name: &str) -> Option<Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(val) = scope.get(name) {
                return Some(val.clone());
            }
        }
        None
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Array(a), Value::Array(b)) => a == b,
            (Value::Struct { name: n1, fields: f1 }, Value::Struct { name: n2, fields: f2 }) => {
                n1 == n2 && f1 == f2
            }
            (
                Value::Variant {
                    enum_name: e1,
                    variant: v1,
                    payload: p1,
                },
                Value::Variant {
                    enum_name: e2,
                    variant: v2,
                    payload: p2,
                },
            ) => e1 == e2 && v1 == v2 && p1 == p2,
            (Value::Void, Value::Void) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::driver::check;

    #[test]
    fn runs_hello() {
        let source = include_str!("../../examples/hello.vpp");
        let typed = check(source).unwrap();
        run(&typed).unwrap();
    }

    #[test]
    fn runs_structs() {
        let source = include_str!("../../examples/structs.vpp");
        run(&check(source).unwrap()).unwrap();
    }

    #[test]
    fn ok_val_function_has_body() {
        let source = include_str!("../../examples/match_test.vpp");
        let typed = check(source).unwrap();
        let func = typed.functions.get("ok_val").expect("ok_val missing");
        assert!(!func.body.is_empty(), "function body should not be empty");
        assert!(matches!(func.ret, Type::Result { .. }));
        assert!(matches!(
            &func.body[0],
            TypedStmt::Return {
                value: Some(TypedExpr::Variant { .. }),
                ..
            }
        ));
        assert_eq!(func.body.len(), 1);
    }

    #[test]
    fn calls_ok_val() {
        let source = include_str!("../../examples/match_test.vpp");
        let typed = check(source).unwrap();
        let call = TypedExpr::Call {
            name: "ok_val".to_string(),
            args: vec![],
            ty: typed.functions["ok_val"].ret.clone(),
        };
        let mut interp = Interpreter::new(typed.functions.clone());
        let val = interp.eval_expr(&call).unwrap();
        assert!(matches!(val, Value::Variant { .. }));
    }
}
