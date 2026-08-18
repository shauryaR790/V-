use std::collections::{HashMap, HashSet};

use crate::ast::{
    BinOp, Block, EnumDecl, Expr, FnDecl, Item, MatchArm, Pattern, Program, Stmt, StructDecl,
    TestDecl, UnOp,
};
use crate::modules::ModuleGraph;
use crate::error::{span_to_source, type_mismatch, VppError, VppResult};
use crate::span::Span;
use crate::symbols::{SymbolDef, SymbolKind};
use crate::types::{
    EnumInfo, FunctionInfo, StructInfo, TestInfo, TypedExpr, TypedMatchArm, TypedPattern,
    TypedProgram, TypedStmt, Type,
};

pub struct TypeChecker<'source> {
    source: &'source str,
    source_file: std::path::PathBuf,
    functions: HashMap<String, FunctionInfo>,
    structs: HashMap<String, StructInfo>,
    enums: HashMap<String, EnumInfo>,
    scopes: Vec<HashMap<String, Type>>,
    symbols: crate::symbols::SymbolIndex,
    expected_type: Option<Type>,
    current_ret: Option<Type>,
    loop_depth: usize,
    modules: ModuleGraph,
    module_scoped: HashSet<String>,
}

impl<'source> TypeChecker<'source> {
    fn resolve_ann(&self, ann: &crate::ast::TypeAnn) -> Type {
        Type::from_ann(ann, &self.structs, &self.enums)
    }

    pub fn new(source: &'source str) -> Self {
        Self::with_file(source, std::path::PathBuf::from("<source>"))
    }

    pub fn with_file(source: &'source str, source_file: std::path::PathBuf) -> Self {
        Self {
            source,
            source_file,
            functions: HashMap::new(),
            structs: HashMap::new(),
            enums: HashMap::new(),
            scopes: vec![HashMap::new()],
            symbols: crate::symbols::SymbolIndex::new(),
            expected_type: None,
            current_ret: None,
            loop_depth: 0,
            modules: ModuleGraph::default(),
            module_scoped: HashSet::new(),
        }
    }

    pub fn with_modules(source: &'source str, source_file: std::path::PathBuf, modules: ModuleGraph) -> Self {
        let module_scoped = modules.scoped_functions.clone();
        Self {
            source,
            source_file,
            functions: HashMap::new(),
            structs: HashMap::new(),
            enums: HashMap::new(),
            scopes: vec![HashMap::new()],
            symbols: crate::symbols::SymbolIndex::new(),
            expected_type: None,
            current_ret: None,
            loop_depth: 0,
            modules,
            module_scoped,
        }
    }

    pub fn check(mut self, program: &Program) -> VppResult<TypedProgram> {
        self.register_types(program)?;
        for item in &program.items {
            if let Item::Function(f) = item {
                self.register_function(f)?;
            }
        }

        let mut top_level = Vec::new();
        let mut tests = Vec::new();
        for item in &program.items {
            match item {
                Item::Function(f) => {
                    let info = self.check_function(f)?;
                    self.functions.insert(f.name.clone(), info);
                }
                Item::Test(test) => {
                    tests.push(self.check_test(test)?);
                }
                Item::Statement(stmt) => {
                    top_level.push(self.check_stmt(stmt)?);
                }
                Item::Import(_) | Item::Struct(_) | Item::Enum(_) => {}
            }
        }

        Ok(TypedProgram {
            functions: self.functions,
            structs: self.structs,
            enums: self.enums,
            tests,
            top_level,
            symbols: self.symbols,
            source_file: self.source_file,
        })
    }

    fn check_test(&mut self, test: &TestDecl) -> VppResult<TestInfo> {
        let saved_ret = self.current_ret.take();
        self.current_ret = None;
        let body = self.check_block_stmts(&test.body)?;
        self.current_ret = saved_ret;
        Ok(TestInfo {
            name: test.name.clone(),
            body,
            span: test.span,
        })
    }

    fn register_types(&mut self, program: &Program) -> VppResult<()> {
        for item in &program.items {
            match item {
                Item::Struct(s) => self.register_struct(s)?,
                Item::Enum(e) => self.register_enum(e)?,
                _ => {}
            }
        }
        Ok(())
    }

    fn register_struct(&mut self, s: &StructDecl) -> VppResult<()> {
        if self.structs.contains_key(&s.name) {
            return Err(VppError::Other {
                message: format!("struct `{}` is already defined", s.name),
            });
        }
        let mut fields = HashMap::new();
        for field in &s.fields {
            fields.insert(
                field.name.clone(),
                self.resolve_ann(&field.ty),
            );
        }
        self.symbols.insert(
            s.name.clone(),
            SymbolDef {
                kind: SymbolKind::Struct,
                file: self.source_file.clone(),
                span: s.span,
            },
        );
        self.structs.insert(
            s.name.clone(),
            StructInfo {
                name: s.name.clone(),
                fields,
                span: s.span,
            },
        );
        Ok(())
    }

    fn register_enum(&mut self, e: &EnumDecl) -> VppResult<()> {
        if self.enums.contains_key(&e.name) {
            return Err(VppError::Other {
                message: format!("enum `{}` is already defined", e.name),
            });
        }
        let mut variants = HashMap::new();
        for variant in &e.variants {
            let payload: Vec<Type> = variant
                .payload
                .iter()
                .map(|t| self.resolve_ann(t))
                .collect();
            self.symbols.insert(
                format!("{}.{}", e.name, variant.name),
                SymbolDef {
                    kind: SymbolKind::Variant,
                    file: self.source_file.clone(),
                    span: variant.span,
                },
            );
            variants.insert(variant.name.clone(), payload);
        }
        self.symbols.insert(
            e.name.clone(),
            SymbolDef {
                kind: SymbolKind::Enum,
                file: self.source_file.clone(),
                span: e.span,
            },
        );
        self.enums.insert(
            e.name.clone(),
            EnumInfo {
                name: e.name.clone(),
                variants,
                span: e.span,
            },
        );
        Ok(())
    }

