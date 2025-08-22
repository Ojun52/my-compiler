use clap::{Arg, command};

fn main() {
    let matches = command!().arg(Arg::new("code")).get_matches();

    let arg = matches.get_one::<String>("code");

    if let Some(integer) = arg {
        println!(".intel_syntax noprefix");
        println!(".globl main");
        println!("main:");
        println!("  mov rax, {}", integer);
        println!("  ret");
    } else {
        println!("The number of argument is wrong.")
    }
}
