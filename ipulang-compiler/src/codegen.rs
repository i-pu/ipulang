use std::cell::Cell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

use anyhow::{Error, Result};
use inkwell;
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::IntType;
use inkwell::values::BasicMetadataValueEnum;
use inkwell::values::CallSiteValue;
use inkwell::values::FunctionValue;
use inkwell::values::IntValue;
use inkwell::values::PointerValue;

use crate::ast::program_parser;
use crate::nodes::*;

pub fn compile(code: String) -> Result<String, Box<Error>> {
    let ast = program_parser(&code);
    let context = Context::create();
    let mut env = Env::new(&context);

    ast.gen_code(&mut env);
    Ok(env.module.print_to_string().to_string())
}

/// コード生成時のための情報
struct Env<'ctx> {
    module: Module<'ctx>,
    ctx: &'ctx Context,
    /// function id -> variable id -> PointerValue
    /// 宣言されている変数一覧
    variables: HashMap<String, HashMap<String, PointerValue<'ctx>>>,
    /// compilerが作った一時変数の個数
    var_count: Rc<Cell<usize>>,
    /// 宣言されている関数一覧
    // TODO: 後から宣言できるようにする
    functions: HashSet<String>,

    /// 現在の builder
    builder: Builder<'ctx>,

    /// 現在のfunction id
    function: String,
    /// 現在の FunctionValue
    function_value: Option<FunctionValue<'ctx>>,
}

impl<'ctx> Env<'ctx> {
    fn new(ctx: &'ctx Context) -> Self {
        // module
        let module = ctx.create_module("main");

        let mut functions = HashSet::new();

        // decrare builtin function
        let i32_type = ctx.i32_type();
        let putchar_type = i32_type.fn_type(&[i32_type.into()], false);
        functions.insert("putchar".to_owned());
        module.add_function("putchar", putchar_type, None);

        Self {
            ctx: ctx,
            module: module,
            variables: HashMap::new(),
            var_count: Rc::new(Cell::new(0)),
            functions: functions,
            builder: ctx.create_builder(),
            function: "".to_owned(),
            function_value: None,
        }
    }

    fn get_tmp_var_id(&self) -> String {
        let tmp = self.var_count.clone();
        (*tmp).set(tmp.get() + 1);
        format!("_v{}", self.var_count.get().to_string())
    }

    fn get_tmp_label_id(&self) -> String {
        let tmp = self.var_count.clone();
        (*tmp).set(tmp.get() + 1);
        format!("label{}", self.var_count.get().to_string())
    }

    /// 関数に変数があるかどうか
    fn contains(&self, name: String) -> bool {
        self.variables
            .get(&self.function)
            .map(|m: &HashMap<String, PointerValue>| m.contains_key(&name))
            .unwrap_or(false)
    }

    /// 関数に宣言されている変数を取得
    fn get_variable(&self, name: String) -> Option<&'_ PointerValue<'ctx>> {
        self.variables
            .get(&self.function)
            .map(|m| m.get(&name))
            .flatten()
    }

    /// 関数に変数情報を登録する
    fn set_variable(&mut self, name: String, value: PointerValue<'ctx>) {
        let mut map = self.variables.get_mut(&self.function).unwrap();
        map.insert(name, value);
    }

    /// PointerValueをIntValueに変換する
    /// IntValueの名前は任意
    fn point_to_int(&mut self, ptr: PointerValue<'ctx>, int_id: Option<String>) -> IntValue {
        let var_id = self.get_tmp_var_id();
        let tmp = self.builder.build_load(ptr, &var_id);
        tmp.into_int_value()
    }
    /// IntValueをPointerValueに変換する
    /// IntValueの名前は任意
    fn int_to_point(&mut self, int: IntValue, ptr_id: Option<&str>) -> PointerValue<'ctx> {
        let i32_type = self.ctx.i32_type();
        if let Some(ptr_id) = ptr_id {
            let ptr: PointerValue = self.builder.build_alloca(i32_type, ptr_id);
            self.builder.build_store(ptr, int);
            ptr
        } else {
            let var_id = self.get_tmp_var_id();
            let ptr: PointerValue = self.builder.build_alloca(i32_type, &var_id);
            self.builder.build_store(ptr, int);
            ptr
        }
    }
}

impl Const {
    fn gen_code<'ctx>(self, env: &Env<'ctx>) -> IntValue<'ctx> {
        let i32_type = env.ctx.i32_type();
        i32_type.const_int(self.0 as u64, false)
    }
}