    fn register_function(&mut self, f: &FnDecl) -> VppResult<()> {
        if self.functions.contains_key(&f.name) {
            return Err(VppError::Other {
                message: format!("function `{}` is already defined", f.name),
            });
        }
        let params: Vec<(String, Type)> = f
            .params
            .iter()
            .map(|p| (p.name.clone(), self.resolve_ann(&p.ty)))
            .collect();
        self.functions.insert(
            f.name.clone(),
            FunctionInfo {
                name: f.name.clone(),
                params,
                ret: self.resolve_ann(&f.ret_type),
                body: Vec::new(),
                span: f.span,
            },
        );
        Ok(())
    }

    fn check_function(&mut self, f: &FnDecl) -> VppResult<FunctionInfo> {
        self.symbols.insert(
            f.name.clone(),
            SymbolDef {
                kind: SymbolKind::Function,
                file: self.source_file.clone(),
                span: f.span,
            },
        );
        self.push_scope();
        for param in &f.params {
            let ty = self.resolve_ann(&param.ty);
            self.define(&param.name, ty);
        }

        self.current_ret = Some(self.resolve_ann(&f.ret_type));
        let body = self.check_block_stmts(&f.body)?;
        self.current_ret = None;
        self.pop_scope();

        let ret = self.resolve_ann(&f.ret_type);
        if ret != Type::Void && !body.is_empty() && !self.body_satisfies_return(&body, &ret) {
            return Err(VppError::MissingReturn {
                expected: ret.name(),
                span: span_to_source(self.source, f.span),
            });
        }

        Ok(FunctionInfo {
            name: f.name.clone(),
            params: f
                .params
                .iter()
                .map(|p| (p.name.clone(), self.resolve_ann(&p.ty)))
                .collect(),
            ret,
            body,
            span: f.span,
        })
    }

    fn body_satisfies_return(&self, body: &[TypedStmt], expected: &Type) -> bool {
        if body
            .iter()
            .any(|s| matches!(s, TypedStmt::Return { .. }))
        {
            return true;
        }
        body.last()
            .map(|stmt| self.stmt_satisfies_return(stmt, expected))
            .unwrap_or(false)
    }

    fn stmt_satisfies_return(&self, stmt: &TypedStmt, expected: &Type) -> bool {
        match stmt {
            TypedStmt::Return { .. } => true,
            TypedStmt::Match { arms, ty, .. } => {
                ty == expected
                    && arms
                        .iter()
                        .all(|arm| self.arm_body_satisfies_return(&arm.body, expected))
            }
            TypedStmt::If {
                then_block,
                else_block,
                ..
            } => {
                self.block_satisfies_return(then_block, expected)
                    && else_block
                        .as_ref()
                        .map(|b| self.block_satisfies_return(b, expected))
                        .unwrap_or(false)
            }
            TypedStmt::Expr(expr) => expr.ty() == *expected,
            TypedStmt::Block(stmts) => self.body_satisfies_return(stmts, expected),
            _ => false,
        }
    }

    fn block_satisfies_return(&self, stmts: &[TypedStmt], expected: &Type) -> bool {
        self.body_satisfies_return(stmts, expected)
    }

    fn arm_body_satisfies_return(&self, body: &[TypedStmt], expected: &Type) -> bool {
        if body
            .iter()
            .any(|s| matches!(s, TypedStmt::Return { .. }))
        {
            return true;
        }
        body.last()
            .and_then(|s| match s {
                TypedStmt::Expr(expr) => Some(expr.ty() == *expected),
                _ => None,
            })
            .unwrap_or(false)
    }

    fn check_block_stmts(&mut self, block: &Block) -> VppResult<Vec<TypedStmt>> {
        self.push_scope();
        let mut stmts = Vec::new();
        for stmt in &block.stmts {
            stmts.push(self.check_stmt(stmt)?);
        }
        self.pop_scope();
        Ok(stmts)
    }

