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
    #[strum(default)]
    External(String),
}