impl BinOp {
    fn gen_code<'ctx>(self, env: &mut Env<'ctx>) -> PointerValue<'ctx> {
        let i32_type = env.ctx.i32_type();
        let bool_type = env.ctx.bool_type();
        let ptr_lhr = self.left.gen_code(env);
        let ptr_rhr = self.right.gen_code(env);
        let builder = &env.builder;
        let tmp_id = env.get_tmp_var_id();
        let load_lhs = builder.build_load(ptr_lhr, &tmp_id).into_int_value();

        let tmp_id = env.get_tmp_var_id();
        let load_rhs = builder.build_load(ptr_rhr, &tmp_id).into_int_value();

        let tmp_id = env.get_tmp_var_id();

        let (tmp, result_type) = match self.op {
            Op::Or => (builder.build_or(load_lhs, load_rhs, &tmp_id), bool_type),
            Op::And => (builder.build_and(load_lhs, load_rhs, &tmp_id), bool_type),
            Op::Eq => (
                builder.build_int_compare(inkwell::IntPredicate::EQ, load_lhs, load_rhs, &tmp_id),
                bool_type,
            ),
            Op::Neq => (
                builder.build_int_compare(inkwell::IntPredicate::NE, load_lhs, load_rhs, &tmp_id),
                bool_type,
            ),
            Op::Geq => (
                builder.build_int_compare(inkwell::IntPredicate::SGE, load_lhs, load_rhs, &tmp_id),
                bool_type,
            ),
            Op::Leq => (
                builder.build_int_compare(inkwell::IntPredicate::SLE, load_lhs, load_rhs, &tmp_id),
                bool_type,
            ),
            Op::Gt => (
                builder.build_int_compare(inkwell::IntPredicate::SGT, load_lhs, load_rhs, &tmp_id),
                bool_type,
            ),
            Op::Lt => (
                builder.build_int_compare(inkwell::IntPredicate::SLT, load_lhs, load_rhs, &tmp_id),
                bool_type,
            ),
            Op::Add => (builder.build_int_add(load_lhs, load_rhs, &tmp_id), i32_type),
            Op::Sub => (builder.build_int_sub(load_lhs, load_rhs, &tmp_id), i32_type),
            Op::Mul => (builder.build_int_mul(load_lhs, load_rhs, &tmp_id), i32_type),
            Op::Div => (
                builder.build_int_signed_div(load_lhs, load_rhs, &tmp_id),
                i32_type,
            ),
            Op::Mod => (
                builder.build_int_signed_rem(load_lhs, load_rhs, &tmp_id),
                i32_type,
            ),
        };

        let tmp_id = env.get_tmp_var_id();
        let ptr = builder.build_alloca(result_type, &tmp_id);
        builder.build_store(ptr, tmp);
        ptr
    }
}

impl VariableDecl {
    fn gen_code<'a>(self, env: &mut Env<'a>) {
        dbg!(&self);
        let i32_type = env.ctx.i32_type();
        if let Some(init) = self.init {
            let init_ptr = init.gen_code(env);
            let tmp_id = env.get_tmp_var_id();
            let tmp = env.builder.build_load(init_ptr, &tmp_id);
            let ptr: PointerValue = env.builder.build_alloca(i32_type, &self.id);
            env.builder.build_store(ptr, tmp.into_int_value());

            env.set_variable(self.id.clone(), ptr);
        } else {
            let ptr: PointerValue = env.builder.build_alloca(i32_type, &self.id);
            let zero = i32_type.const_int(0, false);
            env.builder.build_store(ptr, zero);
            env.set_variable(self.id.clone(), ptr);
        }
    }
}

impl Variable {
    fn gen_code<'a>(self, env: &Env<'a>) -> PointerValue<'a> {
        env.get_variable(self.0).unwrap().clone()
    }
}

impl Expr {
    fn gen_code<'a>(self, env: &mut Env<'a>) -> PointerValue<'a> {
        match self {
            Expr::Const(cns) => {
                let tmp = cns.gen_code(env);
                let tmp_id = env.get_tmp_var_id();
                let ptr = env.builder.build_alloca(env.ctx.i32_type(), &tmp_id);
                env.builder.build_store(ptr, tmp);
                ptr
            }
            Expr::BinOp(bin_op) => bin_op.gen_code(env),
            Expr::Variable(var) => var.gen_code(env),
            Expr::Call(call) => {
                // always returns value
                let callsitevalu = call.gen_code(env).try_as_basic_value().left().unwrap();

                let tmp_id = env.get_tmp_var_id();
                let ptr = env.builder.build_alloca(env.ctx.i32_type(), &tmp_id);
                env.builder.build_store(ptr, callsitevalu);
                ptr
            }
        }
    }
}