    fn check_stmt(&mut self, stmt: &Stmt) -> VppResult<TypedStmt> {
        match stmt {
            Stmt::Let { name, ty, value, span } => {
                let expected = ty
                    .as_ref()
                    .map(|ann| self.resolve_ann(ann));
                let typed_value = if let Some(expected) = expected.clone() {
                    self.with_expected(expected, |this| this.check_expr(value))?
                } else {
                    self.check_expr(value)?
                };
                let binding_ty = expected.unwrap_or_else(|| typed_value.ty());
                self.define(name, binding_ty.clone());
                self.symbols.insert(
                    name.clone(),
                    SymbolDef {
                        kind: SymbolKind::Variable,
                        file: self.source_file.clone(),
                        span: *span,
                    },
                );
                Ok(TypedStmt::Let {
                    name: name.clone(),
                    ty: binding_ty,
                    value: typed_value,
                    span: *span,
                })
            }
            Stmt::Expr(expr) => Ok(TypedStmt::Expr(self.check_expr(expr)?)),
            Stmt::If {
                condition,
                then_block,
                else_block,
                ..
            } => {
                let cond = self.check_expr(condition)?;
                self.expect_type(&cond, &Type::Bool, condition.span())?;
                let then_block = self.check_block_stmts(then_block)?;
                let else_block = if let Some(block) = else_block {
                    Some(self.check_block_stmts(block)?)
                } else {
                    None
                };
                Ok(TypedStmt::If {
                    condition: cond,
                    then_block,
                    else_block,
                })
            }
            Stmt::While { condition, body, .. } => {
                let cond = self.check_expr(condition)?;
                self.expect_type(&cond, &Type::Bool, condition.span())?;
                self.loop_depth += 1;
                let body = self.check_block_stmts(body)?;
                self.loop_depth -= 1;
                Ok(TypedStmt::While {
                    condition: cond,
                    body,
                })
            }
            Stmt::For { var, iter, body, span } => {
                if let Expr::Range { start, end, .. } = iter {
                    let start_t = self.check_expr(start)?;
                    let end_t = self.check_expr(end)?;
                    self.expect_type(&start_t, &Type::Int, start.span())?;
                    self.expect_type(&end_t, &Type::Int, end.span())?;
                    let (TypedExpr::Int(start_val), TypedExpr::Int(end_val)) = (start_t, end_t)
                    else {
                        return Err(type_mismatch(
                            self.source,
                            *span,
                            "integer range",
                            "non-integer range",
                            "range bounds must be integers",
                        ));
                    };
                    self.loop_depth += 1;
                    let loop_body = self.push_scope_for_loop(var, Type::Int, body)?;
                    self.loop_depth -= 1;
                    return Ok(TypedStmt::ForInt {
                        var: var.clone(),
                        start: start_val,
                        end: end_val,
                        body: loop_body,
                    });
                }

                let typed_iter = self.check_expr(iter)?;
                match typed_iter.ty() {
                    Type::Array(elem_ty) => {
                        self.loop_depth += 1;
                        let loop_body =
                            self.push_scope_for_loop(var, (*elem_ty).clone(), body)?;
                        self.loop_depth -= 1;
                        Ok(TypedStmt::ForArray {
                            var: var.clone(),
                            array: typed_iter,
                            elem_ty: *elem_ty,
                            body: loop_body,
                        })
                    }
                    found => Err(VppError::InvalidForIter {
                        found: found.name(),
                        span: span_to_source(self.source, *span),
                    }),
                }
            }
            Stmt::Return { value, .. } => {
                let typed = match value {
                    Some(v) => {
                        if let Some(ret) = self.current_ret.clone() {
                            Some(self.with_expected(ret, |this| this.check_expr(v))?)
                        } else {
                            Some(self.check_expr(v)?)
                        }
                    }
                    None => None,
                };
                Ok(TypedStmt::Return { value: typed })
            }
            Stmt::Match {
                scrutinee,
                arms,
                span,
            } => {
                let typed_scrutinee = self.check_expr(scrutinee)?;
                let (typed_arms, match_ty) =
                    self.check_match_arms(&typed_scrutinee, arms, *span)?;
                Ok(TypedStmt::Match {
                    scrutinee: typed_scrutinee,
                    arms: typed_arms,
                    ty: match_ty,
                })
            }
            Stmt::Block(block) => {
                let stmts = self.check_block_stmts(block)?;
                Ok(TypedStmt::Block(stmts))
            }
            Stmt::Break { span: _ } => {
                if self.loop_depth == 0 {
                    return Err(VppError::Other {
                        message: "`break` can only be used inside a loop".to_string(),
                    });
                }
                Ok(TypedStmt::Break)
            }
            Stmt::Continue { span: _ } => {
                if self.loop_depth == 0 {
                    return Err(VppError::Other {
                        message: "`continue` can only be used inside a loop".to_string(),
                    });
                }
                Ok(TypedStmt::Continue)
            }
        }
    }

    fn check_match_arms(
        &mut self,
        scrutinee: &TypedExpr,
        arms: &[MatchArm],
        span: Span,
    ) -> VppResult<(Vec<TypedMatchArm>, Type)> {
        let scrutinee_ty = scrutinee.ty();
        let mut typed_arms = Vec::new();
        let mut arm_types = Vec::new();

        for arm in arms {
            self.push_scope();
            let pattern = self.check_pattern(&arm.pattern, &scrutinee_ty)?;
            let body = self.check_block_stmts(&arm.body)?;
            self.pop_scope();

            let arm_ty = body
                .last()
                .and_then(|s| match s {
                    TypedStmt::Expr(e) => Some(e.ty()),
                    TypedStmt::Return { value: Some(v) } => Some(v.ty()),
                    _ => None,
                })
                .unwrap_or(Type::Void);
            arm_types.push(arm_ty);
            typed_arms.push(TypedMatchArm { pattern, body });
        }

        if typed_arms.is_empty() {
            return Err(VppError::Other {
                message: "match must have at least one arm".to_string(),
            });
        }

        let match_ty = arm_types[0].clone();
        for (i, ty) in arm_types.iter().enumerate().skip(1) {
            if ty != &match_ty && *ty != Type::Void && match_ty != Type::Void {
                return Err(type_mismatch(
                    self.source,
                    span,
                    &match_ty.name(),
                    &ty.name(),
                    format!("match arm {i} returns a different type than the first arm"),
                ));
            }
        }

        Ok((typed_arms, match_ty))
    }

