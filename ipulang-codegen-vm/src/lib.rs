use anyhow::Result;
use instructions::register_instructions;
use ipulang_parser::nodes::{Const, Expr, FunctionDecl, Program, Stmt};
use operand::Operand;
use stack_vm::{Builder, Code, FromByteCode, Machine, ToByteCode, WriteManyTable};

pub(crate) mod instructions;
pub(crate) mod operand;

trait Codegen {
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
            Stmt::VariableDecl(_) => todo!(),
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
            Expr::Variable(_) => todo!(),
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
            Const::I32Const(i) => builder.push("push", vec![Operand(*i as i64)]),
            Const::I64Const(i) => builder.push("push", vec![Operand(*i)]),
            Const::BoolConst(b) => builder.push("push", vec![Operand(*b as i64)]),
        }
    }
}

pub fn code_gen(ast: Program) -> Result<Vec<u8>> {
    let instruction_table = register_instructions();
    let mut builder: Builder<Operand> = Builder::new(&instruction_table);

    for func in ast.0.iter() {
        func.code_gen(&mut builder);
    }

    let code: Code<Operand> = builder.into();
    println!("{:?}", code);

    let mut bin: Vec<u8> = vec![];
    code.to_byte_code(&mut bin);
    Ok(bin)
}

pub fn execute(bin: &mut &[u8]) {
    let instruction_table = register_instructions();
    let code = Code::<Operand>::from_byte_code(bin);

    let constants: WriteManyTable<Operand> = WriteManyTable::new();
    let mut machine = Machine::new(code, &constants, &instruction_table);
    machine.run();
    println!("{:#?}", machine.operand_stack);
}
