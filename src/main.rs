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
        match command[0] {
            "exit" => break,
            "echo" => echo(command[1]),
            &_ => println!("{}: command not found", command[0].trim()),
        }
    }
}

fn echo(arg: &str) -> () {
    println!("{}", arg)
}
