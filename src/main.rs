extern crate shell_words;
mod command;
mod parse;

use command::{BuiltIn, Command};
// use shell_words::split;
use std::env;
use std::io::{self, Error, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::result;

fn main() -> result::Result<(), std::io::Error> {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut toks = String::new();
        io::stdin().read_line(&mut toks).unwrap();
        let toks = split(&toks).unwrap();
        for tok in toks {
            // each redirect creates a file and reassigns standard out
            // currently run command
            // OK so I guess its time to make a pratt parser
            // lets do a custom lexer too lol, to handle redirects that touch words
        }
        // let toks: Vec<&str> = toks.iter().map(|s| s.as_str()).collect();
        // let [command, rest @ ..]: &[&str] = toks.as_slice() else {
        //     continue;
        // };

        // move to body
        // process_command(command, rest)?;
    }
}

fn process_command(command: &str, rest: &[&str]) -> Result<(), std::io::Error> {
    match Command::from(command) {
        Command::Internal(BuiltIn::Exit) => std::process::exit(0),
        Command::Internal(BuiltIn::Echo) => echo_command(rest),
        Command::Internal(BuiltIn::Type) => type_command(rest[0]),
        Command::Internal(BuiltIn::Pwd) => pwd_command(),
        Command::Internal(BuiltIn::Cd) => cd_command(rest[0])
            .unwrap_or_else(|_| println!("cd: {}: No such file or directory", rest[0])),
        Command::External(_) => run_command(command, rest)?,
    }
    Ok(())
}

fn run_command(program: &str, args: &[&str], redirects: &[Redirect]) -> Result<(), std::io::Error> {
    if let Some(file) = which_executable(program) {
        let _status = std::process::Command::new(file.file_name().unwrap())
            .args(args)
            .status()?;
    } else {
        println!("{}: command not found", program);
        return Ok(());
    }
    Ok(())
}

fn pwd_command() -> () {
    println!("{}", std::env::current_dir().unwrap().display())
}

fn cd_command(dir: &str) -> Result<(), std::io::Error> {
    if let Some(base) = dir.strip_prefix("~")
        && let Some(home) = env::home_dir()
    {
        std::env::set_current_dir(home.join(base))?;
    } else {
        env::set_current_dir(dir)?;
    }
    Ok(())
}

fn echo_command(arg: &[&str]) -> () {
    println!("{}", arg.join(" "))
}

fn type_command(program: &str) -> () {
    if let Command::Internal(builtin) = program.into() {
        println!("{} is a shell builtin", builtin)
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
