use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum SocketErrorType {
    ParseError,
    ParametersNotFound,
    UnknownInitType,
    AlreadyInited,
    WrongInitType
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SocketError {
    pub error_type: SocketErrorType,
    pub message: String
}