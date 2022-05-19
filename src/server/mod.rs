mod socket_utils;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc};
use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::StreamExt;
use log::{debug, error};
use serde_json::Value;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, RwLock};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_tungstenite::tungstenite::Message;
use crate::structs::{Command, CommandType, SocketError, SocketErrorType};
use crate::structs::CommandType::Init;
use crate::structs::InitCommandType::{Provider, Subscriber};

type PeerMap = Arc<RwLock<HashMap<String, Sender<Command>>>>;

pub struct Server {

}

impl Server {
    pub fn new() -> Self {
        Self {

        }
    }

    async fn handle_connection(provider_peers: PeerMap, subscriber_peers: PeerMap, raw_stream: TcpStream, addr: SocketAddr) {
        debug!("Incoming TCP connection from: {}", addr);

        let ws_stream = tokio_tungstenite::accept_async(raw_stream)
            .await
            .expect("Error during the websocket handshake occurred");
        debug!("WebSocket connection established: {}", addr);

        let (outgoing, mut incoming) = ws_stream.split();
        let (tx, mut rx) = mpsc::channel(16);

        let tx_clone = tx.clone();
        tokio::spawn(async move {
            let result = incoming.next().await;
            // let command = result.unwrap().unwrap();
            let msg = result.unwrap().unwrap();
            let command: Command = match serde_json::from_str(msg.to_text().unwrap()) {
                Ok(msg) => msg,
                Err(e) => {
                    debug!("Cant parse '{}' to JSON!", msg.to_text().unwrap());
                    let error = Command::new_error(SocketError {
                        error_type: SocketErrorType::ParseError,
                        message: e.to_string(),
                    });
                    error
                }
            };
            tx_clone.send(command).await.unwrap();
        });

        // Check if provider or subscriber
        let command = rx.recv().await.unwrap();
        if command.r#type == Init {
            let init_command = command.init_command.unwrap();
            match init_command.r#type {
                Subscriber => {
                    subscriber_peers.write().await.insert(addr.to_string(), tx.clone());
                    Self::provider_loop(tx, rx);
                }
                Provider => {
                    provider_peers.write().await.insert(addr.to_string(), tx.clone());
                    Self::subscriber_loop(tx, rx);
                }
            }
        }
    }

    fn subscriber_loop(tx: Sender<Command>, mut rx: Receiver<Command>) {
        tokio::spawn(async move {
            loop {
                let command = rx.recv().await;
                if command.is_none() {
                    debug!("Loop ended");
                    break;
                }
                let command = command.unwrap();
            }
        });
    }

    fn provider_loop(tx: Sender<Command>, mut rx: Receiver<Command>) {
        tokio::spawn(async move {
            loop {
                let command = rx.recv().await;
                if command.is_none() {
                    debug!("Loop ended");
                    break;
                }
                let command = command.unwrap();
                match command.r#type {
                    CommandType::Data => {
                        let json: Value = serde_json::from_str(command.data.as_ref().unwrap()).unwrap();
                        
                    }
                    _ => {
                        error!("Unknown command: {:#?}", command);
                    }
                }
            }
        });
    }

    pub async fn start(&self, addr: &str) {
        let provider_peers = PeerMap::new(RwLock::new(HashMap::new()));
        let subscriber_peers = PeerMap::new(RwLock::new(HashMap::new()));
        let try_socket = TcpListener::bind(&addr).await;
        let listener = try_socket.expect("Failed to bind");
        debug!("Listening on: {}", addr);
        // Let's spawn the handling of each connection in a separate task.
        while let Ok((stream, addr)) = listener.accept().await {
            tokio::spawn(Self::handle_connection(provider_peers.clone(), subscriber_peers.clone(), stream, addr));
        }
    }
}