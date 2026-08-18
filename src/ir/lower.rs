use std::collections::HashMap;

use crate::error::VppResult;
use crate::types::{
    FunctionInfo, TypedExpr, TypedMatchArm, TypedPattern, TypedProgram, TypedStmt, Type,
};

use super::types::*;

pub fn lower_program(program: &TypedProgram) -> VppResult<IrModule> {
    let struct_defs = program
        .structs
        .iter()
        .map(|(name, info)| {
            let fields = info
                .fields
                .iter()
                .map(|(fname, ty)| (fname.clone(), IrType::from_type(ty)))
                .collect();
            (name.clone(), fields)
        })
        .collect();

    let mut enum_defs: HashMap<String, Vec<(String, Vec<IrType>)>> = HashMap::new();
    for (name, info) in &program.enums {
        let mut variants: Vec<_> = info
            .variants
            .iter()
            .map(|(v, pts)| {
                (
                    v.clone(),
                    pts.iter().map(IrType::from_type).collect::<Vec<_>>(),
                )
            })
            .collect();
        variants.sort_by(|a, b| a.0.cmp(&b.0));
        enum_defs.insert(name.clone(), variants);
    }
    // Built-in Option/Result layouts are synthesized in codegen from typed uses.

    let functions = program
        .functions
        .values()
        .map(lower_function)
        .collect::<VppResult<Vec<_>>>()?;

    let top_level = lower_stmts(&program.top_level)?;

    Ok(IrModule {
        functions,
        top_level,
        struct_defs,
        enum_defs,
    })
}

fn lower_function(func: &FunctionInfo) -> VppResult<IrFunction> {
    Ok(IrFunction {
        name: func.name.clone(),
        params: func
            .params
            .iter()
            .map(|(n, t)| (n.clone(), IrType::from_type(t)))
            .collect(),
        ret: IrType::from_type(&func.ret),
        body: lower_stmts(&func.body)?,
    })
}

fn lower_stmts(stmts: &[TypedStmt]) -> VppResult<Vec<IrStmt>> {
    stmts.iter().map(lower_stmt).collect()
}

fn lower_stmt(stmt: &TypedStmt) -> VppResult<IrStmt> {
    match stmt {
        TypedStmt::Let { name, ty, value, .. } => Ok(IrStmt::Let {
            name: name.clone(),
            ty: IrType::from_type(ty),
            value: lower_expr(value)?,
        }),
        TypedStmt::Expr(expr) => Ok(IrStmt::Expr(lower_expr(expr)?)),
        TypedStmt::If {
            condition,
            then_block,
            else_block,
        } => Ok(IrStmt::If {
            cond: lower_expr(condition)?,
            then_body: lower_stmts(then_block)?,
            else_body: match else_block {
                Some(stmts) => Some(lower_stmts(stmts)?),
                None => None,
            },
        }),
        TypedStmt::While { condition, body } => Ok(IrStmt::While {
            cond: lower_expr(condition)?,
            body: lower_stmts(body)?,
        }),
        TypedStmt::ForInt { var, start, end, body } => Ok(IrStmt::ForInt {
            var: var.clone(),
            start: *start,
            end: *end,
            body: lower_stmts(body)?,
        }),
        TypedStmt::ForArray { var, array, elem_ty, body } => Ok(IrStmt::ForArray {
            var: var.clone(),
            array: lower_expr(array)?,
            elem_ty: IrType::from_type(elem_ty),
            body: lower_stmts(body)?,
        }),
        TypedStmt::Return { value } => Ok(IrStmt::Return {
            value: match value {
                Some(v) => Some(lower_expr(v)?),
                None => None,
            },
        }),
        TypedStmt::Block(stmts) => Ok(IrStmt::Block(lower_stmts(stmts)?)),
        TypedStmt::Break => Ok(IrStmt::Break),
        TypedStmt::Continue => Ok(IrStmt::Continue),
        TypedStmt::Match { scrutinee, arms, ty } => Ok(IrStmt::Match {
            scrutinee: lower_expr(scrutinee)?,
            arms: arms.iter().map(lower_match_arm).collect::<VppResult<_>>()?,
            ty: IrType::from_type(ty),
        }),
    }
}

