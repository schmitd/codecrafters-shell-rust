use std::str::FromStr;
use strum::{Display, EnumString};

#[derive(Debug, PartialEq, EnumString, Display)]
#[strum(ascii_case_insensitive)]
pub enum Command {
    #[strum(to_string = "exit")]
    Exit,
    #[strum(to_string = "echo")]
    Echo,
    #[strum(to_string = "type")]
    Type,
    #[strum(to_string = "pwd")]
    Pwd,
    #[strum(disabled)]
    External(String),
}

impl FromStr for Command {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Self::from_str(s) {
            Ok(builtin) => Ok(builtin),
            Err(_) => Ok(Command::External(s.to_string())),
        }
    }
}
