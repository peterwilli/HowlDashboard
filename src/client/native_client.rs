use std::collections::HashMap;
use futures_util::{SinkExt, StreamExt};

use log::{debug, warn};
use serde_json::Value;
use tokio::sync::mpsc::{channel, Sender};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tokio_tungstenite::tungstenite::connect;

use url::Url;

use crate::structs::{Command, CommandType, DataStoreEvent, InitCommand, InitCommandType};

pub struct Client {
    command_tx: Option<Sender<Command>>,
    r#type: InitCommandType,
    on_data_tx: Option<Sender<DataStoreEvent>>
}

impl Client {
    pub fn as_provider() -> Self {
        Self {
            command_tx: None,
            r#type: InitCommandType::Provider,
            on_data_tx: None
        }
    }

    pub fn as_subscriber(on_data_tx: Sender<DataStoreEvent>) -> Self {
        Self {
            command_tx: None,
            r#type: InitCommandType::Subscriber,
            on_data_tx: Some(on_data_tx)
        }
    }

    pub async fn share_data(&self, data: Value) {
        self.command_tx.as_ref().unwrap().send(Command::new_data(data)).await.unwrap();
    }

    pub async fn connect(&mut self, addr: Url) {
        let (tx, mut rx) = channel(2);
        let (ws_stream, _) = connect_async(addr).await.expect("Failed to connect");
        let (mut write, mut read) = ws_stream.split();

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

        if self.r#type == InitCommandType::Subscriber {
            let on_data_tx = self.on_data_tx.as_ref().unwrap().clone();
            tokio::spawn(async move {
                loop {
                    let data = read.next().await.unwrap().unwrap();
                    if data.is_text() {
                        let command: Command = serde_json::from_str(data.into_text().as_ref().unwrap()).unwrap();
                        match command.r#type {
                            CommandType::DataStoreEvent => {
                                on_data_tx.send(command.event.unwrap()).await.unwrap();
                            }
                            _ => {
                                warn!("Unknown command type: {}", command.r#type);
                            }
                        }
                    }
                    else {
                        warn!("Data is not text!")
                    }
                }
            });
        }

        // To connect ourselves
        self.command_tx.as_ref().unwrap().send(Command::new_init(InitCommand {
            r#type: self.r#type
        })).await.unwrap();
    }
}