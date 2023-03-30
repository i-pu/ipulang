use anyhow::Result;
use instructions::register_instructions;
use ipulang_parser::nodes::Program;
use operand::Operand;
use stack_vm::{Builder, Code, FromByteCode, Machine, ToByteCode, WriteManyTable};

use crate::codegen::Codegen;

pub(crate) mod codegen;
pub(crate) mod instructions;
pub(crate) mod operand;

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