fn lower_match_arm(arm: &TypedMatchArm) -> VppResult<IrMatchArm> {
    Ok(IrMatchArm {
        pattern: lower_pattern(&arm.pattern)?,
        body: lower_stmts(&arm.body)?,
    })
}

fn lower_pattern(pattern: &TypedPattern) -> VppResult<IrPattern> {
    match pattern {
        TypedPattern::Wildcard => Ok(IrPattern::Wildcard),
        TypedPattern::Literal(expr) => Ok(IrPattern::Literal(lower_expr(expr)?)),
        TypedPattern::Variant {
            enum_name,
            variant,
            bindings,
            payload_types,
        } => Ok(IrPattern::Variant {
            enum_name: enum_name.clone(),
            variant: variant.clone(),
            bindings: bindings.clone(),
            payload_types: payload_types.iter().map(IrType::from_type).collect(),
        }),
        TypedPattern::Struct {
            struct_name,
            fields,
        } => Ok(IrPattern::Struct {
            struct_name: struct_name.clone(),
            fields: fields
                .iter()
                .map(|(f, b, t)| (f.clone(), b.clone(), IrType::from_type(t)))
                .collect(),
        }),
    }
}

fn lower_expr(expr: &TypedExpr) -> VppResult<IrValue> {
    match expr {
        TypedExpr::Int(v) => Ok(IrValue::Int(*v)),
        TypedExpr::Float(v) => Ok(IrValue::Float(*v)),
        TypedExpr::Bool(v) => Ok(IrValue::Bool(*v)),
        TypedExpr::String(s) => Ok(IrValue::String(s.clone())),
        TypedExpr::Ident { name, ty, .. } => Ok(IrValue::Local {
            name: name.clone(),
            ty: IrType::from_type(ty),
        }),
        TypedExpr::Binary { op, left, right, ty } => Ok(IrValue::Binary {
            op: *op,
            left: Box::new(lower_expr(left)?),
            right: Box::new(lower_expr(right)?),
            ty: IrType::from_type(ty),
        }),
        TypedExpr::Unary { op, expr, ty } => Ok(IrValue::Unary {
            op: *op,
            expr: Box::new(lower_expr(expr)?),
            ty: IrType::from_type(ty),
        }),
        TypedExpr::Call { name, args, ty } => Ok(IrValue::Call {
            name: name.clone(),
            args: args.iter().map(lower_expr).collect::<VppResult<_>>()?,
            ty: IrType::from_type(ty),
        }),
        TypedExpr::Index { target, index, ty } => Ok(IrValue::Index {
            target: Box::new(lower_expr(target)?),
            index: Box::new(lower_expr(index)?),
            ty: IrType::from_type(ty),
        }),
        TypedExpr::Field { target, field, ty } => Ok(IrValue::Field {
            target: Box::new(lower_expr(target)?),
            field: field.clone(),
            ty: IrType::from_type(ty),
        }),
        TypedExpr::Array { elements, ty } => Ok(IrValue::Array {
            elements: elements.iter().map(lower_expr).collect::<VppResult<_>>()?,
            ty: IrType::from_type(ty),
        }),
        TypedExpr::StructLit { name, fields, ty } => Ok(IrValue::StructLit {
            name: name.clone(),
            fields: fields
                .iter()
                .map(|(f, e)| Ok((f.clone(), lower_expr(e)?)))
                .collect::<VppResult<_>>()?,
            ty: IrType::from_type(ty),
        }),
        TypedExpr::Variant {
            enum_name,
            variant,
            payload,
            ty,
        } => Ok(IrValue::Variant {
            enum_name: enum_name.clone(),
            variant: variant.clone(),
            payload: payload.iter().map(lower_expr).collect::<VppResult<_>>()?,
            ty: IrType::from_type(ty),
        }),
        TypedExpr::Assign { name, value, ty } => Ok(IrValue::Assign {
            name: name.clone(),
            value: Box::new(lower_expr(value)?),
            ty: IrType::from_type(ty),
        }),
        TypedExpr::Match { scrutinee, arms, ty } => Ok(IrValue::Match {
            scrutinee: Box::new(lower_expr(scrutinee)?),
            arms: arms.iter().map(lower_match_arm).collect::<VppResult<_>>()?,
            ty: IrType::from_type(ty),
        }),
    }
}

