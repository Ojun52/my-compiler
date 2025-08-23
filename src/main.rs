pub mod lang;
use clap::{Arg, command};
use lang::parser;

fn main() {
    let matches = command!().arg(Arg::new("code")).get_matches();

    let arg = matches.get_one::<String>("code");

    if let Some(code) = arg {
        let (_, expr) = parser::const_int_parser(&code[..]).unwrap();
        println!(".intel_syntax noprefix");
        println!(".globl main");
        println!("main:");
        expr.generate();
        println!("  pop rax");
        println!("  ret");
    } else {
        println!("The number of argument is wrong.")
    }
}
