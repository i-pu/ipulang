use std::collections::HashMap;

use anyhow::{anyhow, bail, ensure, Result};
use ipulang_parser::{
    nodes::{
        Assign, BinOp, Call, Const, Expr, For, FunctionDecl, IfElse, Op, Program, Stmt, Stmts,
        Variable, VariableDecl,
    },
    types::Type,
};

struct Env {
    /// 変数の情報
    variables: HashMap<String, HashMap<String, Type>>,
    /// 現在のfunction
    function_id: Option<String>,
    functions: HashMap<String, (Vec<Type>, Type)>,
}

impl Env {
    fn new() -> Self {
        let mut functions = HashMap::new();
        // decrare putchar(i32): i32
        functions.insert("putchar".to_owned(), (vec![Type::Int32], Type::Int32));
        // decrate getchar(): i32
        functions.insert("getchar".to_owned(), (vec![], Type::Int32));
        functions.insert("printstr".to_owned(), (vec![Type::String], Type::Unit));
        functions.insert("printi32".to_owned(), (vec![Type::Int32], Type::Unit));
        functions.insert("printi64".to_owned(), (vec![Type::Int64], Type::Unit));

        Self {
            variables: HashMap::new(),
            function_id: None,
            functions,
        }
    }

    /// 変数の型情報を取得する
    fn get_var_type(&mut self, var_name: &str) -> Option<Type> {
        self.function_id
            .clone()
            .and_then(|id| self.variables.get(&id).and_then(|m| m.get(var_name))).copied()
    }

    /// 変数に型情報を設定する
    fn set_var_type(&mut self, var_name: String, typ: Type) {
        let map = self
            .variables
            .get_mut(&self.function_id.clone().unwrap())
            .unwrap();
        map.insert(var_name, typ);
    }

    fn get_current_fn_type(&self) -> Option<(Vec<Type>, Type)> {
        self.function_id
            .clone()
            .and_then(|a| self.get_fn_type(&a))
    }
    fn get_fn_type(&self, fn_name: &String) -> Option<(Vec<Type>, Type)> {
        self.functions.get(fn_name).cloned()
    }
}

pub fn type_check(mut program: Program) -> Result<Program> {
    let mut env = Env::new();
    program.type_check(&mut env)?;
    Ok(program)
}

trait TypeCheck {
    fn type_check(&mut self, env: &mut Env) -> Result<Type>;
}

impl TypeCheck for Const {
    fn type_check(&mut self, _env: &mut Env) -> Result<Type> {
        match self {
            Const::Unit => Ok(Type::Unit),
            Const::I32Const(_) => Ok(Type::Int32),
            Const::I64Const(_) => Ok(Type::Int64),
            Const::BoolConst(_) => Ok(Type::Bool),
            Const::String(_) => Ok(Type::String),
        }
    }
}

impl<'a> TypeCheck for BinOp<'a> {
    fn type_check(&mut self, env: &mut Env) -> Result<Type> {
        let left_typ = self.left.type_check(env)?;
        let right_typ = self.right.type_check(env)?;

        // 型チェック
        ensure!(
            left_typ == right_typ,
            "op: {:?}, type mismatch!, {:?} != {:?}",
            self.op,
            left_typ,
            right_typ,
        );

        // 型を設定
        self.ty = match self.op {
            Op::Or | Op::And | Op::Eq | Op::Neq | Op::Geq | Op::Leq | Op::Gt | Op::Lt => Type::Bool,
            Op::Add | Op::Sub | Op::Mul | Op::Div | Op::Mod => left_typ,
        };

        ensure!(self.ty != Type::Unknown, "type is still unknown",);
        Ok(self.ty)
    }
}

impl<'a> TypeCheck for Variable<'a> {
    fn type_check(&mut self, env: &mut Env) -> Result<Type> {
        if let Some(typ) = env.get_var_type(&self.id) {
            Ok(typ)
        } else {
            Err(anyhow!("variable not found: {}", &self.id))
        }
    }
}

impl<'a> TypeCheck for Call<'a> {
    fn type_check(&mut self, env: &mut Env) -> Result<Type> {
        let func_name = self.id.clone();
        if let Some(func_type) = env.functions.get(&func_name).cloned() {
            // 引数の数をチェック
            ensure!(
                func_type.0.len() == self.args.len(),
                "function {} takes {} arguments, but {} given",
                func_name,
                func_type.0.len(),
                self.args.len(),
            );
            // 引数の型をチェック
            for (arg, param_type) in self.args.iter_mut().zip(func_type.0.iter()) {
                let arg_typ = arg.type_check(env)?;
                ensure!(
                    arg_typ == *param_type,
                    format!("type mismatch!, {:?} != {:?}", arg_typ, param_type,),
                );
            }
            Ok(func_type.1)
        } else {
            bail!("function not found: {}", func_name);
        }
    }
}

