#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        command = command.trim().to_string();
        let command: Vec<&str> = command.split_whitespace().collect();
        match command.as_slice() {
            ["exit"] => break,
            ["echo", args @ ..] => echo_command(args),
            ["type", program] => type_command(program),
            &_ => println!("{}: command not found", command[0].trim()),
        }
    }
}

fn echo_command(arg: &[&str]) -> () {
    println!("{}", arg.join(" "))
}

fn type_command(program: &str) -> () {
    if program == "exit" || program == "echo" || program == "type" {
        println!("{} is a shell builtin", program)
    } else {
        println!("{}: not found", program)
    }
}
