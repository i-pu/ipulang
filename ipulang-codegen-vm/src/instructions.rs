use crate::operand::Operand;
use stack_vm::{Instruction, InstructionTable, Machine};

fn push(machine: &mut Machine<Operand>, args: &[usize]) {
    let arg = machine.get_data(args[0]).clone();
    machine.operand_push(arg);
}

fn add(machine: &mut Machine<Operand>, _args: &[usize]) {
    let rhs = machine.operand_pop().clone();
    let lhs = machine.operand_pop().clone();
    machine.operand_push(Operand(lhs.0 + rhs.0));
}

pub fn register_instructions() -> InstructionTable<Operand> {
    let mut instruction_table = InstructionTable::new();
    instruction_table.insert(Instruction::new(0, "push", 1, push));
    instruction_table.insert(Instruction::new(1, "add", 0, add));
    instruction_table
}
