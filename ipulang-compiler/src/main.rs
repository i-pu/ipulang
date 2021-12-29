use std::fs;

use anyhow::{Error, Result};
use clap::Parser;
mod ast;
mod codegen;
mod nodes;
mod type_check;
mod types;

use ast::program_parser;
use codegen::code_gen;
use type_check::type_check;

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

pub fn compile(code: String) -> Result<String, Box<Error>> {
    let ast = program_parser(&code);
    let ast = type_check(ast)?;
    let ir = code_gen(ast)?;
    Ok(ir)
}

fn main() {
    let args = Args::parse();

    let code = fs::read_to_string(&args.file).unwrap();
    let ir = compile(code).unwrap();

    // let bin = ast.gen_code();
    fs::write(&args.output, ir).unwrap();
}