impl FunctionDecl {
    fn gen_code(self, env: &mut Env) {
        let i32_type = env.ctx.i32_type();
        let fn_type = i32_type.fn_type(&vec![i32_type.into(); self.args.len()], false);
        let fn_value = env.module.add_function(&self.id, fn_type, None);

        // 現在の関数の情報を設定
        env.function_value = Some(fn_value.clone());
        if !env.functions.insert(self.id.clone()) {
            panic!("function {} is already decleared", &self.id);
        }
        // 変数マップを作る
        env.variables.entry(self.id.clone()).or_default();
        env.function = self.id.clone();

        // block
        // TODO: main() だけでいいのか？
        let basic_block = env.ctx.append_basic_block(fn_value, "entry");
        env.builder.position_at_end(basic_block);

        // 引数を使う時
        for (i, arg) in self.args.iter().enumerate() {
            let param = fn_value.get_nth_param(i as u32).unwrap().into_int_value();
            // 引数名に対応するptrを作成
            let ptr_param = env.int_to_point(param, Some(arg));
            env.set_variable(arg.clone(), ptr_param);
        }

        self.stmts.gen_code(env);

        // returnがないときも0をかえすようにしている
        let zero = i32_type.const_int(0, false);
        env.builder.build_return(Some(&zero));
    }
}

impl Call {
    fn gen_code<'a>(self, env: &mut Env<'a>) -> CallSiteValue<'a> {
        // eval exprs
        let mut evaluated_args: Vec<BasicMetadataValueEnum> = vec![];
        for arg in self.args {
            let evaluated_ptr = arg.gen_code(env);
            let var_id = env.get_tmp_var_id();
            let evaluated = env.builder.build_load(evaluated_ptr, &var_id);
            evaluated_args.push(evaluated.into());
        }

        let function = env.module.get_function(&self.id).unwrap();
        let var_id = env.get_tmp_var_id();
        env.builder.build_call(function, &evaluated_args, &var_id)
    }
}

impl IfElse {
    fn gen_code<'a>(self, env: &mut Env<'a>) {
        // generate cond, success, failure block
        let ptr = self.cond.gen_code(env);
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
        self.success.gen_code(env);
        env.builder.build_unconditional_branch(dest_block);

        env.builder.position_at_end(failure_block);
        // else
        if let Some(failure) = self.failure {
            failure.gen_code(env)
        };
        env.builder.build_unconditional_branch(dest_block);

        env.builder.position_at_end(dest_block);
    }
}

impl Stmt {
    fn gen_code<'a>(self, env: &mut Env<'a>) {
        match self {
            Stmt::Expr(expr) => {
                expr.gen_code(env);
            }
            Stmt::VariableDecl(decl) => {
                decl.gen_code(env);
            }
            Stmt::Return(expr) => {
                let ptr = expr.gen_code(env);
                let tmp_id = env.get_tmp_var_id();
                let tmp = env.builder.build_load(ptr, &tmp_id);
                env.builder.build_return(Some(&tmp));
            }
            Stmt::Assign(assign) => {
                assign.gen_code(env);
            }
            Stmt::IfElse(if_else) => {
                if_else.gen_code(env);
            }
            Stmt::For(for_) => {
                for_.gen_code(env);
            }
        };
    }
}

impl Assign {
    fn gen_code<'ctx>(self, env: &mut Env<'ctx>) {
        let ptr_right = self.right.gen_code(env);
        let tmp_id = env.get_tmp_var_id();
        if let Some(ptr_left) = env.get_variable(self.left.clone()) {
            env.builder.build_store(
                ptr_left.clone(),
                env.builder.build_load(ptr_right, &tmp_id).into_int_value(),
            );
        } else {
            panic!("variable {} is not found.", self.left)
        }
    }
}

impl For {
    fn gen_code<'ctx>(self, env: &mut Env<'ctx>) {
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

        self.var_decl.gen_code(env);
        env.builder.build_unconditional_branch(cond_block);

        // generate cond
        env.builder.position_at_end(cond_block);
        let ptr = self.cond.gen_code(env);
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
        self.stmts.gen_code(env);

        // jmp update:
        env.builder.build_unconditional_branch(update_block);

        // generate update
        env.builder.position_at_end(update_block);
        self.assign.gen_code(env);
        env.builder.build_unconditional_branch(cond_block);

        // jmp dest:
        env.builder.position_at_end(dest_block);
    }
}

impl Stmts {
    fn gen_code<'a>(self, env: &mut Env<'a>) {
        for stmt in self.0 {
            stmt.gen_code(env);
        }
    }
}

impl Program {
    fn gen_code<'a>(self, env: &mut Env<'a>) {
        for function in self.0 {
            function.gen_code(env);
        }
    }
}
