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
    let operand = machine
        .get_local_deep(name.as_str())
        .expect(format!("{} not found", name).as_str());
    machine.operand_push(operand.clone());
}

fn push(machine: &mut Machine<Operand>, args: &[usize]) {
    let imm = machine.get_data(args[0]).clone();
    machine.operand_push(imm);
}

fn add(machine: &mut Machine<Operand>, _args: &[usize]) {
    let rhs = machine.operand_pop().i64();
    let lhs = machine.operand_pop().i64();
    machine.operand_push(Operand::ImmI64(lhs + rhs));
}

fn sub(machine: &mut Machine<Operand>, _args: &[usize]) {
    let rhs = machine.operand_pop().i64();
    let lhs = machine.operand_pop().i64();
    machine.operand_push(Operand::ImmI64(lhs - rhs));
}

fn mul(machine: &mut Machine<Operand>, _args: &[usize]) {
    let rhs = machine.operand_pop().i64();
    let lhs = machine.operand_pop().i64();
    machine.operand_push(Operand::ImmI64(lhs * rhs));
}

fn div(machine: &mut Machine<Operand>, _args: &[usize]) {
    let rhs = machine.operand_pop().i64();
    let lhs = machine.operand_pop().i64();
    machine.operand_push(Operand::ImmI64(lhs / rhs));
}

fn modulo(machine: &mut Machine<Operand>, _args: &[usize]) {
    let rhs = machine.operand_pop().i64();
    let lhs = machine.operand_pop().i64();
    machine.operand_push(Operand::ImmI64(lhs % rhs));
}

fn lt(machine: &mut Machine<Operand>, _args: &[usize]) {
    let rhs = machine.operand_pop().i64();
    let lhs = machine.operand_pop().i64();
    machine.operand_push(Operand::ImmI64((lhs < rhs) as i64));
}

fn leq(machine: &mut Machine<Operand>, _args: &[usize]) {
    let rhs = machine.operand_pop().i64();
    let lhs = machine.operand_pop().i64();
    machine.operand_push(Operand::ImmI64((lhs <= rhs) as i64));
}

fn gt(machine: &mut Machine<Operand>, _args: &[usize]) {
    let rhs = machine.operand_pop().i64();
    let lhs = machine.operand_pop().i64();
    machine.operand_push(Operand::ImmI64((lhs > rhs) as i64));
}

fn geq(machine: &mut Machine<Operand>, _args: &[usize]) {
    let rhs = machine.operand_pop().i64();
    let lhs = machine.operand_pop().i64();
    machine.operand_push(Operand::ImmI64((lhs >= rhs) as i64));
}

fn eq(machine: &mut Machine<Operand>, _args: &[usize]) {
    let rhs = machine.operand_pop().i64();
    let lhs = machine.operand_pop().i64();
    machine.operand_push(Operand::ImmI64((lhs == rhs) as i64));
}

fn neq(machine: &mut Machine<Operand>, _args: &[usize]) {
    let rhs = machine.operand_pop().i64();
    let lhs = machine.operand_pop().i64();
    machine.operand_push(Operand::ImmI64((lhs != rhs) as i64));
}

fn or(machine: &mut Machine<Operand>, _args: &[usize]) {
    let rhs = machine.operand_pop().i64();
    let lhs = machine.operand_pop().i64();
    machine.operand_push(Operand::ImmI64((lhs != 0 || rhs != 0) as i64));
}

fn and(machine: &mut Machine<Operand>, _args: &[usize]) {
    let rhs = machine.operand_pop().i64();
    let lhs = machine.operand_pop().i64();
    machine.operand_push(Operand::ImmI64((lhs != 0 && rhs != 0) as i64));
}

fn ret(machine: &mut Machine<Operand>, _args: &[usize]) {
    machine.ret()
}

fn call(machine: &mut Machine<Operand>, args: &[usize]) {
    let label = machine.get_data(args[0]).label();
    machine.call(label.as_str());
}

fn jump(machine: &mut Machine<Operand>, args: &[usize]) {
    let label = machine.get_data(args[0]).label();
    machine.jump(label.as_str());
}

fn jump_if_zero(machine: &mut Machine<Operand>, args: &[usize]) {
    let label = machine.get_data(args[0]).label();
    let value = machine.operand_pop().i64();
    if value == 0 {
        machine.jump(label.as_str());
    }
}

fn puts(machine: &mut Machine<Operand>, _args: &[usize]) {
    match machine.operand_stack.peek() {
        Operand::ImmI64(i) => print!("{}", i),
        Operand::ImmString(s) => print!("{}", s.replace("\\n", "\n")),
        Operand::Label(l) => print!(".{}", l),
    }
}

fn inspect(machine: &mut Machine<Operand>, _args: &[usize]) {
    println!("{:?}", machine.operand_stack);
    println!("{:?}", machine.call_stack);
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
    instruction_table.insert(Instruction::new(8, "puts", 0, puts));
    instruction_table.insert(Instruction::new(9, "call", 1, call));
    instruction_table.insert(Instruction::new(10, "jump", 1, jump));
    instruction_table.insert(Instruction::new(11, "jump_if_zero", 1, jump_if_zero));
    instruction_table.insert(Instruction::new(12, "inspect", 0, inspect));

    instruction_table.insert(Instruction::new(13, "lt", 0, lt));
    instruction_table.insert(Instruction::new(14, "leq", 0, leq));
    instruction_table.insert(Instruction::new(15, "gt", 0, gt));
    instruction_table.insert(Instruction::new(16, "geq", 0, geq));
    instruction_table.insert(Instruction::new(17, "eq", 0, eq));
    instruction_table.insert(Instruction::new(18, "neq", 0, neq));
    instruction_table.insert(Instruction::new(19, "or", 0, or));
    instruction_table.insert(Instruction::new(20, "and", 0, and));
    instruction_table.insert(Instruction::new(21, "mod", 0, modulo));

    instruction_table
}
