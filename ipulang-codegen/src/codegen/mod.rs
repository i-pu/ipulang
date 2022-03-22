pub mod context;

use anyhow::{Error, Result};
use inkwell;
use inkwell::context::Context;
use inkwell::types::*;
use inkwell::values::*;
use ipulang_parser::nodes::*;
use ipulang_parser::types::Type;

use self::context::Env;

type VoidValue<'ll> = IntValue<'ll>;

pub fn code_gen(ast: Program) -> Result<String, Box<Error>> {
    let context = Context::create();
    let mut env = Env::new(&context);
    ast.code_gen(&mut env);
    Ok(env.module.print_to_string().to_string())
}

trait CodeGen<'ll, T: 'll + AnyValue<'ll>> {
    fn code_gen(self, env: &mut Env<'ll>) -> Option<T>;
}

impl<'ll> CodeGen<'ll, IntValue<'ll>> for Const {
    fn code_gen(self, env: &mut Env<'ll>) -> Option<IntValue<'ll>> {
        match self {
            Const::I32Const(i) => Some(env.ctx.i32_type().const_int(i as u64, false)),
            Const::I64Const(i) => Some(env.ctx.i64_type().const_int(i as u64, false)),
            Const::BoolConst(b) => Some(env.ctx.bool_type().const_int(b as u64, false)),
            _ => panic!("not support"),
        }
    }
}

