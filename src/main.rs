#[allow(unused_imports)]
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

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
    } else if is_executable(program) {
        println!(
            "{} is {}",
            program,
            std::env::var("PATH").unwrap_or_default()
        )
    } else {
        println!("{}: not found", program)
    }
}

fn is_executable(program: &str) -> bool {
    for path in std::env::var("PATH").unwrap_or_default().split(':') {
        let file = Path::join(Path::new(path), program);
        if !file.exists() {
            return false;
        }
        let permissions = std::fs::metadata(&file).unwrap().permissions().mode();
        if permissions & 0o111 != 0 {
            return true;
        }
    }
    return false;
}
