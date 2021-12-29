use std::collections::HashMap;

use anyhow::{anyhow, bail, ensure, Result};

use crate::nodes::*;
use crate::types::*;

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
        let putchar_type = (vec![Type::Int32], Type::Int32);
        functions.insert("putchar".to_owned(), putchar_type);

        // decrate getchar(): i32
        functions.insert("getchar".to_owned(), (vec![], Type::Int32));

        Self {
            variables: HashMap::new(),
            function_id: None,
            functions: functions,
        }
    }

    /// 変数の型情報を取得する
    fn get_var_type(&mut self, var_name: &str) -> Option<Type> {
        self.function_id
            .clone()
            .map(|id| self.variables.get(&id).map(|m| m.get(var_name)).flatten())
            .flatten()
            .map(|ty| *ty)
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
            .map(|a| self.get_fn_type(&a))
            .flatten()
    }
    fn get_fn_type(&self, fn_name: &String) -> Option<(Vec<Type>, Type)> {
        self.functions.get(fn_name).map(|ty| ty.clone())
    }
}

pub fn type_check(mut program: Program) -> Result<Program> {
    let mut env = Env::new();
    program.type_check(&mut env)?;
    Ok(program)
}

impl Const {
    fn type_check(&mut self, env: &mut Env) -> Result<Type> {
        match self {
            Const::I32Const(_) => Ok(Type::Int32),
            Const::I64Const(_) => Ok(Type::Int64),
            Const::BoolConst(_) => Ok(Type::Bool),
            // Const::String(_) => Ok(Type::String),
        }
    }
}

impl BinOp {
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

impl Variable {
    fn type_check(&mut self, env: &mut Env) -> Result<Type> {
        if let Some(typ) = env.get_var_type(&self.id) {
            Ok(typ)
        } else {
            Err(anyhow!("variable not found: {}", &self.id))
        }
    }
}

impl Call {
    fn type_check(&mut self, env: &mut Env) -> Result<Type> {
        let func_name = self.id.clone();
        if let Some(func_type) = env.functions.get(&func_name).map(|a| a.clone()) {
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

impl Expr {
    fn type_check(&mut self, env: &mut Env) -> Result<Type> {
        match self {
            Expr::Const(c) => c.type_check(env),
            Expr::BinOp(bin_op) => bin_op.type_check(env),
            Expr::Variable(var) => var.type_check(env),
            Expr::Call(call) => call.type_check(env),
        }
    }
}

impl IfElse {
    fn type_check(&mut self, env: &mut Env) -> Result<()> {
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
        Ok(())
    }
}

impl Assign {
    fn type_check(&mut self, env: &mut Env) -> Result<()> {
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
        Ok(())
    }
}

impl For {
    fn type_check(&mut self, env: &mut Env) -> Result<()> {
        self.var_decl.type_check(env)?;
        self.cond.type_check(env)?;
        self.assign.type_check(env)?;
        self.stmts.type_check(env)?;
        Ok(())
    }
}

impl VariableDecl {
    fn type_check(&mut self, env: &mut Env) -> Result<()> {
        if let Some(init) = self.init.as_mut() {
            let init_ty = init.type_check(env)?;
            ensure!(
                self.ty == init_ty,
                "var decl type is not equal. {} != {}",
                self.ty,
                init_ty
            );
        }
        env.set_var_type(self.id.clone(), self.ty.clone());
        Ok(())
    }
}

impl Stmt {
    fn type_check(&mut self, env: &mut Env) -> Result<()> {
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
            Stmt::VariableDecl(vd) => vd.type_check(env)?,
            Stmt::Assign(assign) => assign.type_check(env)?,
            Stmt::IfElse(if_else) => {
                if_else.type_check(env)?;
            }
            Stmt::For(f) => {
                f.type_check(env)?;
            }
        }
        Ok(())
    }
}

impl Stmts {
    fn type_check(&mut self, env: &mut Env) -> Result<()> {
        for stmt in self.0.iter_mut() {
            stmt.type_check(env)?;
        }
        Ok(())
    }
}

impl FunctionDecl {
    fn type_check(&mut self, env: &mut Env) -> Result<()> {
        env.function_id = Some(self.id.clone());
        if env
            .functions
            .insert(
                self.id.clone(),
                (
                    self.args.iter().map(|arg| arg.ty.clone()).collect(),
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

        Ok(())
    }
}

impl Program {
    fn type_check(&mut self, env: &mut Env) -> Result<()> {
        for function in self.0.iter_mut() {
            function.type_check(env)?;
        }
        Ok(())
    }
}
