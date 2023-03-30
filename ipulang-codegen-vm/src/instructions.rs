use crate::operand::Operand;
use stack_vm::{Instruction, InstructionTable, Machine};

/// stora stack top value as args[0]
fn store(machine: &mut Machine<Operand>, args: &[usize]) {
    let name = machine.get_data(args[0]).label();
    let operand = machine.operand_pop();
    machine.set_local(name.as_str(), operand);
}

/// load by name to stack top
fn load(machine: &mut Machine<Operand>, args: &[usize]) {
    let name = machine.get_data(args[0]).label();
    let operand = machine.get_local(name.as_str()).unwrap();
    machine.operand_push(operand.clone());
}

fn push(machine: &mut Machine<Operand>, args: &[usize]) {
    let imm = machine.get_data(args[0]).clone();
    machine.operand_push(imm);
}

fn add(machine: &mut Machine<Operand>, _args: &[usize]) {
    let rhs = machine.operand_pop().imm();
    let lhs = machine.operand_pop().imm();
    machine.operand_push(Operand::Imm(lhs + rhs));
}

fn sub(machine: &mut Machine<Operand>, _args: &[usize]) {
    let rhs = machine.operand_pop().imm();
    let lhs = machine.operand_pop().imm();
    machine.operand_push(Operand::Imm(lhs - rhs));
}

fn mul(machine: &mut Machine<Operand>, _args: &[usize]) {
    let rhs = machine.operand_pop().imm();
    let lhs = machine.operand_pop().imm();
    machine.operand_push(Operand::Imm(lhs * rhs));
}

fn div(machine: &mut Machine<Operand>, _args: &[usize]) {
    let rhs = machine.operand_pop().imm();
    let lhs = machine.operand_pop().imm();
    machine.operand_push(Operand::Imm(lhs / rhs));
}

fn ret(machine: &mut Machine<Operand>, _args: &[usize]) {
    machine.ret()
}

fn puts(machine: &mut Machine<Operand>, args: &[usize]) {
    let v = machine.get_data(args[0]).clone();
    println!("{:?}", v);
}

pub fn register_instructions() -> InstructionTable<Operand> {
    let mut instruction_table = InstructionTable::new();
    instruction_table.insert(Instruction::new(0, "push", 1, push));
    instruction_table.insert(Instruction::new(1, "add", 0, add));
    instruction_table.insert(Instruction::new(2, "sub", 0, sub));
    instruction_table.insert(Instruction::new(3, "mul", 0, mul));
    instruction_table.insert(Instruction::new(4, "div", 0, div));
    instruction_table.insert(Instruction::new(5, "store", 1, store));
    instruction_table.insert(Instruction::new(6, "load", 1, load));
    instruction_table.insert(Instruction::new(7, "ret", 0, ret));
    instruction_table.insert(Instruction::new(8, "puts", 1, puts));

    instruction_table
}