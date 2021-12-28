use std::fs;

use clap::Parser;

mod ast;
mod codegen;
mod nodes;
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

fn main() {
    let args = Args::parse();

    let code = fs::read_to_string(&args.file).unwrap();
    let ir = codegen::compile(code).unwrap();

    // let bin = ast.gen_code();
    fs::write(&args.output, ir).unwrap();
}
