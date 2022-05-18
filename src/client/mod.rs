use std::collections::HashMap;
use futures_util::{SinkExt, StreamExt};

use log::debug;
use tokio::sync::mpsc::{channel, Sender};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

use crate::structs::{Command, CommandType, InitCommand, InitCommandType};

pub struct Client {
    command_tx: Option<Sender<Command>>,
    r#type: InitCommandType
}

impl Client {
    pub fn new(r#type: InitCommandType) -> Self {
        Self {
            command_tx: None,
            r#type
        }
    }

    pub async fn connect(&mut self, addr: Url) {
        let (tx, mut rx) = channel(2);
        let (ws_stream, _) = connect_async(addr).await.expect("Failed to connect");
        let (mut write, read) = ws_stream.split();

        self.command_tx = Some(tx);

        tokio::spawn(async move {
            loop {
                let msg = rx.recv().await;
                if msg.is_none() {
                    debug!("Howl client command sender loop ended");
                    break;
                }
                let json = serde_json::to_string(msg.as_ref().unwrap()).unwrap();
                write.send(Message::from(json)).await.unwrap();
            }
        });

        // To connect ourselves
        self.command_tx.as_ref().unwrap().send(Command::new_init(InitCommand {
            r#type: self.r#type
        })).await.unwrap();
    }
}