// Register synthetic enum layouts used by Option/Result from expression types.
impl IrModule {
    pub fn ensure_enum_from_type(&mut self, ty: &Type) {
        match ty {
            Type::Option(inner) => {
                let key = format!("Option<{}>", inner.name());
                self.enum_defs.entry(key).or_insert_with(|| {
                    vec![
                        ("None".to_string(), vec![]),
                        ("Some".to_string(), vec![IrType::from_type(inner)]),
                    ]
                });
            }
            Type::Result { ok, err } => {
                let key = format!("Result<{}, {}>", ok.name(), err.name());
                self.enum_defs.entry(key).or_insert_with(|| {
                    vec![
                        ("Ok".to_string(), vec![IrType::from_type(ok)]),
                        ("Err".to_string(), vec![IrType::from_type(err)]),
                    ]
                });
            }
            _ => {}
        }
    }
}

pub fn lower_program_with_enums(program: &TypedProgram) -> VppResult<IrModule> {
    let mut ir = lower_program(program)?;
    for (_, func) in &program.functions {
        ir.ensure_enum_from_type(&func.ret);
        for (_, t) in &func.params {
            ir.ensure_enum_from_type(t);
        }
    }
    for stmt in &program.top_level {
        collect_types_stmt(stmt, &mut ir);
    }
    Ok(ir)
}

fn collect_types_stmt(stmt: &TypedStmt, ir: &mut IrModule) {
    match stmt {
        TypedStmt::Let { ty, value, .. } => {
            ir.ensure_enum_from_type(ty);
            collect_types_value(value, ir);
        }
        TypedStmt::Expr(v) => collect_types_value(v, ir),
        TypedStmt::If { then_block, else_block, .. } => {
            for s in then_block {
                collect_types_stmt(s, ir);
            }
            if let Some(stmts) = else_block {
                for s in stmts {
                    collect_types_stmt(s, ir);
                }
            }
        }
        TypedStmt::While { body, .. } | TypedStmt::Block(body) => {
            for s in body {
                collect_types_stmt(s, ir);
            }
        }
        TypedStmt::ForInt { body, .. } | TypedStmt::ForArray { body, .. } => {
            for s in body {
                collect_types_stmt(s, ir);
            }
        }
        TypedStmt::Return { value: Some(v) } => collect_types_value(v, ir),
        TypedStmt::Match { scrutinee, arms, .. } => {
            collect_types_value(scrutinee, ir);
            for arm in arms {
                for s in &arm.body {
                    collect_types_stmt(s, ir);
                }
            }
        }
        _ => {}
    }
}

fn collect_types_value(expr: &TypedExpr, ir: &mut IrModule) {
    ir.ensure_enum_from_type(&expr.ty());
    match expr {
        TypedExpr::Binary { left, right, .. } => {
            collect_types_value(left, ir);
            collect_types_value(right, ir);
        }
        TypedExpr::Unary { expr, .. } => collect_types_value(expr, ir),
        TypedExpr::Call { args, .. } => {
            for a in args {
                collect_types_value(a, ir);
            }
        }
        TypedExpr::Index { target, index, .. } => {
            collect_types_value(target, ir);
            collect_types_value(index, ir);
        }
        TypedExpr::Field { target, .. } => collect_types_value(target, ir),
        TypedExpr::Array { elements, .. } => {
            for e in elements {
                collect_types_value(e, ir);
            }
        }
        TypedExpr::StructLit { fields, .. } => {
            for (_, e) in fields {
                collect_types_value(e, ir);
            }
        }
        TypedExpr::Variant { payload, .. } => {
            for e in payload {
                collect_types_value(e, ir);
            }
        }
        TypedExpr::Assign { value, .. } => collect_types_value(value, ir),
        TypedExpr::Match { scrutinee, arms, .. } => {
            collect_types_value(scrutinee, ir);
            for arm in arms {
                for s in &arm.body {
                    collect_types_stmt(s, ir);
                }
            }
        }
        _ => {}
    }
}
