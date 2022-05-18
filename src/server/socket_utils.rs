use futures_channel::mpsc::UnboundedSender;
use futures_util::future;
use futures_util::future::Ready;
use serde::Serialize;
use tokio::sync::mpsc::Sender;
use tokio_tungstenite::tungstenite::{Error, Message};

use crate::structs::{Command, SocketError, SocketErrorType};