    fn check_pattern(&mut self, pattern: &Pattern, scrutinee_ty: &Type) -> VppResult<TypedPattern> {
        match pattern {
            Pattern::Wildcard { .. } => Ok(TypedPattern::Wildcard),
            Pattern::Literal(expr) => {
                let typed = self.check_expr(expr)?;
                if typed.ty() != *scrutinee_ty {
                    return Err(type_mismatch(
                        self.source,
                        expr.span(),
                        &scrutinee_ty.name(),
                        &typed.ty().name(),
                        "pattern literal must match scrutinee type",
                    ));
                }
                Ok(TypedPattern::Literal(typed))
            }
            Pattern::Variant {
                enum_name,
                variant,
                bindings,
                span,
            } => {
                let (resolved_enum, payload_types) =
                    self.resolve_variant(enum_name.as_deref(), variant, scrutinee_ty, *span)?;

                if bindings.len() != payload_types.len() {
                    return Err(VppError::WrongArgCount {
                        name: variant.clone(),
                        expected: payload_types.len(),
                        found: bindings.len(),
                        span: span_to_source(self.source, *span),
                    });
                }

                for (binding, ty) in bindings.iter().zip(payload_types.iter()) {
                    self.define(binding, ty.clone());
                }

                Ok(TypedPattern::Variant {
                    enum_name: resolved_enum,
                    variant: variant.clone(),
                    payload_types,
                    bindings: bindings.clone(),
                })
            }
            Pattern::Struct {
                struct_name,
                fields,
                span,
            } => {
                let struct_ty = self.resolve_struct_pattern(struct_name.as_deref(), scrutinee_ty, *span)?;
                let Type::Struct { name, fields: struct_fields } = struct_ty else {
                    unreachable!();
                };

                let mut typed_fields = Vec::new();
                for (field, binding) in fields {
                    let field_ty = struct_fields.get(field).ok_or_else(|| VppError::Other {
                        message: format!("struct `{name}` has no field `{field}`"),
                    })?;
                    self.define(binding, field_ty.clone());
                    typed_fields.push((field.clone(), binding.clone(), field_ty.clone()));
                }

                Ok(TypedPattern::Struct {
                    struct_name: name,
                    fields: typed_fields,
                })
            }
        }
    }

    fn resolve_variant(
        &self,
        enum_name: Option<&str>,
        variant: &str,
        scrutinee_ty: &Type,
        span: Span,
    ) -> VppResult<(String, Vec<Type>)> {
        if let Some(inner) = scrutinee_ty.option_inner() {
            if variant == "None" {
                return Ok(("Option".to_string(), Vec::new()));
            }
            if variant == "Some" {
                return Ok(("Option".to_string(), vec![(*inner).clone()]));
            }
        }
        if let Some((ok, err)) = scrutinee_ty.result_inner() {
            if variant == "Ok" {
                return Ok(("Result".to_string(), vec![(*ok).clone()]));
            }
            if variant == "Err" {
                return Ok(("Result".to_string(), vec![(*err).clone()]));
            }
        }

        let enum_name = enum_name
            .map(String::from)
            .or_else(|| match scrutinee_ty {
                Type::Enum { name, .. } => Some(name.clone()),
                _ => None,
            })
            .ok_or_else(|| VppError::Other {
                message: format!("unknown variant `{variant}` for type {}", scrutinee_ty.name()),
            })?;

        let enum_info = self.enums.get(&enum_name).ok_or_else(|| VppError::Other {
            message: format!("unknown enum `{enum_name}`"),
        })?;

        let payload = enum_info
            .variants
            .get(variant)
            .ok_or_else(|| VppError::Other {
                message: format!("enum `{enum_name}` has no variant `{variant}`"),
            })?
            .clone();

        if !matches!(scrutinee_ty, Type::Enum { name: scr, .. } if scr == &enum_name) {
            if scrutinee_ty.name() != enum_name {
                return Err(type_mismatch(
                    self.source,
                    span,
                    &enum_name,
                    &scrutinee_ty.name(),
                    "variant pattern does not match scrutinee",
                ));
            }
        }

        Ok((enum_name, payload))
    }

    fn resolve_struct_pattern(
        &self,
        struct_name: Option<&str>,
        scrutinee_ty: &Type,
        span: Span,
    ) -> VppResult<Type> {
        if let Type::Struct { name, fields } = scrutinee_ty {
            if let Some(expected) = struct_name {
                if expected != name {
                    return Err(type_mismatch(
                        self.source,
                        span,
                        name,
                        expected,
                        "struct pattern name does not match scrutinee",
                    ));
                }
            }
            return Ok(Type::Struct {
                name: name.clone(),
                fields: fields.clone(),
            });
        }
        Err(type_mismatch(
            self.source,
            span,
            "struct",
            &scrutinee_ty.name(),
            "expected a struct scrutinee",
        ))
    }

    fn push_scope_for_loop(
        &mut self,
        var: &str,
        ty: Type,
        body: &Block,
    ) -> VppResult<Vec<TypedStmt>> {
        self.push_scope();
        self.define(var, ty);
        let stmts = self.check_block_stmts(body)?;
        self.pop_scope();
        Ok(stmts)
    }

