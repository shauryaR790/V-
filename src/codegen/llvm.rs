use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::{Linkage, Module};
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, FunctionType};
use inkwell::values::{BasicMetadataValueEnum, BasicValueEnum, CallSiteValue, FunctionValue, PointerValue};
use inkwell::AddressSpace;
use inkwell::IntPredicate;

use crate::ast::BinOp;
use crate::codegen::runtime;
use crate::error::{VppError, VppResult};
use crate::types::{
    FunctionInfo, TypedExpr, TypedProgram, TypedStmt, Type,
};

struct Codegen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    functions: HashMap<String, FunctionValue<'ctx>>,
    locals: HashMap<String, PointerValue<'ctx>>,
    local_types: HashMap<String, Type>,
    i64_type: inkwell::types::IntType<'ctx>,
    f64_type: inkwell::types::FloatType<'ctx>,
    i1_type: inkwell::types::IntType<'ctx>,
    i8_ptr_type: inkwell::types::PointerType<'ctx>,
    void_type: inkwell::types::VoidType<'ctx>,
}

pub fn compile(
    program: &TypedProgram,
    source_path: &Path,
    output: Option<&Path>,
    emit_ir: Option<&Path>,
) -> VppResult<()> {
    crate::ensure_llvm_stubs_linked();
    let context = Context::create();
    let module = context.create_module("vpp_module");
    let mut codegen = Codegen::new(&context, module);

    codegen.declare_runtime()?;
    codegen.compile_functions(program)?;
    codegen.compile_main(program)?;

    if let Some(ir_path) = emit_ir {
        codegen
            .module
            .print_to_file(ir_path)
            .map_err(|e| VppError::Other {
                message: format!("failed to write IR: {e}"),
            })?;
    }

    let Some(output) = output else {
        let _ = source_path;
        return Ok(());
    };

    let temp_dir = std::env::temp_dir().join(format!("vpp-{}", std::process::id()));
    std::fs::create_dir_all(&temp_dir).map_err(|source| VppError::Io { source })?;

    let ll_path = temp_dir.join("out.ll");
    let obj_path = temp_dir.join("out.o");
    let runtime_c = temp_dir.join("vpp_runtime.c");
    let runtime_o = temp_dir.join("vpp_runtime.o");

    runtime::emit_runtime_c(&runtime_c)?;

    codegen
        .module
        .print_to_file(&ll_path)
        .map_err(|e| VppError::Other {
            message: format!("failed to write IR: {e}"),
        })?;

    let ll_path_str = ll_path.to_string_lossy();
    let obj_path_str = obj_path.to_string_lossy();
    let runtime_c_str = runtime_c.to_string_lossy();
    let runtime_o_str = runtime_o.to_string_lossy();
    let output_str = output.to_string_lossy();

    run_command(
        "clang",
        &["-c", &ll_path_str, "-o", &obj_path_str, "-O1"],
    )?;

    run_command("clang", &["-c", &runtime_c_str, "-o", &runtime_o_str])?;

    run_command(
        "clang",
        &[&obj_path_str, &runtime_o_str, "-o", &output_str],
    )?;

    let _ = source_path;
    Ok(())
}

