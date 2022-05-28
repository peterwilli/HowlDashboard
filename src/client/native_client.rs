use std::sync::Arc;
use futures_util::{SinkExt, StreamExt};
use log::{debug, warn};
use serde_json::Value;
use tokio::sync::mpsc::{channel, Sender};
use tokio::sync::{mpsc, RwLock};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;
use crate::client::base_client::BaseClient;

use crate::structs::{Command, CommandType, DataStoreEvent, InitCommand, InitCommandType};

pub struct Client {
    base_client: Arc<RwLock<BaseClient>>
}

impl Client {
    pub fn new(r#type: InitCommandType) -> Self {
        let base_client = BaseClient::new(r#type);
        Self {
            base_client: Arc::new(RwLock::new(base_client))
        }
    }

    pub async fn share_data(&self, data: Value) {
        self.base_client.read().await.share_data(data).await;
    }

    pub async fn set_on_new_data(&self, tx: Sender<DataStoreEvent>) {
        self.base_client.write().await.set_on_new_data(tx);
    }

    pub async fn set_on_initial_data(&self, tx: Sender<DataStoreEvent>) {
        self.base_client.write().await.set_on_new_data(tx);
    }

    pub async fn connect(&mut self, addr: Url) {
        let (ws_stream, _) = connect_async(addr).await.expect("Failed to connect");
        let (mut write, mut read) = ws_stream.split();
        let (tx_out, mut rx_out) = mpsc::channel::<Command>(16);
        self.base_client.write().await.set_command_out_tx(tx_out);

        tokio::spawn(async move {
            loop {
                let msg = rx_out.recv().await;
                if msg.is_none() {
                    debug!("Howl client command sender loop ended");
                    break;
                }
                let json = serde_json::to_string(msg.as_ref().unwrap()).unwrap();
                write.send(Message::from(json)).await.unwrap();
            }
        });

        let base_client_lock = self.base_client.clone();
        tokio::spawn(async move {
            loop {
                let data = read.next().await.unwrap().unwrap();
                if data.is_text() {
                    let command: Command = match serde_json::from_str(data.into_text().as_ref().unwrap()) {
                        Ok(command) => {
                            command
                        },
                        Err(e) => {
                            warn!("text is not a valid command JSON! Error: {}", e);
                            continue;
                        }
                    };
                    base_client_lock.read().await.execute_command(command).await;
                }
                else {
                    warn!("Data is not text!")
                }
            }
        });

        self.base_client.read().await.after_connection().await;
    }
}