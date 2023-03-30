use std::collections::HashMap;

use crate::operand::Operand;
use ipulang_parser::nodes::{Const, Expr, FunctionDecl, Stmt};
use stack_vm::Builder;

pub(crate) trait Codegen {
    fn code_gen(&self, ctx: &mut Ctx<'_>, builder: &mut Builder<Operand>);
}

pub struct Ctx<'a> {
    pub functions: HashMap<String, FunctionDecl<'a>>,
}

impl Codegen for FunctionDecl<'_> {
    fn code_gen(&self, ctx: &mut Ctx<'_>, builder: &mut Builder<Operand>) {
        builder.label(self.id.as_str());
        for stmt in self.stmts.0.iter() {
            stmt.code_gen(ctx, builder);
        }
    }
}

impl Codegen for Stmt<'_> {
    fn code_gen(&self, ctx: &mut Ctx, builder: &mut Builder<Operand>) {
        match self {
            Stmt::Expr(expr) => expr.code_gen(ctx, builder),
            Stmt::Return(ret) => {
                ret.code_gen(ctx, builder);
                builder.push("ret", vec![]);
            }
            Stmt::VariableDecl(decl) => {
                if let Some(init) = decl.init.as_ref() {
                    init.code_gen(ctx, builder);
                } else {
                    builder.push("push", vec![Operand::Imm(0)]);
                };
                builder.push("store", vec![Operand::Label(decl.id.clone())]);
            }
            Stmt::Assign(_) => todo!(),
            Stmt::IfElse(_) => todo!(),
            Stmt::For(_) => todo!(),
        }
    }
}

impl Codegen for Expr<'_> {
    fn code_gen(&self, ctx: &mut Ctx, builder: &mut Builder<Operand>) {
        match self {
            Expr::Const(c) => c.code_gen(ctx, builder),
            Expr::Variable(var) => {
                builder.push("load", vec![Operand::Label(var.id.clone())]);
            }
            Expr::BinOp(binop) => {
                binop.left.code_gen(ctx, builder);
                binop.right.code_gen(ctx, builder);
                match binop.op {
                    ipulang_parser::nodes::Op::Or => todo!(),
                    ipulang_parser::nodes::Op::And => todo!(),
                    ipulang_parser::nodes::Op::Eq => todo!(),
                    ipulang_parser::nodes::Op::Neq => todo!(),
                    ipulang_parser::nodes::Op::Geq => todo!(),
                    ipulang_parser::nodes::Op::Leq => todo!(),
                    ipulang_parser::nodes::Op::Gt => todo!(),
                    ipulang_parser::nodes::Op::Lt => todo!(),
                    ipulang_parser::nodes::Op::Add => {
                        builder.push("add", vec![]);
                    }
                    ipulang_parser::nodes::Op::Sub => {
                        builder.push("sub", vec![]);
                    }
                    ipulang_parser::nodes::Op::Mul => {
                        builder.push("mul", vec![]);
                    }
                    ipulang_parser::nodes::Op::Div => {
                        builder.push("div", vec![]);
                    }
                    ipulang_parser::nodes::Op::Mod => todo!(),
                }
            }
            Expr::Call(call) => {
                let func = ctx
                    .functions
                    .get(&call.id)
                    .expect(format!("function {} not found", call.id).as_str());
                let params = func.args.clone();
                for arg in call.args.iter() {
                    arg.code_gen(ctx, builder);
                }
                for param in params.iter() {
                    builder.push("store", vec![Operand::Label(param.id.clone())]);
                }
                builder.push("call", vec![Operand::Label(call.id.clone())]);
                builder.push("inspect", vec![]);
            }
        }
    }
}

impl Codegen for Const {
    fn code_gen(&self, ctx: &mut Ctx, builder: &mut Builder<Operand>) {
        match self {
            Const::I32Const(i) => builder.push("push", vec![Operand::Imm(*i as i64)]),
            Const::I64Const(i) => builder.push("push", vec![Operand::Imm(*i)]),
            Const::BoolConst(b) => builder.push("push", vec![Operand::Imm(*b as i64)]),
        }
    }
}
