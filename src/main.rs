pub mod lang;
use clap::{Arg, command};
use lang::parser;

fn main() {
    let matches = command!().arg(Arg::new("code")).get_matches();

    let arg = matches.get_one::<String>("code");

    if let Some(code) = arg {
        let (_, programs) = parser::program_parser(&code[..]).unwrap();
        println!(".intel_syntax noprefix");
        println!(".globl main");
        println!("main:");

        println!("  push rbp");
        println!("  mov rbp, rsp");
        println!("  sub rsp, 208");
        programs.iter().for_each(|x| {
            x.generate();
            println!("  pop rax");
        });
        println!("  mov rsp, rbp");
        println!("  pop rbp");
        println!("  ret");
    } else {
        println!("The number of argument is wrong.")
    }
}