fn run_command(program: &str, args: &[&str]) -> VppResult<()> {
    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(|e| VppError::Other {
            message: format!(
                "failed to run `{program}`: {e}. Ensure LLVM/clang is installed and on PATH."
            ),
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        return Err(VppError::Other {
            message: format!("`{program}` failed:\n{stdout}\n{stderr}"),
        });
    }
    Ok(())
}

impl<'ctx> Codegen<'ctx> {
    fn new(context: &'ctx Context, module: Module<'ctx>) -> Self {
        Self {
            context,
            module,
            builder: context.create_builder(),
            functions: HashMap::new(),
            locals: HashMap::new(),
            local_types: HashMap::new(),
            i64_type: context.i64_type(),
            f64_type: context.f64_type(),
            i1_type: context.bool_type(),
            i8_ptr_type: context.ptr_type(AddressSpace::default()),
            void_type: context.void_type(),
        }
    }

    fn declare_runtime(&self) -> VppResult<()> {
        let i64 = self.i64_type;
        let f64 = self.f64_type;
        let i32 = self.context.i32_type();
        let i8_ptr = self.i8_ptr_type;
        let void = self.void_type;

        self.module.add_function(
            "vpp_print_int",
            void.fn_type(&[i64.into()], false),
            None,
        );
        self.module.add_function(
            "vpp_print_float",
            void.fn_type(&[f64.into()], false),
            None,
        );
        self.module.add_function(
            "vpp_print_bool",
            void.fn_type(&[i32.into()], false),
            None,
        );
        self.module.add_function(
            "vpp_print_str",
            void.fn_type(&[i8_ptr.into()], false),
            None,
        );
        self.module.add_function(
            "vpp_alloc",
            i8_ptr.fn_type(&[self.context.i64_type().into()], false),
            None,
        );
        self.module.add_function(
            "vpp_make_array",
            i8_ptr.fn_type(&[i64.into(), i64.into()], false),
            None,
        );
        self.module.add_function(
            "vpp_array_len",
            i64.fn_type(&[i8_ptr.into()], false),
            None,
        );
        self.module.add_function(
            "vpp_array_data",
            i8_ptr.fn_type(&[i8_ptr.into()], false),
            None,
        );
        self.module.add_function(
            "vpp_strlen",
            i64.fn_type(&[i8_ptr.into()], false),
            None,
        );

        Ok(())
    }

    fn compile_functions(&mut self, program: &TypedProgram) -> VppResult<()> {
        for func in program.functions.values() {
            let fn_type = self.function_type(&func.params.iter().map(|(_, t)| t.clone()).collect::<Vec<_>>(), &func.ret);
            let function = self.module.add_function(&func.name, fn_type, None);

            for (i, (name, ty)) in func.params.iter().enumerate() {
                let param = function.get_nth_param(i as u32).unwrap();
                param.set_name(name);
                let _ = ty;
            }

            self.functions.insert(func.name.clone(), function);
        }

        for func in program.functions.values() {
            self.compile_function_body(func)?;
        }
        Ok(())
    }

    fn compile_function_body(&mut self, func: &FunctionInfo) -> VppResult<()> {
        let function = *self.functions.get(&func.name).unwrap();
        let entry = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry);

        self.locals.clear();
        self.local_types.clear();

        for (i, (name, ty)) in func.params.iter().enumerate() {
            let alloca = self.builder.build_alloca(self.llvm_type(ty), name).unwrap();
            let param = function.get_nth_param(i as u32).unwrap();
            self.builder.build_store(alloca, param).unwrap();
            self.locals.insert(name.clone(), alloca);
            self.local_types.insert(name.clone(), ty.clone());
        }

        for stmt in &func.body {
            self.compile_stmt(stmt)?;
        }

        if func.ret == Type::Void && self.builder.get_insert_block().unwrap().get_terminator().is_none() {
            self.builder.build_return(None).unwrap();
        } else if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
            let zero = self.i64_type.const_int(0, false);
            self.builder.build_return(Some(&zero)).unwrap();
        }

        function.verify(true);
        Ok(())
    }

    fn compile_main(&mut self, program: &TypedProgram) -> VppResult<()> {
        let fn_type = self.i64_type.fn_type(&[], false);
        let function = self
            .module
            .add_function("main", fn_type, Some(Linkage::External));

        let entry = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry);

        self.locals.clear();
        self.local_types.clear();

        for stmt in &program.top_level {
            self.compile_stmt(stmt)?;
        }

        let zero = self.i64_type.const_int(0, false);
        self.builder.build_return(Some(&zero)).unwrap();
        function.verify(true);
        Ok(())
    }

    fn compile_stmt(&mut self, stmt: &TypedStmt) -> VppResult<()> {
        match stmt {
            TypedStmt::Let { name, ty, value, .. } => {
                let val = self.compile_expr(value)?;
                let alloca = self
                    .builder
                    .build_alloca(self.llvm_type(ty), name)
                    .unwrap();
                self.builder.build_store(alloca, val).unwrap();
                self.locals.insert(name.clone(), alloca);
                self.local_types.insert(name.clone(), ty.clone());
            }
            TypedStmt::Expr(expr) => {
                self.compile_expr(expr)?;
            }
            TypedStmt::If {
                condition,
                then_block,
                else_block,
            } => {
                let cond = self.compile_expr(condition)?;
                let cond_int = self.bool_to_i1(cond);

                let function = self.builder.get_insert_block().unwrap().get_parent().unwrap();
                let then_bb = self.context.append_basic_block(function, "then");
                let else_bb = self.context.append_basic_block(function, "else");
                let merge_bb = self.context.append_basic_block(function, "merge");

                self.builder
                    .build_conditional_branch(cond_int, then_bb, else_bb)
                    .unwrap();

                self.builder.position_at_end(then_bb);
                for s in then_block {
                    self.compile_stmt(s)?;
                }
                if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
                    self.builder.build_unconditional_branch(merge_bb).unwrap();
                }

                self.builder.position_at_end(else_bb);
                if let Some(stmts) = else_block {
                    for s in stmts {
                        self.compile_stmt(s)?;
                    }
                }
                if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
                    self.builder.build_unconditional_branch(merge_bb).unwrap();
                }

                self.builder.position_at_end(merge_bb);
            }
            TypedStmt::While { condition, body } => {
                let function = self.builder.get_insert_block().unwrap().get_parent().unwrap();
                let cond_bb = self.context.append_basic_block(function, "while.cond");
                let body_bb = self.context.append_basic_block(function, "while.body");
                let end_bb = self.context.append_basic_block(function, "while.end");

                self.builder.build_unconditional_branch(cond_bb).unwrap();
                self.builder.position_at_end(cond_bb);
                let cond = self.compile_expr(condition)?;
                let cond_i1 = self.bool_to_i1(cond);
                self.builder
                    .build_conditional_branch(cond_i1, body_bb, end_bb)
                    .unwrap();

                self.builder.position_at_end(body_bb);
                for s in body {
                    self.compile_stmt(s)?;
                }
                if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
                    self.builder.build_unconditional_branch(cond_bb).unwrap();
                }

                self.builder.position_at_end(end_bb);
            }
            TypedStmt::ForInt { var, start, end, body } => {
                let start_val = self.i64_type.const_int(*start as u64, true);
                let end_val = self.i64_type.const_int(*end as u64, true);

                let alloca = self.builder.build_alloca(self.i64_type, var).unwrap();
                self.builder.build_store(alloca, start_val).unwrap();
                self.locals.insert(var.clone(), alloca);
                self.local_types.insert(var.clone(), Type::Int);

                let function = self.builder.get_insert_block().unwrap().get_parent().unwrap();
                let cond_bb = self.context.append_basic_block(function, "for.cond");
                let body_bb = self.context.append_basic_block(function, "for.body");
                let inc_bb = self.context.append_basic_block(function, "for.inc");
                let end_bb = self.context.append_basic_block(function, "for.end");

                self.builder.build_unconditional_branch(cond_bb).unwrap();
                self.builder.position_at_end(cond_bb);
                let cur = self.builder.build_load(self.i64_type, alloca, var).unwrap();
                let cond = self
                    .builder
                    .build_int_compare(IntPredicate::SLT, cur.into_int_value(), end_val, "for.cmp")
                    .unwrap();
                self.builder
                    .build_conditional_branch(cond, body_bb, end_bb)
                    .unwrap();

                self.builder.position_at_end(body_bb);
                for s in body {
                    self.compile_stmt(s)?;
                }
                if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
                    self.builder.build_unconditional_branch(inc_bb).unwrap();
                }

                self.builder.position_at_end(inc_bb);
                let cur = self.builder.build_load(self.i64_type, alloca, var).unwrap();
                let next = self
                    .builder
                    .build_int_add(cur.into_int_value(), self.i64_type.const_int(1, true), "inc")
                    .unwrap();
                self.builder.build_store(alloca, next).unwrap();
                self.builder.build_unconditional_branch(cond_bb).unwrap();

                self.builder.position_at_end(end_bb);
                self.locals.remove(var);
                self.local_types.remove(var);
            }
            TypedStmt::ForArray { var, array, elem_ty, body } => {
                let arr_val = self.compile_expr(array)?;
                let arr_ptr = self.expr_to_ptr(arr_val, array.ty())?;

                let len_fn = self.module.get_function("vpp_array_len").unwrap();
                let len = self.call_basic(
                    self.builder
                        .build_call(len_fn, &[arr_ptr.into()], "len")
                        .unwrap(),
                );

                let idx_alloca = self.builder.build_alloca(self.i64_type, "idx").unwrap();
                self.builder
                    .build_store(idx_alloca, self.i64_type.const_int(0, true))
                    .unwrap();

                let function = self.builder.get_insert_block().unwrap().get_parent().unwrap();
                let cond_bb = self.context.append_basic_block(function, "farr.cond");
                let body_bb = self.context.append_basic_block(function, "farr.body");
                let inc_bb = self.context.append_basic_block(function, "farr.inc");
                let end_bb = self.context.append_basic_block(function, "farr.end");

                self.builder.build_unconditional_branch(cond_bb).unwrap();
                self.builder.position_at_end(cond_bb);
                let idx = self.builder.build_load(self.i64_type, idx_alloca, "idx").unwrap();
                let cond = self
                    .builder
                    .build_int_compare(IntPredicate::SLT, idx.into_int_value(), len.into_int_value(), "farr.cmp")
                    .unwrap();
                self.builder
                    .build_conditional_branch(cond, body_bb, end_bb)
                    .unwrap();

                self.builder.position_at_end(body_bb);
                let data_fn = self.module.get_function("vpp_array_data").unwrap();
                let data_ptr = self
                    .call_basic(
                        self.builder
                            .build_call(data_fn, &[arr_ptr.into()], "data")
                            .unwrap(),
                    )
                    .into_pointer_value();

                let elem_ptr = unsafe {
                    self.builder.build_gep(
                        self.llvm_type(elem_ty),
                        data_ptr,
                        &[idx.into_int_value()],
                        var,
                    ).unwrap()
                };
                let elem_val = self.builder.build_load(self.llvm_type(elem_ty), elem_ptr, var).unwrap();
                let elem_alloca = self.builder.build_alloca(self.llvm_type(elem_ty), var).unwrap();
                self.builder.build_store(elem_alloca, elem_val).unwrap();
                self.locals.insert(var.clone(), elem_alloca);
                self.local_types.insert(var.clone(), elem_ty.clone());

                for s in body {
                    self.compile_stmt(s)?;
                }
                if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
                    self.builder.build_unconditional_branch(inc_bb).unwrap();
                }

                self.builder.position_at_end(inc_bb);
                self.locals.remove(var);
                self.local_types.remove(var);
                let idx = self.builder.build_load(self.i64_type, idx_alloca, "idx").unwrap();
                let next = self
                    .builder
                    .build_int_add(idx.into_int_value(), self.i64_type.const_int(1, true), "inc")
                    .unwrap();
                self.builder.build_store(idx_alloca, next).unwrap();
                self.builder.build_unconditional_branch(cond_bb).unwrap();

                self.builder.position_at_end(end_bb);
            }
            TypedStmt::Return { value } => {
                if let Some(val) = value {
                    let compiled = self.compile_expr(val)?;
                    self.builder.build_return(Some(&compiled)).unwrap();
                } else {
                    self.builder.build_return(None).unwrap();
                }
            }
            TypedStmt::Block(stmts) => {
                self.push_scope();
                for s in stmts {
                    self.compile_stmt(s)?;
                }
                self.pop_scope();
            }
            TypedStmt::Match { .. } => {
                return Err(VppError::Other {
                    message: "native codegen for `match` is not implemented yet; use `vpp run` (interpreter)".to_string(),
                });
            }
            TypedStmt::Break | TypedStmt::Continue => {
                return Err(VppError::Other {
                    message: "native codegen for `break`/`continue` is not implemented yet; use `vpp run` (interpreter)".to_string(),
                });
            }
        }
        Ok(())
    }

    fn push_scope(&mut self) {
        // For MVP, blocks share the same locals map; future: scoped bindings
    }

    fn pop_scope(&mut self) {
        // placeholder
    }

    fn compile_expr(&mut self, expr: &TypedExpr) -> VppResult<BasicValueEnum<'ctx>> {
        match expr {
            TypedExpr::Int(v) => Ok(self.i64_type.const_int(*v as u64, true).into()),
            TypedExpr::Float(v) => Ok(self.f64_type.const_float(*v).into()),
            TypedExpr::Bool(v) => Ok(self
                .i1_type
                .const_int(if *v { 1 } else { 0 }, false)
                .into()),
            TypedExpr::String(s) => {
                let global = self
                    .builder
                    .build_global_string_ptr(s, "str")
                    .unwrap();
                Ok(global.as_pointer_value().into())
            }
            TypedExpr::Ident { name, .. } => {
                let ptr = *self.locals.get(name).ok_or_else(|| VppError::Other {
                    message: format!("codegen: undefined local `{name}`"),
                })?;
                let ty = self.local_types.get(name).cloned().unwrap_or(Type::Int);
                Ok(self
                    .builder
                    .build_load(self.llvm_type(&ty), ptr, name)
                    .unwrap())
            }
            TypedExpr::Binary { op, left, right, ty } => {
                let l = self.compile_expr(left)?;
                let r = self.compile_expr(right)?;
                self.compile_binary(*op, l, r, ty)
            }
            TypedExpr::Unary { op, expr, .. } => {
                let val = self.compile_expr(expr)?;
                match op {
                    crate::ast::UnOp::Not => {
                        let i1 = self.bool_to_i1(val);
                        Ok(self
                            .builder
                            .build_not(i1, "not")
                            .unwrap()
                            .into())
                    }
                    crate::ast::UnOp::Neg => {
                        if val.is_int_value() {
                            Ok(self
                                .builder
                                .build_int_neg(val.into_int_value(), "neg")
                                .unwrap()
                                .into())
                        } else {
                            Ok(self
                                .builder
                                .build_float_neg(val.into_float_value(), "neg")
                                .unwrap()
                                .into())
                        }
                    }
                }
            }
            TypedExpr::Call { name, args, .. } => self.compile_call(name, args),
            TypedExpr::Index { target, index, ty } => {
                let arr_val = self.compile_expr(target)?;
                let arr_ptr = self.expr_to_ptr(arr_val, target.ty())?;
                let idx = self.compile_expr(index)?.into_int_value();

                let data_fn = self.module.get_function("vpp_array_data").unwrap();
                let data_ptr = self
                    .call_basic(
                        self.builder
                            .build_call(data_fn, &[arr_ptr.into()], "data")
                            .unwrap(),
                    )
                    .into_pointer_value();

                let elem_ptr = unsafe {
                    self.builder.build_gep(
                        self.llvm_type(ty),
                        data_ptr,
                        &[idx],
                        "elem",
                    ).unwrap()
                };
                Ok(self
                    .builder
                    .build_load(self.llvm_type(ty), elem_ptr, "load")
                    .unwrap())
            }
            TypedExpr::Array { elements, ty } => {
                let Type::Array(elem_ty) = ty else {
                    return Err(VppError::Other {
                        message: "expected array type".to_string(),
                    });
                };
                let len = elements.len() as i64;
                let elem_size = self.type_size(elem_ty) as i64;
                let make_fn = self.module.get_function("vpp_make_array").unwrap();
                let arr = self
                    .call_basic(
                        self.builder
                            .build_call(
                                make_fn,
                                &[
                                    self.i64_type.const_int(len as u64, true).into(),
                                    self.i64_type.const_int(elem_size as u64, true).into(),
                                ],
                                "arr",
                            )
                            .unwrap(),
                    )
                    .into_pointer_value();

                let data_fn = self.module.get_function("vpp_array_data").unwrap();
                let data_ptr = self
                    .call_basic(
                        self.builder
                            .build_call(data_fn, &[arr.into()], "data")
                            .unwrap(),
                    )
                    .into_pointer_value();

                for (i, elem) in elements.iter().enumerate() {
                    let val = self.compile_expr(elem)?;
                    let ptr = unsafe {
                        self.builder.build_gep(
                            self.llvm_type(elem_ty),
                            data_ptr,
                            &[self.i64_type.const_int(i as u64, true)],
                            "slot",
                        ).unwrap()
                    };
                    self.builder.build_store(ptr, val).unwrap();
                }

                Ok(arr.into())
            }
            TypedExpr::Assign { name, value, .. } => {
                let val = self.compile_expr(value)?;
                let ptr = *self.locals.get(name).ok_or_else(|| VppError::Other {
                    message: format!("codegen: undefined local `{name}`"),
                })?;
                self.builder.build_store(ptr, val).unwrap();
                Ok(val)
            }
            TypedExpr::Field { .. } => Err(VppError::Other {
                message: "native codegen for struct field access is not implemented yet; use `vpp run`".to_string(),
            }),
            TypedExpr::StructLit { .. } => Err(VppError::Other {
                message: "native codegen for struct literals is not implemented yet; use `vpp run`".to_string(),
            }),
            TypedExpr::Variant { .. } => Err(VppError::Other {
                message: "native codegen for enums/Option/Result is not implemented yet; use `vpp run`".to_string(),
            }),
            TypedExpr::Match { .. } => Err(VppError::Other {
                message: "native codegen for `match` expressions is not implemented yet; use `vpp run`".to_string(),
            }),
        }
    }

    fn compile_call(&mut self, name: &str, args: &[TypedExpr]) -> VppResult<BasicValueEnum<'ctx>> {
        if name == "print" {
            for arg in args {
                match arg.ty() {
                    Type::Int => {
                        let val = self.compile_expr(arg)?.into_int_value();
                        let f = self.module.get_function("vpp_print_int").unwrap();
                        self.builder.build_call(f, &[val.into()], "print").unwrap();
                    }
                    Type::Float => {
                        let val = self.compile_expr(arg)?.into_float_value();
                        let f = self.module.get_function("vpp_print_float").unwrap();
                        self.builder.build_call(f, &[val.into()], "print").unwrap();
                    }
                    Type::Bool => {
                        let compiled = self.compile_expr(arg)?;
                        let val = self.bool_to_i1(compiled);
                        let i32 = self
                            .builder
                            .build_int_z_extend(val, self.context.i32_type(), "b32")
                            .unwrap();
                        let f = self.module.get_function("vpp_print_bool").unwrap();
                        self.builder.build_call(f, &[i32.into()], "print").unwrap();
                    }
                    Type::String => {
                        let val = self.compile_expr(arg)?.into_pointer_value();
                        let f = self.module.get_function("vpp_print_str").unwrap();
                        self.builder.build_call(f, &[val.into()], "print").unwrap();
                    }
                    other => {
                        return Err(VppError::Other {
                            message: format!("cannot print type {}", other.name()),
                        });
                    }
                }
            }
            return Ok(self.i64_type.const_int(0, true).into());
        }

        if name == "len" {
            let arg = &args[0];
            match arg.ty() {
                Type::String => {
                    let ptr = self.compile_expr(arg)?.into_pointer_value();
                    let f = self.module.get_function("vpp_strlen").unwrap();
                    return Ok(self.call_basic(
                        self.builder
                            .build_call(f, &[ptr.into()], "len")
                            .unwrap(),
                    ));
                }
                Type::Array(_) => {
                    let val = self.compile_expr(arg)?;
                    let ptr = self.expr_to_ptr(val, arg.ty())?;
                    let f = self.module.get_function("vpp_array_len").unwrap();
                    return Ok(self.call_basic(
                        self.builder
                            .build_call(f, &[ptr.into()], "len")
                            .unwrap(),
                    ));
                }
                other => {
                    return Err(VppError::Other {
                        message: format!("len unsupported for {}", other.name()),
                    });
                }
            }
        }

        let function = *self.functions.get(name).ok_or_else(|| VppError::Other {
            message: format!("undefined function `{name}`"),
        })?;

        let mut compiled_args: Vec<BasicMetadataValueEnum> = Vec::new();
        for arg in args {
            compiled_args.push(self.compile_expr(arg)?.into());
        }

        Ok(self
            .call_basic(
                self.builder
                    .build_call(function, &compiled_args, "call")
                    .unwrap(),
            ))
    }

    fn compile_binary(
        &self,
        op: BinOp,
        left: BasicValueEnum<'ctx>,
        right: BasicValueEnum<'ctx>,
        result_ty: &Type,
    ) -> VppResult<BasicValueEnum<'ctx>> {
        match op {
            BinOp::Add if *result_ty == Type::String => {
                // MVP: string concat not fully implemented — error at typecheck for now
                Err(VppError::Other {
                    message: "string concatenation codegen not yet implemented".to_string(),
                })
            }
            BinOp::Add if left.is_int_value() => Ok(self
                .builder
                .build_int_add(left.into_int_value(), right.into_int_value(), "add")
                .unwrap()
                .into()),
            BinOp::Add => Ok(self
                .builder
                .build_float_add(left.into_float_value(), right.into_float_value(), "add")
                .unwrap()
                .into()),
            BinOp::Sub if left.is_int_value() => Ok(self
                .builder
                .build_int_sub(left.into_int_value(), right.into_int_value(), "sub")
                .unwrap()
                .into()),
            BinOp::Sub => Ok(self
                .builder
                .build_float_sub(left.into_float_value(), right.into_float_value(), "sub")
                .unwrap()
                .into()),
            BinOp::Mul if left.is_int_value() => Ok(self
                .builder
                .build_int_mul(left.into_int_value(), right.into_int_value(), "mul")
                .unwrap()
                .into()),
            BinOp::Mul => Ok(self
                .builder
                .build_float_mul(left.into_float_value(), right.into_float_value(), "mul")
                .unwrap()
                .into()),
            BinOp::Div if left.is_int_value() => Ok(self
                .builder
                .build_int_signed_div(left.into_int_value(), right.into_int_value(), "div")
                .unwrap()
                .into()),
            BinOp::Div => Ok(self
                .builder
                .build_float_div(left.into_float_value(), right.into_float_value(), "div")
                .unwrap()
                .into()),
            BinOp::Mod => Ok(self
                .builder
                .build_int_signed_rem(left.into_int_value(), right.into_int_value(), "mod")
                .unwrap()
                .into()),
            BinOp::Eq => Ok(self
                .builder
                .build_int_compare(
                    if left.is_int_value() {
                        IntPredicate::EQ
                    } else {
                        IntPredicate::EQ
                    },
                    self.value_as_cmp_int(left),
                    self.value_as_cmp_int(right),
                    "eq",
                )
                .unwrap()
                .into()),
            BinOp::NotEq => Ok(self
                .builder
                .build_int_compare(IntPredicate::NE, self.value_as_cmp_int(left), self.value_as_cmp_int(right), "ne")
                .unwrap()
                .into()),
            BinOp::Lt => self.build_cmp(IntPredicate::SLT, left, right),
            BinOp::LtEq => self.build_cmp(IntPredicate::SLE, left, right),
            BinOp::Gt => self.build_cmp(IntPredicate::SGT, left, right),
            BinOp::GtEq => self.build_cmp(IntPredicate::SGE, left, right),
            BinOp::And => Ok(self
                .builder
                .build_and(self.bool_to_i1(left), self.bool_to_i1(right), "and")
                .unwrap()
                .into()),
            BinOp::Or => Ok(self
                .builder
                .build_or(self.bool_to_i1(left), self.bool_to_i1(right), "or")
                .unwrap()
                .into()),
        }
    }

    fn build_cmp(
        &self,
        pred: IntPredicate,
        left: BasicValueEnum<'ctx>,
        right: BasicValueEnum<'ctx>,
    ) -> VppResult<BasicValueEnum<'ctx>> {
        if left.is_int_value() {
            Ok(self
                .builder
                .build_int_compare(pred, left.into_int_value(), right.into_int_value(), "cmp")
                .unwrap()
                .into())
        } else {
            let pred_f = match pred {
                IntPredicate::SLT => inkwell::FloatPredicate::OLT,
                IntPredicate::SLE => inkwell::FloatPredicate::OLE,
                IntPredicate::SGT => inkwell::FloatPredicate::OGT,
                IntPredicate::SGE => inkwell::FloatPredicate::OGE,
                _ => inkwell::FloatPredicate::OEQ,
            };
            Ok(self
                .builder
                .build_float_compare(pred_f, left.into_float_value(), right.into_float_value(), "fcmp")
                .unwrap()
                .into())
        }
    }

    fn value_as_cmp_int(&self, val: BasicValueEnum<'ctx>) -> inkwell::values::IntValue<'ctx> {
        if val.is_int_value() {
            val.into_int_value()
        } else {
            self.bool_to_i1(val)
        }
    }

    fn bool_to_i1(&self, val: BasicValueEnum<'ctx>) -> inkwell::values::IntValue<'ctx> {
        if val.is_int_value() && val.get_type().into_int_type().get_bit_width() == 1 {
            val.into_int_value()
        } else if val.is_int_value() {
            self.builder
                .build_int_compare(
                    IntPredicate::NE,
                    val.into_int_value(),
                    self.i64_type.const_int(0, true),
                    "tobool",
                )
                .unwrap()
        } else {
            val.into_int_value()
        }
    }

    fn expr_to_ptr(&self, val: BasicValueEnum<'ctx>, ty: Type) -> VppResult<PointerValue<'ctx>> {
        match ty {
            Type::Array(_) | Type::String => {
                if val.is_pointer_value() {
                    Ok(val.into_pointer_value())
                } else {
                    Err(VppError::Other {
                        message: "expected pointer value".to_string(),
                    })
                }
            }
            other => Err(VppError::Other {
                message: format!("cannot take pointer of {}", other.name()),
            }),
        }
    }

    fn llvm_type(&self, ty: &Type) -> BasicTypeEnum<'ctx> {
        match ty {
            Type::Int => self.i64_type.into(),
            Type::Float => self.f64_type.into(),
            Type::Bool => self.i1_type.into(),
            Type::String => self.i8_ptr_type.into(),
            Type::Array(inner) => self.llvm_type(inner),
            Type::Void => self.i64_type.into(),
            _ => self.i64_type.into(),
        }
    }

    fn function_type(&self, params: &[Type], ret: &Type) -> FunctionType<'ctx> {
        let param_types: Vec<BasicMetadataTypeEnum> = params
            .iter()
            .map(|t| self.llvm_type(t).into())
            .collect();
        self.llvm_type(ret).fn_type(&param_types, false)
    }

    fn call_basic(&self, call: CallSiteValue<'ctx>) -> BasicValueEnum<'ctx> {
        call.try_as_basic_value().unwrap_basic()
    }

    fn type_size(&self, ty: &Type) -> u64 {
        match ty {
            Type::Int => 8,
            Type::Float => 8,
            Type::Bool => 1,
            Type::String => 8,
            Type::Array(inner) => self.type_size(inner),
            _ => 8,
        }
    }
}
