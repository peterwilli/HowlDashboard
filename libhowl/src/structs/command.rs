use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum::{Display, EnumString};

use crate::structs::SocketError;
use crate::types::StateHashmap;

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

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct InitCommand {
    pub r#type: InitCommandType
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Command {
    Init(InitCommand),
    Error(SocketError),
    Event(StateHashmap),
    InitialState(StateHashmap),
    Data(Value),
    CloseConnection
}