impl<'a> TypeCheck for Expr<'a> {
    fn type_check(&mut self, env: &mut Env) -> Result<Type> {
        match self {
            Expr::Const(c) => c.type_check(env),
            Expr::BinOp(bin_op) => bin_op.type_check(env),
            Expr::Variable(var) => var.type_check(env),
            Expr::Call(call) => call.type_check(env),
        }
    }
}

impl<'a> TypeCheck for IfElse<'a> {
    fn type_check(&mut self, env: &mut Env) -> Result<Type> {
        let cond_typ = self.cond.type_check(env)?;
        self.success.type_check(env)?;
        self.failure.as_mut().map(|ref mut f| f.type_check(env));

        ensure!(
            cond_typ == Type::Bool,
            "{:?} type mismatch!, {:?} != {:?}",
            self.cond,
            cond_typ,
            Type::Bool,
        );
        Ok(Type::Unit)
    }
}

impl<'a> TypeCheck for Assign<'a> {
    fn type_check(&mut self, env: &mut Env) -> Result<Type> {
        // TODO: 型推論
        if let Some(var_typ) = env.get_var_type(&self.left) {
            let right_typ = self.right.type_check(env)?;

            ensure!(
                var_typ == right_typ,
                "type mismatch!, {:?} != {:?}",
                var_typ,
                right_typ,
            );
        } else {
            bail!("var {} is not found", &self.left);
        }
        Ok(Type::Unit)
    }
}

impl<'a> TypeCheck for For<'a> {
    fn type_check(&mut self, env: &mut Env) -> Result<Type> {
        self.var_decl.type_check(env)?;
        self.cond.type_check(env)?;
        self.assign.type_check(env)?;
        self.stmts.type_check(env)?;
        Ok(Type::Unit)
    }
}

impl<'a> TypeCheck for VariableDecl<'a> {
    fn type_check(&mut self, env: &mut Env) -> Result<Type> {
        if let Some(init) = self.init.as_mut() {
            let init_ty = init.type_check(env)?;
            ensure!(
                self.ty == init_ty,
                "var decl type is not equal. {} != {}",
                self.ty,
                init_ty
            );
        }
        env.set_var_type(self.id.clone(), self.ty);
        Ok(Type::Unit)
    }
}

impl<'a> TypeCheck for Stmt<'a> {
    fn type_check(&mut self, env: &mut Env) -> Result<Type> {
        match self {
            Stmt::Expr(expr) => {
                expr.type_check(env)?;
            }
            Stmt::Return(ret) => {
                let expr_ty = ret.type_check(env)?;
                if let Some((_, ret_ty)) = env.get_current_fn_type() {
                    ensure!(
                        ret_ty == expr_ty,
                        "return expr type {} != fn return type {}",
                        expr_ty,
                        ret_ty
                    );
                } else {
                    bail!("return is out of scope function");
                }
            }
            Stmt::VariableDecl(vd) => {
                vd.type_check(env)?;
            }
            Stmt::Assign(assign) => {
                assign.type_check(env)?;
            }
            Stmt::IfElse(if_else) => {
                if_else.type_check(env)?;
            }
            Stmt::For(f) => {
                f.type_check(env)?;
            }
        }
        Ok(Type::Unit)
    }
}

impl<'a> TypeCheck for Stmts<'a> {
    fn type_check(&mut self, env: &mut Env) -> Result<Type> {
        for stmt in self.0.iter_mut() {
            stmt.type_check(env)?;
        }
        Ok(Type::Unit)
    }
}

impl<'a> TypeCheck for FunctionDecl<'a> {
    fn type_check(&mut self, env: &mut Env) -> Result<Type> {
        env.function_id = Some(self.id.clone());
        if env
            .functions
            .insert(
                self.id.clone(),
                (
                    self.args.iter().map(|arg| arg.ty).collect(),
                    self.ret_typ,
                ),
            )
            .is_some()
        {
            bail!("function {} is already decleared", &self.id);
        } else {
            env.variables.entry(self.id.clone()).or_default();
        }
        for arg in self.args.iter() {
            env.set_var_type(arg.id.clone(), arg.ty);
        }

        self.stmts.type_check(env)?;

        env.function_id = None;

        Ok(Type::Unit)
    }
}

impl<'a> TypeCheck for Program<'a> {
    fn type_check(&mut self, env: &mut Env) -> Result<Type> {
        for function in self.0.iter_mut() {
            function.type_check(env)?;
        }
        Ok(Type::Unit)
    }
}