    fn check_expr(&mut self, expr: &Expr) -> VppResult<TypedExpr> {
        match expr {
            Expr::Int { value, .. } => Ok(TypedExpr::Int(*value)),
            Expr::Float { value, .. } => Ok(TypedExpr::Float(*value)),
            Expr::Bool { value, .. } => Ok(TypedExpr::Bool(*value)),
            Expr::String { value, .. } => Ok(TypedExpr::String(value.clone())),
            Expr::Ident { name, span } => {
                if name == "None" {
                    let ty = self
                        .expected_type
                        .clone()
                        .or_else(|| self.current_ret.clone())
                        .ok_or_else(|| VppError::Other {
                            message: "cannot infer type for `None`; add a type annotation".to_string(),
                        })?;
                    if ty.option_inner().is_none() {
                        return Err(type_mismatch(
                            self.source,
                            *span,
                            "Option<T>",
                            &ty.name(),
                            "`None` requires an Option type",
                        ));
                    }
                    return Ok(TypedExpr::Variant {
                        enum_name: "Option".to_string(),
                        variant: "None".to_string(),
                        payload: Vec::new(),
                        ty,
                    });
                }
                if let Some(Type::Enum {
                    name: enum_name,
                    variants,
                }) = self.expected_type.clone()
                {
                    if variants.contains_key(name) {
                        return Ok(TypedExpr::Variant {
                            enum_name: enum_name.clone(),
                            variant: name.clone(),
                            payload: Vec::new(),
                            ty: Type::Enum {
                                name: enum_name,
                                variants,
                            },
                        });
                    }
                }
                let ty = self
                    .lookup(name)
                    .ok_or_else(|| VppError::UndefinedVariable {
                        name: name.clone(),
                        span: span_to_source(self.source, *span),
                        help: format!("did you mean to declare it with `let {name} = ...`?"),
                    })?;
                Ok(TypedExpr::Ident {
                    name: name.clone(),
                    ty,
                    span: *span,
                })
            }
            Expr::Binary { op, left, right, span } => {
                let left_t = self.check_expr(left)?;
                let right_t = self.check_expr(right)?;
                self.check_binary(*op, left_t, right_t, *span)
            }
            Expr::Unary { op, expr, span } => {
                let inner = self.check_expr(expr)?;
                self.check_unary(*op, inner, *span)
            }
            Expr::Call { name, args, span } => self.check_call(name, args, *span),
            Expr::Index { target, index, span } => {
                let target_t = self.check_expr(target)?;
                let index_t = self.check_expr(index)?;
                self.expect_type(&index_t, &Type::Int, index.span())?;
                match target_t.ty() {
                    Type::Array(elem) => Ok(TypedExpr::Index {
                        target: Box::new(target_t),
                        index: Box::new(index_t),
                        ty: *elem,
                    }),
                    found => Err(type_mismatch(
                        self.source,
                        *span,
                        "array",
                        &found.name(),
                        "only arrays can be indexed with `[`",
                    )),
                }
            }
            Expr::Field { target, field, span } => {
                let target_t = self.check_expr(target)?;
                self.check_field(target_t, field, *span)
            }
            Expr::Array { elements, span } => {
                if elements.is_empty() {
                    return Err(VppError::EmptyArrayNoType {
                        span: span_to_source(self.source, *span),
                    });
                }
                let first = self.check_expr(&elements[0])?;
                let elem_ty = first.ty();
                let mut typed_elems = vec![first];
                for elem in &elements[1..] {
                    let typed = self.check_expr(elem)?;
                    if typed.ty() != elem_ty {
                        return Err(VppError::ArrayElementMismatch {
                            expected: elem_ty.name(),
                            found: typed.ty().name(),
                            span: span_to_source(self.source, elem.span()),
                        });
                    }
                    typed_elems.push(typed);
                }
                Ok(TypedExpr::Array {
                    elements: typed_elems,
                    ty: Type::Array(Box::new(elem_ty)),
                })
            }
            Expr::StructLit { name, fields, span } => self.check_struct_lit(name.as_deref(), fields, *span),
            Expr::Range { span, .. } => Err(type_mismatch(
                self.source,
                *span,
                "for-in loop",
                "range expression",
                "ranges like `0..10` can only be used in `for` loops",
            )),
            Expr::Assign { name, value, span } => {
                let typed_value = self.check_expr(value)?;
                let existing = self.lookup(name).ok_or_else(|| VppError::ImmutableAssign {
                    name: name.clone(),
                    span: span_to_source(self.source, *span),
                })?;
                if typed_value.ty() != existing {
                    return Err(type_mismatch(
                        self.source,
                        *span,
                        &existing.name(),
                        &typed_value.ty().name(),
                        format!("`{name}` was declared as {}", existing.name()),
                    ));
                }
                Ok(TypedExpr::Assign {
                    name: name.clone(),
                    value: Box::new(typed_value.clone()),
                    ty: existing,
                })
            }
            Expr::Match {
                scrutinee,
                arms,
                span,
            } => {
                let typed_scrutinee = self.check_expr(scrutinee)?;
                let (typed_arms, match_ty) =
                    self.check_match_arms(&typed_scrutinee, arms, *span)?;
                Ok(TypedExpr::Match {
                    scrutinee: Box::new(typed_scrutinee),
                    arms: typed_arms,
                    ty: match_ty,
                })
            }
        }
    }

    fn check_struct_lit(
        &mut self,
        name: Option<&str>,
        fields: &[(String, Expr)],
        _span: Span,
    ) -> VppResult<TypedExpr> {
        let struct_name = name
            .map(String::from)
            .or_else(|| {
                self.expected_type.as_ref().and_then(|ty| match ty {
                    Type::Struct { name, .. } => Some(name.clone()),
                    _ => None,
                })
            })
            .ok_or_else(|| VppError::Other {
                message: "struct literal needs a type name or context type".to_string(),
            })?;

        let struct_info = self
            .structs
            .get(&struct_name)
            .cloned()
            .ok_or_else(|| VppError::Other {
                message: format!("unknown struct `{struct_name}`"),
            })?;

        let mut typed_fields = Vec::new();
        for (field_name, expr) in fields {
            let expected = struct_info.fields.get(field_name).ok_or_else(|| VppError::Other {
                message: format!("struct `{struct_name}` has no field `{field_name}`"),
            })?;
            let typed = self.with_expected(expected.clone(), |this| this.check_expr(expr))?;
            typed_fields.push((field_name.clone(), typed));
        }

        for field_name in struct_info.fields.keys() {
            if !fields.iter().any(|(n, _)| n == field_name) {
                return Err(VppError::Other {
                    message: format!("missing field `{field_name}` in `{struct_name}` literal"),
                });
            }
        }

        Ok(TypedExpr::StructLit {
            name: struct_name.clone(),
            fields: typed_fields,
            ty: Type::Struct {
                name: struct_name,
                fields: struct_info.fields,
            },
        })
    }

