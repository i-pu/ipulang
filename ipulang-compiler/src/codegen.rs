use std::collections::HashMap;
use std::collections::HashSet;

use anyhow::{Error, Result};
use inkwell;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::module::Module;
use inkwell::values::BasicMetadataValueEnum;
use inkwell::values::CallSiteValue;
use inkwell::values::IntValue;
use inkwell::values::PointerValue;
use inkwell::OptimizationLevel;

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
    var_count: usize,
    /// 宣言されている関数一覧
    // TODO: 後から宣言できるようにする
    functions: HashSet<String>,

    /// 現在の builder
    builder: Builder<'ctx>,

    /// 現在のfunction id
    function: String,
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
            var_count: 0,
            functions: functions,
            builder: ctx.create_builder(),
            function: "".to_owned(),
        }
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
        self.var_count += 1;
        let tmp = self.builder.build_load(ptr, &self.var_count.to_string());
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
            self.var_count += 1;
            let ptr: PointerValue = self
                .builder
                .build_alloca(i32_type, &self.var_count.to_string());
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
        let ptr_lhr = self.left.gen_code(env);
        let ptr_rhr = self.right.gen_code(env);
        let builder = &env.builder;
        env.var_count += 1;
        let load_lhr = builder
            .build_load(ptr_lhr, &env.var_count.to_string())
            .into_int_value();
        env.var_count += 1;

        let load_rhr = builder
            .build_load(ptr_rhr, &env.var_count.to_string())
            .into_int_value();
        env.var_count += 1;

        let tmp = match self.op {
            Op::Add => builder.build_int_add(load_lhr, load_rhr, &env.var_count.to_string()),
            Op::Sub => builder.build_int_sub(load_lhr, load_rhr, &env.var_count.to_string()),
            Op::Mul => builder.build_int_mul(load_lhr, load_rhr, &env.var_count.to_string()),
            Op::Div => builder.build_int_signed_div(load_lhr, load_rhr, &env.var_count.to_string()),
        };

        env.var_count += 1;
        let ptr = builder.build_alloca(env.ctx.i32_type(), &env.var_count.to_string());
        builder.build_store(ptr, tmp);
        ptr
    }
}

impl VariableDecl {
    fn gen_code<'a>(self, env: &mut Env<'a>) {
        dbg!(&self);
        let i32_type = env.ctx.i32_type();
        if let Some(init) = self.init {
            let tmp_ptr = init.gen_code(env);
            env.var_count += 1;
            let tmp = env.builder.build_load(tmp_ptr, &env.var_count.to_string());
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
                env.var_count += 1;
                let ptr = env
                    .builder
                    .build_alloca(env.ctx.i32_type(), &env.var_count.to_string());
                env.builder.build_store(ptr, tmp);
                ptr
            }
            Expr::BinOp(bin_op) => bin_op.gen_code(env),
            Expr::Variable(var) => var.gen_code(env),
            Expr::Call(call) => {
                // always returns value
                let callsitevalu = call.gen_code(env).try_as_basic_value().left().unwrap();

                env.var_count += 1;
                let ptr = env
                    .builder
                    .build_alloca(env.ctx.i32_type(), &env.var_count.to_string());
                env.builder.build_store(ptr, callsitevalu);
                ptr
            }
        }
    }
}

impl FunctionDecl {
    fn gen_code(self, env: &mut Env) {
        // FIXME: mainのみしか対応していない
        let i32_type = env.ctx.i32_type();
        // let main_fn_type = i32_type.fn_type(&[i32_type.into()], false);
        let fn_type = i32_type.fn_type(&vec![i32_type.into(); self.args.len()], false);
        let fn_value = env.module.add_function(&self.id, fn_type, None);

        if !env.functions.insert(self.id.clone()) {
            panic!("function {} is already decleared", &self.id);
        }
        // 変数マップを作る
        env.variables.entry(self.id.clone()).or_default();
        env.function = self.id.clone();

        // block
        let basic_block = env.ctx.append_basic_block(fn_value, "entry");
        env.builder.position_at_end(basic_block);

        // 引数を使う時
        // let param0 = main_fn.get_nth_param(0).unwrap().into_int_value();
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
            env.var_count += 1;
            let evaluated = env
                .builder
                .build_load(evaluated_ptr, &env.var_count.to_string());
            evaluated_args.push(evaluated.into());
        }

        let function = env.module.get_function(&self.id).unwrap();
        env.var_count += 1;
        env.builder
            .build_call(function, &evaluated_args, &env.var_count.to_string())
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
                env.var_count += 1;
                let tmp = env.builder.build_load(ptr, &env.var_count.to_string());
                env.builder.build_return(Some(&tmp));
            }
            Stmt::Assing(assign) => {
                let ptr_right = assign.right.gen_code(env);
                if let Some(ptr_left) = env.get_variable(assign.left.clone()) {
                    env.builder.build_store(
                        ptr_left.clone(),
                        env.builder
                            .build_load(ptr_right, &env.var_count.to_string())
                            .into_int_value(),
                    );
                } else {
                    panic!("variable {} is not found.", assign.left)
                }
            }
        };
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_add() {
//         let ast = BinOp::new(
//             Expr::Const(Const::new(3)),
//             Op::Add,
//             Expr::Const(Const::new(3)),
//         );
//         let code = ast.gen_code();
//         assert_eq!(
//             code,
//             r#"lit 3
// lit 3
// add
// "#
//         )
//     }
// }
