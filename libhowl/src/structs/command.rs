use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum::{Display, EnumString};

use crate::structs::SocketError;

#[derive(Serialize, Deserialize, Display, Debug, PartialEq, Copy, Clone, EnumString)]
pub enum InitCommandType {
    Provider,
    Subscriber
}

impl Default for InitCommandType {
    fn default() -> InitCommandType {
        InitCommandType::Provider
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub struct InitCommand {
    pub r#type: InitCommandType
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Command {
    Init(InitCommand),
    Error(SocketError),
    Event(Value),
    InitialState(Value),
    Data(Value),
    CloseConnection
}