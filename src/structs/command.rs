use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use strum::Display;
use crate::structs::SocketError;

#[derive(Serialize, Deserialize, Display, Debug, PartialEq)]
pub enum CommandType {
    Init,
    Error,
    Data
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_command: Option<InitCommand>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<SocketError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>
}

impl Command {
    pub fn new_error(error: SocketError) -> Self {
        return Self {
            r#type: CommandType::Error,
            init_command: None,
            error: Some(error),
            data: None
        };
    }

    pub fn new_init(init_command: InitCommand) -> Self {
        return Self {
            r#type: CommandType::Init,
            init_command: Some(init_command),
            error: None,
            data: None
        };
    }

    pub fn new_data(init_command: InitCommand) -> Self {
        return Self {
            r#type: CommandType::Data,
            init_command: Some(init_command),
            error: None,
            data: None
        };
    }
}