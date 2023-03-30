use std::sync::atomic::AtomicUsize;

use crate::operand::Operand;
use ipulang_parser::nodes::{Const, Expr, FunctionDecl, Stmt};
use stack_vm::Builder;

const COUNT: AtomicUsize = AtomicUsize::new(0);

fn issue_label() -> Operand {
    let i = COUNT.fetch_add(1, std::sync::atomic::Ordering::Acquire);
    Operand::Label(format!("%{}", i))
}

pub(crate) trait Codegen {
    fn code_gen(&self, builder: &mut Builder<Operand>);
}

impl Codegen for FunctionDecl<'_> {
    fn code_gen(&self, builder: &mut Builder<Operand>) {
        if self.id != "main" {
            builder.label(self.id.as_str());
        }
        for stmt in self.stmts.0.iter() {
            stmt.code_gen(builder);
        }
    }
}

impl Codegen for Stmt<'_> {
    fn code_gen(&self, builder: &mut Builder<Operand>) {
        match self {
            Stmt::Expr(expr) => expr.code_gen(builder),
            Stmt::Return(_) => todo!(),
            Stmt::VariableDecl(decl) => {
                if let Some(init) = decl.init.as_ref() {
                    init.code_gen(builder);
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
    fn code_gen(&self, builder: &mut Builder<Operand>) {
        match self {
            Expr::Const(c) => c.code_gen(builder),
            Expr::Variable(var) => {
                builder.push("load", vec![Operand::Label(var.id.clone())]);
            }
            Expr::BinOp(binop) => {
                binop.left.code_gen(builder);
                binop.right.code_gen(builder);
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
                    ipulang_parser::nodes::Op::Sub => todo!(),
                    ipulang_parser::nodes::Op::Mul => todo!(),
                    ipulang_parser::nodes::Op::Div => todo!(),
                    ipulang_parser::nodes::Op::Mod => todo!(),
                }
            }
            Expr::Call(_) => todo!(),
        }
    }
}

impl Codegen for Const {
    fn code_gen(&self, builder: &mut Builder<Operand>) {
        match self {
            Const::I32Const(i) => builder.push("push", vec![Operand::Imm(*i as i64)]),
            Const::I64Const(i) => builder.push("push", vec![Operand::Imm(*i)]),
            Const::BoolConst(b) => builder.push("push", vec![Operand::Imm(*b as i64)]),
        }
    }
}
