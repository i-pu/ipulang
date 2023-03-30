use std::collections::HashMap;

use crate::operand::Operand;
use ipulang_parser::nodes::{Assign, Const, Expr, FunctionDecl, Stmt, Stmts, VariableDecl};
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
        self.stmts.code_gen(ctx, builder);
    }
}

impl Codegen for Stmts<'_> {
    fn code_gen(&self, ctx: &mut Ctx<'_>, builder: &mut Builder<Operand>) {
        for stmt in self.0.iter() {
            stmt.code_gen(ctx, builder);
        }
    }
}

impl Codegen for VariableDecl<'_> {
    fn code_gen(&self, ctx: &mut Ctx<'_>, builder: &mut Builder<Operand>) {
        if let Some(init) = self.init.as_ref() {
            init.code_gen(ctx, builder);
        } else {
            builder.push("push", vec![Operand::Imm(0)]);
        };
        builder.push("store", vec![Operand::Label(self.id.clone())]);
    }
}

impl Codegen for Assign<'_> {
    fn code_gen(&self, ctx: &mut Ctx<'_>, builder: &mut Builder<Operand>) {
        self.right.code_gen(ctx, builder);
        builder.push("store", vec![Operand::Label(self.left.clone())]);
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
            Stmt::VariableDecl(decl) => decl.code_gen(ctx, builder),
            Stmt::Assign(assign) => assign.code_gen(ctx, builder),
            Stmt::IfElse(ifelse) => {
                // <cond>
                // jump_if_zero .failure (has else-branch) .end (otherwise)
                // <success>
                // jump .end
                // .failure
                // <failure>
                // .end
                // ...
                ifelse.cond.code_gen(ctx, builder);

                let i = builder.labels.keys().len();
                let failure_label = format!("failure_{}", i);
                let end_label = format!("fi_{}", i);
                if ifelse.failure.is_some() {
                    builder.push("jump_if_zero", vec![Operand::Label(failure_label.clone())]);
                } else {
                    builder.push("jump_if_zero", vec![Operand::Label(end_label.clone())]);
                }
                ifelse.success.code_gen(ctx, builder);
                builder.push("jump", vec![Operand::Label(end_label.clone())]);
                if let Some(failure) = ifelse.failure.as_ref() {
                    builder.label(&failure_label);
                    failure.code_gen(ctx, builder);
                }
                builder.label(&end_label);
            }
            Stmt::For(fr) => {
                //  <decl>
                // .start:
                //  <stmts>
                //  <cond>
                //  jump_if_zero .end
                //  <assign>
                //  jump .start
                // .end:
                //  ...
                fr.var_decl.code_gen(ctx, builder);
                let i = builder.labels.keys().len();
                let start_label = format!("for_{}_start", i);
                let end_label = format!("for_{}_end", i);
                builder.label(&start_label);
                fr.cond.code_gen(ctx, builder);
                builder.push("jump_if_zero", vec![Operand::Label(end_label.clone())]);
                fr.stmts.code_gen(ctx, builder);
                fr.assign.code_gen(ctx, builder);
                builder.push("jump", vec![Operand::Label(start_label.clone())]);
                builder.label(&end_label);
            }
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
                    ipulang_parser::nodes::Op::Or => {
                        builder.push("or", vec![]);
                    }
                    ipulang_parser::nodes::Op::And => {
                        builder.push("and", vec![]);
                    }
                    ipulang_parser::nodes::Op::Eq => {
                        builder.push("eq", vec![]);
                    }
                    ipulang_parser::nodes::Op::Neq => {
                        builder.push("neq", vec![]);
                    }
                    ipulang_parser::nodes::Op::Geq => {
                        builder.push("geq", vec![]);
                    }
                    ipulang_parser::nodes::Op::Leq => {
                        builder.push("leq", vec![]);
                    }
                    ipulang_parser::nodes::Op::Gt => {
                        builder.push("gt", vec![]);
                    }
                    ipulang_parser::nodes::Op::Lt => {
                        builder.push("lt", vec![]);
                    }
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
                    ipulang_parser::nodes::Op::Mod => {
                        builder.push("mod", vec![]);
                    }
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
