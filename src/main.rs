#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command = String::new(); 
        io::stdin().read_line(&mut command).unwrap();
        command = command.trim().to_string();
        match &command[..] {
            "exit" => break,
            "echo" => echo(&command),
            &_ => println!("{}: command not found", command.trim()),
        }
    }
}

fn echo(arg: &String) -> () {
    println!("{}", arg)
}
