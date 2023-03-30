use anyhow::Result;
use codegen::Ctx;
use instructions::register_instructions;
use ipulang_parser::nodes::{Program};
use operand::Operand;
use stack_vm::{Builder, Code, FromByteCode, Machine, ToByteCode, WriteManyTable, WriteOnceTable};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

use crate::codegen::Codegen;

pub(crate) mod codegen;
pub(crate) mod instructions;
pub(crate) mod operand;

pub fn code_gen(ast: Program) -> Result<Vec<u8>> {
    let instruction_table = register_instructions();
    let mut ctx = Ctx {
        functions: HashMap::from_iter(
            ast.0
                .iter()
                .map(|fun| (fun.id.clone(), fun.clone()))
                .collect::<Vec<_>>(),
        ),
    };
    let mut builder: Builder<Operand> = Builder {
        instruction_table: &instruction_table,
        instructions: vec![],
        labels: WriteOnceTable::new(),
        data: vec![],
    };
    for func in ast.0.iter() {
        dbg!(&func);
        func.code_gen(&mut ctx, &mut builder);
    }

    let code: Code<Operand> = builder.into();
    let mut file = File::create("debug.vm")?;
    write!(file, "{:?}", code)?;

    let mut bin: Vec<u8> = vec![];
    code.to_byte_code(&mut bin);
    Ok(bin)
}

pub fn execute(bin: &mut &[u8]) {
    let instruction_table = register_instructions();
    let code = Code::<Operand>::from_byte_code(bin);

    let constants: WriteManyTable<Operand> = WriteManyTable::new();
    let mut machine = Machine::new(code, &constants, &instruction_table);
    machine.jump("main");
    machine.run();
    // println!("ret = {:?}", machine.operand_pop());
    // println!("{:#?}", machine.operand_stack);
}
