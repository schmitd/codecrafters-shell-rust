use std::str::FromStr;
use strum::{Display, EnumString};

#[derive(Debug, PartialEq, EnumString, Display)]
#[strum(ascii_case_insensitive, serialize_all = "lowercase")]
pub enum BuiltIn {
    Exit,
    Echo,
    Type,
    Pwd,
    Cd,
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
