#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    io::stdout().flush().unwrap();
    loop {
        print!("$ ");
        let mut command = String::new(); 
        io::stdin().read_line(&mut command).unwrap();
        println!("{}: command not found", command.trim());
    }
}
