use std::collections::HashMap;
use std::collections::HashSet;

use anyhow::{Error, Result};
use inkwell;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::module::Module;
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
struct Env<'a> {
    module: Module<'a>,
    ctx: &'a Context,
    /// 宣言されている変数一覧
    variables: HashMap<String, PointerValue<'a>>,
    /// compilerが作った一時変数の個数
    var_count: usize,
    /// 宣言されている関数一覧
    // TODO: 後から宣言できるようにする
    functions: HashSet<String>,

    /// 現在の builder
    builder: Builder<'a>,
}

impl<'a> Env<'a> {
    fn new(ctx: &'a Context) -> Self {
        // module
        let module = ctx.create_module("main");
        Self {
            ctx: ctx,
            module: module,
            variables: HashMap::new(),
            var_count: 0,
            functions: HashSet::new(),
            builder: ctx.create_builder(),
        }
    }
}

impl Const {
    fn gen_code<'a>(self, env: &Env<'a>) -> IntValue<'a> {
        dbg!("code_gen: Const");
        let i32_type = env.ctx.i32_type();
        i32_type.const_int(self.0 as u64, false)
    }
}

impl BinOp {
    fn gen_code<'a>(self, env: &mut Env<'a>) -> PointerValue<'a> {
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
        let zero = i32_type.const_int(0, false);
        let ptr: PointerValue = env.builder.build_alloca(i32_type, &self.0);
        env.builder.build_store(ptr, zero);
        env.variables.insert(self.0.clone(), ptr);
        dbg!("end: VariableDecl");
    }
}

impl Variable {
    fn gen_code<'a>(self, env: &Env<'a>) -> PointerValue<'a> {
        return env.variables.get(&self.0).unwrap().clone();
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
        }
    }
}

impl FunctionDecl {
    fn gen_code(self, env: &mut Env) {
        // FIXME: mainのみしか対応していない
        let i32_type = env.ctx.i32_type();
        let main_fn_type = i32_type.fn_type(&[i32_type.into()], false);
        let main_fn = env.module.add_function(&self.id, main_fn_type, None);

        // block
        let basic_block = env.ctx.append_basic_block(main_fn, "entry");

        env.builder.position_at_end(basic_block);

        // 引数を使う時
        // let param0 = main_fn.get_nth_param(0).unwrap().into_int_value();

        self.stmts.gen_code(env);
        let zero = i32_type.const_int(0, false);
        env.builder.build_return(Some(&zero));
    }
}

impl Stmt {
    fn gen_code<'a>(self, env: &mut Env<'a>) {
        dbg!(&self);
        match self {
            Stmt::Expr(expr) => {
                expr.gen_code(env);
            }
            Stmt::VariableDecl(decl) => {
                decl.gen_code(env);
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