    fn check_field(&self, target: TypedExpr, field: &str, span: Span) -> VppResult<TypedExpr> {
        match target.ty() {
            Type::Struct { name, fields } => {
                let field_ty = fields.get(field).ok_or_else(|| VppError::Other {
                    message: format!("struct `{name}` has no field `{field}`"),
                })?;
                Ok(TypedExpr::Field {
                    target: Box::new(target),
                    field: field.to_string(),
                    ty: field_ty.clone(),
                })
            }
            Type::Enum { name, variants } => {
                if variants.contains_key(field) {
                    Ok(TypedExpr::Variant {
                        enum_name: name.clone(),
                        variant: field.to_string(),
                        payload: Vec::new(),
                        ty: Type::Enum {
                            name: name.clone(),
                            variants: variants.clone(),
                        },
                    })
                } else {
                    Err(VppError::Other {
                        message: format!("enum `{name}` has no variant `{field}`"),
                    })
                }
            }
            other => Err(type_mismatch(
                self.source,
                span,
                "struct or enum",
                &other.name(),
                "field access requires a struct or enum type",
            )),
        }
    }

    fn check_binary(
        &self,
        op: BinOp,
        left: TypedExpr,
        right: TypedExpr,
        span: Span,
    ) -> VppResult<TypedExpr> {
        let result_ty = match op {
            BinOp::Add => {
                if left.ty() == Type::String && right.ty() == Type::String {
                    Type::String
                } else if left.ty() == Type::Int && right.ty() == Type::Int {
                    Type::Int
                } else if left.ty() == Type::Float && right.ty() == Type::Float {
                    Type::Float
                } else {
                    return Err(type_mismatch(
                        self.source,
                        span,
                        "matching numeric or string types",
                        &format!("{} and {}", left.ty(), right.ty()),
                        "use matching types on both sides of the operator",
                    ));
                }
            }
            BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Mod => {
                if left.ty() == Type::Int && right.ty() == Type::Int {
                    Type::Int
                } else if left.ty() == Type::Float && right.ty() == Type::Float {
                    Type::Float
                } else {
                    return Err(type_mismatch(
                        self.source,
                        span,
                        "matching numeric types",
                        &format!("{} and {}", left.ty(), right.ty()),
                        "arithmetic operators require matching int or float operands",
                    ));
                }
            }
            BinOp::Eq | BinOp::NotEq => {
                if left.ty() != right.ty() {
                    return Err(type_mismatch(
                        self.source,
                        span,
                        &left.ty().name(),
                        &right.ty().name(),
                        "both sides of a comparison must have the same type",
                    ));
                }
                Type::Bool
            }
            BinOp::Lt | BinOp::LtEq | BinOp::Gt | BinOp::GtEq => {
                if !left.ty().is_numeric() || left.ty() != right.ty() {
                    return Err(type_mismatch(
                        self.source,
                        span,
                        "matching numeric types",
                        &format!("{} and {}", left.ty(), right.ty()),
                        "ordering comparisons require matching int or float operands",
                    ));
                }
                Type::Bool
            }
            BinOp::And | BinOp::Or => {
                self.expect_type(&left, &Type::Bool, span)?;
                self.expect_type(&right, &Type::Bool, span)?;
                Type::Bool
            }
        };

        Ok(TypedExpr::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
            ty: result_ty,
        })
    }

    fn check_unary(&self, op: UnOp, expr: TypedExpr, span: Span) -> VppResult<TypedExpr> {
        let ty = match op {
            UnOp::Not => {
                self.expect_type(&expr, &Type::Bool, span)?;
                Type::Bool
            }
            UnOp::Neg => {
                if expr.ty() == Type::Int {
                    Type::Int
                } else if expr.ty() == Type::Float {
                    Type::Float
                } else {
                    return Err(type_mismatch(
                        self.source,
                        span,
                        "int or float",
                        &expr.ty().name(),
                        "unary `-` requires a numeric operand",
                    ));
                }
            }
        };
        Ok(TypedExpr::Unary {
            op,
            expr: Box::new(expr),
            ty,
        })
    }

    fn check_call(&mut self, name: &str, args: &[Expr], span: Span) -> VppResult<TypedExpr> {
        if name == "Some" {
            if args.len() != 1 {
                return Err(VppError::WrongArgCount {
                    name: name.to_string(),
                    expected: 1,
                    found: args.len(),
                    span: span_to_source(self.source, span),
                });
            }
            let arg = self.check_expr(&args[0])?;
            let ty = self
                .expected_type
                .clone()
                .or_else(|| self.current_ret.clone())
                .unwrap_or(Type::Option(Box::new(arg.ty())));
            let inner = ty.option_inner().ok_or_else(|| VppError::Other {
                message: "`Some` requires Option context or annotation".to_string(),
            })?;
            if arg.ty() != *inner {
                return Err(type_mismatch(
                    self.source,
                    args[0].span(),
                    &inner.name(),
                    &arg.ty().name(),
                    "`Some` payload must match Option inner type",
                ));
            }
            return Ok(TypedExpr::Variant {
                enum_name: "Option".to_string(),
                variant: "Some".to_string(),
                payload: vec![arg],
                ty,
            });
        }

        if name == "Ok" || name == "Err" {
            if args.len() != 1 {
                return Err(VppError::WrongArgCount {
                    name: name.to_string(),
                    expected: 1,
                    found: args.len(),
                    span: span_to_source(self.source, span),
                });
            }
            let arg = self.check_expr(&args[0])?;
            let ty = self
                .expected_type
                .clone()
                .or_else(|| self.current_ret.clone())
                .unwrap_or(Type::Result {
                    ok: Box::new(arg.ty()),
                    err: Box::new(Type::String),
                });
            let (ok_ty, err_ty) = ty.result_inner().ok_or_else(|| VppError::Other {
                message: format!("`{name}` requires Result context or annotation"),
            })?;
            if name == "Ok" && arg.ty() != *ok_ty {
                return Err(type_mismatch(
                    self.source,
                    args[0].span(),
                    &ok_ty.name(),
                    &arg.ty().name(),
                    "`Ok` payload must match Result ok type",
                ));
            }
            if name == "Err" && arg.ty() != *err_ty {
                return Err(type_mismatch(
                    self.source,
                    args[0].span(),
                    &err_ty.name(),
                    &arg.ty().name(),
                    "`Err` payload must match Result err type",
                ));
            }
            return Ok(TypedExpr::Variant {
                enum_name: "Result".to_string(),
                variant: name.to_string(),
                payload: vec![arg],
                ty,
            });
        }

        if name.contains('.') {
            let parts: Vec<&str> = name.split('.').collect();
            if parts.len() == 2 {
                let (module_alias, member) = (parts[0], parts[1]);
                if let Some(exports) = self.modules.namespaces.get(module_alias) {
                    if exports.functions.contains(member) {
                        if let Some(func) = self.functions.get(member).cloned() {
                            if args.len() != func.params.len() {
                                return Err(VppError::WrongArgCount {
                                    name: name.to_string(),
                                    expected: func.params.len(),
                                    found: args.len(),
                                    span: span_to_source(self.source, span),
                                });
                            }
                            let mut typed_args = Vec::new();
                            for (arg, (_, expected)) in args.iter().zip(func.params.iter()) {
                                let typed = self.check_expr(arg)?;
                                if typed.ty() != *expected {
                                    return Err(type_mismatch(
                                        self.source,
                                        arg.span(),
                                        &expected.name(),
                                        &typed.ty().name(),
                                        format!("argument to `{name}` must be {}", expected.name()),
                                    ));
                                }
                                typed_args.push(typed);
                            }
                            return Ok(TypedExpr::Call {
                                name: member.to_string(),
                                args: typed_args,
                                ty: func.ret,
                            });
                        }
                        return Err(VppError::UnknownModuleMember {
                            module: module_alias.to_string(),
                            name: member.to_string(),
                            span: span_to_source(self.source, span),
                            help: format!("import `{module_alias}` and call `{module_alias}.{member}(…)`"),
                        });
                    }
                    if exports.structs.contains(member) || exports.enums.contains(member) {
                        return Err(VppError::Other {
                            message: format!("`{module_alias}.{member}` is a type, not a function"),
                        });
                    }
                    let suggestions: Vec<_> = exports
                        .functions
                        .iter()
                        .filter(|f| f.starts_with(member) || member.starts_with(f.as_str()))
                        .take(3)
                        .cloned()
                        .collect();
                    let help = if suggestions.is_empty() {
                        format!("available: {}", exports.functions.iter().cloned().collect::<Vec<_>>().join(", "))
                    } else {
                        format!("did you mean: {}", suggestions.join(", "))
                    };
                    return Err(VppError::UnknownModuleMember {
                        module: module_alias.to_string(),
                        name: member.to_string(),
                        span: span_to_source(self.source, span),
                        help,
                    });
                }

                let (enum_name, variant) = (parts[0], parts[1]);
                if let Some(enum_info) = self.enums.get(enum_name).cloned() {
                    let payload_types = enum_info.variants.get(variant).ok_or_else(|| {
                        VppError::Other {
                            message: format!("enum `{enum_name}` has no variant `{variant}`"),
                        }
                    })?;
                    if args.len() != payload_types.len() {
                        return Err(VppError::WrongArgCount {
                            name: name.to_string(),
                            expected: payload_types.len(),
                            found: args.len(),
                            span: span_to_source(self.source, span),
                        });
                    }
                    let mut typed_args = Vec::new();
                    for (arg, expected) in args.iter().zip(payload_types.iter()) {
                        let typed = self.check_expr(arg)?;
                        if typed.ty() != *expected {
                            return Err(type_mismatch(
                                self.source,
                                arg.span(),
                                &expected.name(),
                                &typed.ty().name(),
                                format!("argument to `{name}`"),
                            ));
                        }
                        typed_args.push(typed);
                    }
                    return Ok(TypedExpr::Variant {
                        enum_name: enum_name.to_string(),
                        variant: variant.to_string(),
                        payload: typed_args,
                        ty: Type::Enum {
                            name: enum_name.to_string(),
                            variants: enum_info.variants,
                        },
                    });
                }
            }
        }

        if name == "print" {
            let mut typed_args = Vec::new();
            for arg in args {
                let typed = self.check_expr(arg)?;
                if !Self::is_printable(&typed.ty()) {
                    return Err(type_mismatch(
                        self.source,
                        arg.span(),
                        "printable value",
                        &typed.ty().name(),
                        "`print` accepts int, float, bool, string, or struct values",
                    ));
                }
                typed_args.push(typed);
            }
            return Ok(TypedExpr::Call {
                name: name.to_string(),
                args: typed_args,
                ty: Type::Void,
            });
        }

        if name == "len" {
            if args.len() != 1 {
                return Err(VppError::WrongArgCount {
                    name: name.to_string(),
                    expected: 1,
                    found: args.len(),
                    span: span_to_source(self.source, span),
                });
            }
            let arg = self.check_expr(&args[0])?;
            match arg.ty() {
                Type::Array(_) | Type::String => Ok(TypedExpr::Call {
                    name: name.to_string(),
                    args: vec![arg],
                    ty: Type::Int,
                }),
                other => Err(type_mismatch(
                    self.source,
                    args[0].span(),
                    "array or string",
                    &other.name(),
                    "`len` accepts arrays and strings",
                )),
            }
        } else if name == "assert" {
            if args.len() != 1 {
                return Err(VppError::WrongArgCount {
                    name: name.to_string(),
                    expected: 1,
                    found: args.len(),
                    span: span_to_source(self.source, span),
                });
            }
            let arg = self.check_expr(&args[0])?;
            self.expect_type(&arg, &Type::Bool, args[0].span())?;
            Ok(TypedExpr::Call {
                name: name.to_string(),
                args: vec![arg],
                ty: Type::Void,
            })
        } else if name == "assert_eq" {
            if args.len() != 2 {
                return Err(VppError::WrongArgCount {
                    name: name.to_string(),
                    expected: 2,
                    found: args.len(),
                    span: span_to_source(self.source, span),
                });
            }
            let left = self.check_expr(&args[0])?;
            let right = self.check_expr(&args[1])?;
            if left.ty() != right.ty() {
                return Err(type_mismatch(
                    self.source,
                    args[1].span(),
                    &left.ty().name(),
                    &right.ty().name(),
                    "`assert_eq` requires both operands to have the same type",
                ));
            }
            Ok(TypedExpr::Call {
                name: name.to_string(),
                args: vec![left, right],
                ty: Type::Void,
            })
        } else if name == "read_file" {
            if args.len() != 1 {
                return Err(VppError::WrongArgCount {
                    name: name.to_string(),
                    expected: 1,
                    found: args.len(),
                    span: span_to_source(self.source, span),
                });
            }
            let arg = self.check_expr(&args[0])?;
            self.expect_type(&arg, &Type::String, args[0].span())?;
            Ok(TypedExpr::Call {
                name: name.to_string(),
                args: vec![arg],
                ty: Type::String,
            })
        } else if name == "write_file" {
            if args.len() != 2 {
                return Err(VppError::WrongArgCount {
                    name: name.to_string(),
                    expected: 2,
                    found: args.len(),
                    span: span_to_source(self.source, span),
                });
            }
            let path = self.check_expr(&args[0])?;
            let contents = self.check_expr(&args[1])?;
            self.expect_type(&path, &Type::String, args[0].span())?;
            self.expect_type(&contents, &Type::String, args[1].span())?;
            Ok(TypedExpr::Call {
                name: name.to_string(),
                args: vec![path, contents],
                ty: Type::Void,
            })
        } else if name == "file_exists" {
            if args.len() != 1 {
                return Err(VppError::WrongArgCount {
                    name: name.to_string(),
                    expected: 1,
                    found: args.len(),
                    span: span_to_source(self.source, span),
                });
            }
            let arg = self.check_expr(&args[0])?;
            self.expect_type(&arg, &Type::String, args[0].span())?;
            Ok(TypedExpr::Call {
                name: name.to_string(),
                args: vec![arg],
                ty: Type::Bool,
            })
        } else if name == "json_parse" || name == "json_stringify" {
            if args.len() != 1 {
                return Err(VppError::WrongArgCount {
                    name: name.to_string(),
                    expected: 1,
                    found: args.len(),
                    span: span_to_source(self.source, span),
                });
            }
            let arg = self.check_expr(&args[0])?;
            self.expect_type(&arg, &Type::String, args[0].span())?;
            Ok(TypedExpr::Call {
                name: name.to_string(),
                args: vec![arg],
                ty: Type::String,
            })
        } else if name == "process_run" {
            if args.len() != 1 {
                return Err(VppError::WrongArgCount {
                    name: name.to_string(),
                    expected: 1,
                    found: args.len(),
                    span: span_to_source(self.source, span),
                });
            }
            let arg = self.check_expr(&args[0])?;
            self.expect_type(&arg, &Type::String, args[0].span())?;
            Ok(TypedExpr::Call {
                name: name.to_string(),
                args: vec![arg],
                ty: Type::Int,
            })
        } else {
            if self.module_scoped.contains(name) {
                let hint = self
                    .modules
                    .namespaces
                    .iter()
                    .find(|(_, exports)| exports.functions.contains(name))
                    .map(|(alias, _)| format!("use `{alias}.{name}(…)`"))
                    .unwrap_or_else(|| format!("`{name}` is exported from an imported module"));
                return Err(VppError::Other {
                    message: format!("function `{name}` is not in scope; {hint}"),
                });
            }
            let func = self
                .functions
                .get(name)
                .cloned()
                .ok_or_else(|| VppError::UndefinedFunction {
                    name: name.to_string(),
                    span: span_to_source(self.source, span),
                })?;

            if args.len() != func.params.len() {
                return Err(VppError::WrongArgCount {
                    name: name.to_string(),
                    expected: func.params.len(),
                    found: args.len(),
                    span: span_to_source(self.source, span),
                });
            }

            let mut typed_args = Vec::new();
            for (arg, (_, expected)) in args.iter().zip(func.params.iter()) {
                let typed = self.check_expr(arg)?;
                if typed.ty() != *expected {
                    return Err(type_mismatch(
                        self.source,
                        arg.span(),
                        &expected.name(),
                        &typed.ty().name(),
                        format!("argument to `{name}` must be {}", expected.name()),
                    ));
                }
                typed_args.push(typed);
            }

            Ok(TypedExpr::Call {
                name: name.to_string(),
                args: typed_args,
                ty: func.ret,
            })
        }
    }

    fn is_printable(ty: &Type) -> bool {
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

    fn with_expected<F>(&mut self, expected: Type, f: F) -> VppResult<TypedExpr>
    where
        F: FnOnce(&mut Self) -> VppResult<TypedExpr>,
    {
        let prev = self.expected_type.replace(expected);
        let result = f(self);
        self.expected_type = prev;
        result
    }

    fn expect_type(&self, expr: &TypedExpr, expected: &Type, span: Span) -> VppResult<()> {
        if expr.ty() != *expected {
            return Err(type_mismatch(
                self.source,
                span,
                &expected.name(),
                &expr.ty().name(),
                format!("expected `{}` here", expected.name()),
            ));
        }
        Ok(())
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn define(&mut self, name: &str, ty: Type) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), ty);
        }
    }

    fn lookup(&self, name: &str) -> Option<Type> {
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(ty.clone());
            }
        }
        None
    }
}
