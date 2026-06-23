use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::result;

fn main() -> result::Result<(), std::io::Error> {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        command = command.trim().to_string();
        let command: Vec<&str> = command.split_whitespace().collect();
        match command.as_slice() {
            ["exit"] => break Ok(()),
            ["echo", args @ ..] => echo_command(args),
            ["type", program, _rest @ ..] => type_command(program),
            [program, args @ ..] => run_command(program, args)?,
            &_ => println!("{}: command not found", command[0].trim()),
        }
    }
}

fn run_command(program: &str, args: &[&str]) -> Result<(), std::io::Error> {
    if let Some(file) = which_executable(program) {
        let _status =
            std::process::Command::new(file.components().collect::<Vec<_>>().as_slice()[0])
                .args(args)
                .status()?;
    }
    Ok(())
}

fn echo_command(arg: &[&str]) -> () {
    println!("{}", arg.join(" "))
}

fn type_command(program: &str) -> () {
    if program == "exit" || program == "echo" || program == "type" {
        println!("{} is a shell builtin", program)
    } else if let Some(file) = which_executable(program) {
        println!("{} is {}", program, file.display());
    } else {
        println!("{}: not found", program)
    }
}

fn which_executable(program: &str) -> Option<PathBuf> {
    if let Some(path) = std::env::var_os("PATH") {
        for dir in std::env::split_paths(&path) {
            let file = Path::join(Path::new(&dir), program);
            // if !file.exists() {
            //     println!("debug:{:#?} does not exist", file);
            // }
            if let Some(meta) = std::fs::metadata(&file).ok() {
                let permissions = meta.permissions().mode();
                if permissions & 0o111 != 0 {
                    return Some(file);
                }
            }
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;
}
