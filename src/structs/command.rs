use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum::Display;

use crate::structs::{DataStoreEvent, SocketError};

#[derive(Serialize, Deserialize, Display, Debug, PartialEq)]
pub enum CommandType {
    Init,
    Error,
    Data,
    InitialData,
    DataStoreEvent
}

#[derive(Serialize, Deserialize, Display, Debug, PartialEq, Copy, Clone)]
pub enum InitCommandType {
    Provider,
    Subscriber
}

impl Default for CommandType {
    fn default() -> CommandType {
        CommandType::Init
    }
}

impl Default for InitCommandType {
    fn default() -> InitCommandType {
        InitCommandType::Provider
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct InitCommand {
    pub r#type: InitCommandType
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Command {
    pub r#type: CommandType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_command: Option<InitCommand>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<SocketError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<DataStoreEvent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_data: Option<DataStoreEvent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>
}

impl Command {
    pub fn new_error(error: SocketError) -> Self {
        return Self {
            r#type: CommandType::Error,
            error: Some(error),
            ..Default::default()
        };
    }

    pub fn new_init(init_command: InitCommand) -> Self {
        return Self {
            r#type: CommandType::Init,
            init_command: Some(init_command),
            ..Default::default()
        };
    }

    pub fn new_data(data: Value) -> Self {
        return Self {
            r#type: CommandType::Data,
            data: Some(data),
            ..Default::default()
        };
    }

    pub fn new_initial_data(data: DataStoreEvent) -> Self {
        return Self {
            r#type: CommandType::InitialData,
            initial_data: Some(data),
            ..Default::default()
        };
    }

    pub fn new_event(event: DataStoreEvent) -> Self {
        return Self {
            r#type: CommandType::DataStoreEvent,
            event: Some(event),
            ..Default::default()
        };
    }
}