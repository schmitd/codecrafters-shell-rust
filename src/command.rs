use std::str::FromStr;
use strum::{Display, EnumString};

#[derive(Debug, PartialEq, EnumString, Display)]
#[strum(ascii_case_insensitive)]
pub enum BuiltIn {
    #[strum(to_string = "exit")]
    Exit,
    #[strum(to_string = "echo")]
    Echo,
    #[strum(to_string = "type")]
    Type,
    #[strum(to_string = "pwd")]
    Pwd,
}

pub enum Command {
    Internal(BuiltIn),
    External(String),
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        if let Ok(builtin) = BuiltIn::from_str(s) {
            Command::Internal(builtin)
        } else {
            Command::External(s.to_string())
        }
    }
}
