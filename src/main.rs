use std::{fs, io::Read};

use clap::Parser;

mod ast;
mod codegen;
mod nodes;

use nodes::Node;
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
    codegen::jit_compile().unwrap();

    // let args = Args::parse();

    // dbg!("{}", &args.file);
    // dbg!("{}", &args.output);
    // let code = fs::read_to_string(&args.file).unwrap();

    // let ast = ast::program_parser(&code);

    // let bin = ast.gen_code();
    // fs::write(&args.output, bin).unwrap();
}