impl<'ll> CodeGen<'ll, PointerValue<'ll>> for BinOp<'ll> {
    fn code_gen(self, env: &mut Env<'ll>) -> Option<PointerValue<'ll>> {
        let bool_type = env.ctx.bool_type();

        let tmp_id = env.get_tmp_var_id();
        let ptr_lhr = self.left.code_gen(env).unwrap();
        let load_lhs = env.builder.build_load(ptr_lhr, &tmp_id).into_int_value();

        let tmp_id = env.get_tmp_var_id();
        let ptr_rhr = self.right.code_gen(env).unwrap();
        let load_rhs = env.builder.build_load(ptr_rhr, &tmp_id).into_int_value();
        let tmp_id = env.get_tmp_var_id();

        let (tmp, result_type) = match self.op {
            Op::Or => (env.builder.build_or(load_lhs, load_rhs, &tmp_id), bool_type),
            Op::And => (
                env.builder.build_and(load_lhs, load_rhs, &tmp_id),
                bool_type,
            ),
            Op::Eq => (
                env.builder.build_int_compare(
                    inkwell::IntPredicate::EQ,
                    load_lhs,
                    load_rhs,
                    &tmp_id,
                ),
                bool_type,
            ),
            Op::Neq => (
                env.builder.build_int_compare(
                    inkwell::IntPredicate::NE,
                    load_lhs,
                    load_rhs,
                    &tmp_id,
                ),
                bool_type,
            ),
            Op::Geq => (
                env.builder.build_int_compare(
                    inkwell::IntPredicate::SGE,
                    load_lhs,
                    load_rhs,
                    &tmp_id,
                ),
                bool_type,
            ),
            Op::Leq => (
                env.builder.build_int_compare(
                    inkwell::IntPredicate::SLE,
                    load_lhs,
                    load_rhs,
                    &tmp_id,
                ),
                bool_type,
            ),
            Op::Gt => (
                env.builder.build_int_compare(
                    inkwell::IntPredicate::SGT,
                    load_lhs,
                    load_rhs,
                    &tmp_id,
                ),
                bool_type,
            ),
            Op::Lt => (
                env.builder.build_int_compare(
                    inkwell::IntPredicate::SLT,
                    load_lhs,
                    load_rhs,
                    &tmp_id,
                ),
                bool_type,
            ),
            Op::Add => (
                env.builder.build_int_add(load_lhs, load_rhs, &tmp_id),
                load_lhs.get_type(),
            ),
            Op::Sub => (
                env.builder.build_int_sub(load_lhs, load_rhs, &tmp_id),
                load_lhs.get_type(),
            ),
            Op::Mul => (
                env.builder.build_int_mul(load_lhs, load_rhs, &tmp_id),
                load_lhs.get_type(),
            ),
            Op::Div => (
                env.builder
                    .build_int_signed_div(load_lhs, load_rhs, &tmp_id),
                load_lhs.get_type(),
            ),
            Op::Mod => (
                env.builder
                    .build_int_signed_rem(load_lhs, load_rhs, &tmp_id),
                load_lhs.get_type(),
            ),
        };

        let tmp_id = env.get_tmp_var_id();
        let ptr = env.builder.build_alloca(result_type, &tmp_id);
        env.builder.build_store(ptr, tmp);
        Some(ptr)
    }
}

impl<'ll> CodeGen<'ll, IntValue<'ll>> for VariableDecl<'ll> {
    fn code_gen(self, env: &mut Env<'ll>) -> Option<IntValue<'ll>> {
        let var_type = match self.ty {
            Type::Int32 => env.ctx.i32_type(),
            Type::Int64 => env.ctx.i64_type(),
            Type::Bool => env.ctx.bool_type(),
            _ => panic!("ty: {:?} is unknown", self.ty),
        };
        if let Some(init) = self.init {
            let init_ptr = init.code_gen(env).unwrap();
            let tmp_id = env.get_tmp_var_id();
            let tmp = env.builder.build_load(init_ptr, &tmp_id);
            let ptr: PointerValue = env.builder.build_alloca(var_type, &self.id);
            env.builder.build_store(ptr, tmp.into_int_value());

            env.set_variable(self.id.clone(), ptr);
        } else {
            let ptr: PointerValue = env.builder.build_alloca(var_type, &self.id);
            let zero = env.ctx.i32_type().const_int(0, false);
            env.builder.build_store(ptr, zero);
            env.set_variable(self.id.clone(), ptr);
        }
        None
    }
}

impl<'ll> CodeGen<'ll, PointerValue<'ll>> for Variable<'ll> {
    fn code_gen(self, env: &mut Env<'ll>) -> Option<PointerValue<'ll>> {
        Some(env.get_variable(self.id).unwrap().clone())
    }
}

impl<'ll> CodeGen<'ll, PointerValue<'ll>> for Expr<'ll> {
    fn code_gen(self, env: &mut Env<'ll>) -> Option<PointerValue<'ll>> {
        match self {
            Expr::Const(cns) => {
                let tmp = cns.code_gen(env).unwrap();
                let tmp_id = env.get_tmp_var_id();
                let ptr = env.builder.build_alloca(tmp.get_type(), &tmp_id);
                env.builder.build_store(ptr, tmp);
                Some(ptr)
            }
            Expr::BinOp(bin_op) => bin_op.code_gen(env),
            Expr::Variable(var) => var.code_gen(env),
            Expr::Call(call) => {
                // always returns value
                let call_id = call.id.clone();
                let callsitevalu = call.code_gen(env).unwrap();
                let tmp_id = env.get_tmp_var_id();
                let f = env.functions.get(&call_id).unwrap();
                // 関数がvoidを返すならNoneを返す
                if let Some(ret_type) = f.get_type().get_return_type() {
                    match ret_type {
                        BasicTypeEnum::IntType(int_type)
                            if int_type == env.ctx.i32_type() || int_type == env.ctx.i64_type() =>
                        {
                            let ptr = env.builder.build_alloca(int_type, &tmp_id);
                            env.builder.build_store(
                                ptr,
                                callsitevalu.try_as_basic_value().left().unwrap(),
                            );
                            Some(ptr)
                        }
                        _ => panic!(""),
                    }
                } else {
                    None
                }
            }
        }
    }
}

impl<'ll> CodeGen<'ll, CallSiteValue<'ll>> for Call<'ll> {
    fn code_gen(self, env: &mut Env<'ll>) -> Option<CallSiteValue<'ll>> {
        // eval exprs
        let mut evaluated_args: Vec<BasicMetadataValueEnum> = vec![];
        for arg in self.args {
            let evaluated_ptr = arg.code_gen(env).unwrap();
            let var_id = env.get_tmp_var_id();
            let evaluated = env.builder.build_load(evaluated_ptr, &var_id);
            evaluated_args.push(evaluated.into());
        }

        let function = env.module.get_function(&self.id).unwrap();
        let var_id = env.get_tmp_var_id();
        Some(env.builder.build_call(function, &evaluated_args, &var_id))
    }
}

impl<'ll> CodeGen<'ll, VoidValue<'ll>> for IfElse<'ll> {
    fn code_gen(self, env: &mut Env<'ll>) -> Option<VoidValue<'ll>> {
        // generate cond, success, failure block
        let ptr = self.cond.code_gen(env).unwrap();
        let var_id = env.get_tmp_var_id();
        let res = env.builder.build_load(ptr, &var_id).into_int_value();

        // cond != 0
        let zero = res.get_type().const_int(0, false);
        let var_id = env.get_tmp_var_id();
        let cond = env
            .builder
            .build_int_compare(inkwell::IntPredicate::NE, res, zero, &var_id);

        // make success, failure, dest label
        let success_label = env.get_tmp_label_id();
        let fn_value = env.function_value.clone().unwrap();
        let success_block = env.ctx.append_basic_block(fn_value, &success_label);
        let tmp_label = env.get_tmp_label_id();
        let failure_block = env.ctx.append_basic_block(fn_value, &tmp_label);
        let label_id = env.get_tmp_label_id();
        let dest_block = env.ctx.append_basic_block(fn_value, &label_id);

        env.builder
            .build_conditional_branch(cond, success_block, failure_block);

        env.builder.position_at_end(success_block);
        // then_block is always exists
        self.success.code_gen(env);
        env.builder.build_unconditional_branch(dest_block);

        env.builder.position_at_end(failure_block);
        // else
        if let Some(failure) = self.failure {
            failure.code_gen(env);
        };
        env.builder.build_unconditional_branch(dest_block);

        env.builder.position_at_end(dest_block);
        None
    }
}

impl<'ll> CodeGen<'ll, VoidValue<'ll>> for Assign<'ll> {
    fn code_gen(self, env: &mut Env<'ll>) -> Option<IntValue<'ll>> {
        let ptr_right = self.right.code_gen(env).unwrap();
        let tmp_id = env.get_tmp_var_id();
        if let Some(ptr_left) = env.get_variable(self.left.clone()) {
            env.builder.build_store(
                ptr_left.clone(),
                env.builder.build_load(ptr_right, &tmp_id).into_int_value(),
            );
        } else {
            panic!("variable {} is not found.", self.left)
        }
        None
    }
}

impl<'ll> CodeGen<'ll, VoidValue<'ll>> for For<'ll> {
    fn code_gen(self, env: &mut Env<'ll>) -> Option<IntValue<'ll>> {
        //   <decl>
        //   jmp cond
        // cond:
        //   <cond>
        //   jmp <do> or <dest>
        // do:
        //   <stmts>
        //   jmp update
        // update:
        //   <update>
        //   jmp <cond>
        // dest:
        let cond_label = env.get_tmp_label_id();
        let fn_value = env.function_value.clone().unwrap();
        let cond_block = env.ctx.append_basic_block(fn_value, &cond_label);

        let do_label = env.get_tmp_label_id();
        let do_block = env.ctx.append_basic_block(fn_value, &do_label);
        let update_id = env.get_tmp_label_id();
        let update_block = env.ctx.append_basic_block(fn_value, &update_id);
        let dest_id = env.get_tmp_label_id();
        let dest_block = env.ctx.append_basic_block(fn_value, &dest_id);

        self.var_decl.code_gen(env);
        env.builder.build_unconditional_branch(cond_block);

        // generate cond
        env.builder.position_at_end(cond_block);
        let ptr = self.cond.code_gen(env).unwrap();
        let var_id = env.get_tmp_var_id();
        let res = env.builder.build_load(ptr, &var_id).into_int_value();
        // cond != 0
        let zero = res.get_type().const_int(0, false);
        let var_id = env.get_tmp_var_id();
        let cond = env
            .builder
            .build_int_compare(inkwell::IntPredicate::NE, res, zero, &var_id);

        // jmp do: if cond else dest:
        env.builder
            .build_conditional_branch(cond, do_block, dest_block);

        // generate do
        env.builder.position_at_end(do_block);
        self.stmts.code_gen(env);

        // jmp update:
        env.builder.build_unconditional_branch(update_block);

        // generate update
        env.builder.position_at_end(update_block);
        self.assign.code_gen(env);
        env.builder.build_unconditional_branch(cond_block);

        // jmp dest:
        env.builder.position_at_end(dest_block);
        None
    }
}

impl<'ll> CodeGen<'ll, VoidValue<'ll>> for Stmt<'ll> {
    fn code_gen(self, env: &mut Env<'ll>) -> Option<VoidValue<'ll>> {
        match self {
            Stmt::Expr(expr) => {
                Some(expr.code_gen(env));
            }
            Stmt::VariableDecl(decl) => {
                Some(decl.code_gen(env));
            }
            Stmt::Return(expr) => {
                let ptr = expr.code_gen(env);
                if let Some(pv) = ptr {
                    let tmp_id = env.get_tmp_var_id();
                    let tmp = env.builder.build_load(pv, &tmp_id);
                    env.builder.build_return(Some(&tmp));
                } else {
                    env.builder.build_return(None);
                }
            }
            Stmt::Assign(assign) => {
                Some(assign.code_gen(env));
            }
            Stmt::IfElse(if_else) => {
                Some(if_else.code_gen(env));
            }
            Stmt::For(for_) => {
                Some(for_.code_gen(env));
            }
        };
        None
    }
}

impl<'ll> CodeGen<'ll, VoidValue<'ll>> for Stmts<'ll> {
    fn code_gen(self, env: &mut Env<'ll>) -> Option<VoidValue<'ll>> {
        for stmt in self.0 {
            stmt.code_gen(env);
        }
        None
    }
}

impl<'ll> CodeGen<'ll, IntValue<'ll>> for FunctionDecl<'ll> {
    fn code_gen(self, env: &mut Env<'ll>) -> Option<IntValue<'ll>> {
        let llvm_ret_typ = env.get_llvm_fn_type(self.ret_typ.clone());

        let fn_type: FunctionType = if let Some(llvm_ret_type) = llvm_ret_typ {
            llvm_ret_type.fn_type(
                &self
                    .args
                    .iter()
                    .map(|arg| env.get_llvm_value_type(arg.ty.clone()))
                    .collect::<Vec<BasicMetadataTypeEnum>>(),
                false,
            )
        } else {
            env.ctx.void_type().fn_type(
                &self
                    .args
                    .iter()
                    .map(|arg| env.get_llvm_value_type(arg.ty.clone()))
                    .collect::<Vec<BasicMetadataTypeEnum>>(),
                false,
            )
        };

        let fn_value = env.module.add_function(&self.id, fn_type, None);

        // 現在の関数の情報を設定
        env.function_value = Some(fn_value.clone());
        env.function = self.id.clone();
        dbg!(&env.functions);
        if env.functions.insert(self.id.clone(), fn_value).is_some() {
            panic!("function {} is already decleared", &self.id);
        }
        // 変数マップを作る
        env.variables.entry(self.id.clone()).or_default();

        // block
        // TODO: main() だけでいいのか？
        let basic_block = env.ctx.append_basic_block(fn_value, "entry");
        env.builder.position_at_end(basic_block);

        // 引数を使う時
        for (i, arg) in self.args.iter().enumerate() {
            let param = fn_value.get_nth_param(i as u32).unwrap().into_int_value();

            // TODO: type check

            // 引数名に対応するptrを作成
            let ptr_param = env.int_to_point(param, Some(&arg.id.clone()));
            env.set_variable(arg.id.clone(), ptr_param);
        }

        self.stmts.code_gen(env);

        // returnがないときも0をかえすようにしている
        if llvm_ret_typ.is_none() {
            env.builder.build_return(None);
        }

        env.function_value = None;
        env.function = "".to_owned();
        None
    }
}

impl<'ll> CodeGen<'ll, IntValue<'ll>> for Program<'ll> {
    fn code_gen(self, env: &mut Env<'ll>) -> Option<IntValue<'ll>> {
        for function in self.0 {
            function.code_gen(env);
        }
        None
    }
}
