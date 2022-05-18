use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use strum::Display;
use crate::structs::CommandType::{Error, Init};
use crate::structs::SocketError;

#[derive(Serialize, Deserialize, Display, Debug, PartialEq)]
pub enum CommandType {
    Init,
    Error
}

#[derive(Serialize, Deserialize, Display, Debug, PartialEq, Copy, Clone)]
pub enum InitCommandType {
    Provider,
    Subscriber
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct InitCommand {
    pub r#type: InitCommandType
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Command {
    pub r#type: CommandType,
    pub init_command: Option<InitCommand>,
    pub error: Option<SocketError>
}

impl Command {
    pub fn new_error(error: SocketError) -> Self {
        return Self {
            r#type: Error,
            init_command: None,
            error: Some(error)
        };
    }

    pub fn new_init(init_command: InitCommand) -> Self {
        return Self {
            r#type: Init,
            init_command: Some(init_command),
            error: None
        };
    }
}