use anyhow::Result;
use clap::Parser;
use ipulang_codegen_vm::{code_gen, execute};
use std::fs;
// use ipulang_codegen_llvm::codegen::code_gen;
use ipulang_parser::{ast::program_parser, nodes::Span};
use ipulang_typecheck::type_check::type_check;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// Name of the person to greet
    file: String,

    #[clap(short, long)]
    output: String,
    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

pub fn compile(code: String) -> Result<Vec<u8>> {
    let code = Span::new(code.as_str());
    let ast = program_parser(code);
    let ast = type_check(ast)?;
    dbg!(&ast);
    let ir = code_gen(ast)?;
    Ok(ir)
}

fn main() {
    let args = Args::parse();
    let code = fs::read_to_string(&args.file).unwrap();
    let ir = compile(code).unwrap();
    fs::write(&args.output, ir.clone()).unwrap();

    execute(&mut ir.as_slice());
}
