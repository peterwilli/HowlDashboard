use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use futures_util::{SinkExt, StreamExt};
use log::{debug, error, warn};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, RwLock};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio_tungstenite::tungstenite::Message;
use crate::data_store::{DataStore, DataStoreEvent};

use crate::structs::{Command, CommandType, SocketError, SocketErrorType};
use crate::structs::CommandType::Init;
use crate::structs::InitCommandType::{Provider, Subscriber};

mod socket_utils;

type PeerMap = Arc<RwLock<HashMap<String, Sender<Command>>>>;

pub struct Server {
}

impl Server {
    pub fn new() -> Self {
        Self {
        }
    }

    fn start_event_listener(mut rx: Receiver<DataStoreEvent>, subscriber_peers: PeerMap) {
        tokio::spawn(async move {
           loop {
               let event = rx.recv().await.unwrap();
               let peers = subscriber_peers.read().await;
               for peer_tx in peers.values() {
                   debug!("writing to peer: {:?}", event);
                   match peer_tx.send(Command::new_event(event.clone())).await {
                       Err(e) => {
                           warn!("peer_tx send error (ignored)! Could be because a closed client still is in subscriber_peers! {}", e);
                       },
                       _ => {}
                   }
               }
           }
        });
    }

    async fn clean_closed_client(addr: &str, provider_peers: PeerMap, subscriber_peers: PeerMap) {
        if provider_peers.read().await.contains_key(addr) {
            provider_peers.write().await.remove(addr);
        }
        if subscriber_peers.read().await.contains_key(addr) {
            subscriber_peers.write().await.remove(addr);
        }
    }

    async fn handle_connection(provider_peers: PeerMap, subscriber_peers: PeerMap, data_store: Arc<RwLock<DataStore>>, raw_stream: TcpStream, addr: SocketAddr) {
        debug!("Incoming TCP connection from: {}", addr);

        let ws_stream = tokio_tungstenite::accept_async(raw_stream)
            .await
            .expect("Error during the websocket handshake occurred");
        debug!("WebSocket connection established: {}", addr);

        let (mut outgoing, mut incoming) = ws_stream.split();
        let (tx_in, mut rx_in) = mpsc::channel(16);
        let (tx_out, mut rx_out) = mpsc::channel::<Command>(16);

        tokio::spawn(async move {
            loop {
                let msg = rx_out.recv().await;
                if msg.is_none() {
                    debug!("Loop ended");
                    break;
                }
                let msg = msg.unwrap();
                let json = serde_json::to_string(&msg).unwrap();
                outgoing.send(Message::from(json)).await.unwrap();
            }
        });

        let subscriber_peers_clone = subscriber_peers.clone();
        let provider_peers_clone = provider_peers.clone();
        let tx_clone = tx_in.clone();
        tokio::spawn(async move {
           loop {
               let result = incoming.next().await;
               if result.is_none() {
                   let addr_string = addr.to_string();
                   Self::clean_closed_client(&addr_string, provider_peers_clone, subscriber_peers_clone).await;
                   break;
               }
               let result = result.unwrap();
               if result.is_err() {
                   debug!("incoming loop ended: {:?}", result.err().unwrap());
                   let addr_string = addr.to_string();
                   Self::clean_closed_client(&addr_string, provider_peers_clone, subscriber_peers_clone).await;
                   break;
               }
               let msg = result.unwrap();
               debug!("Get message: {}", msg.to_text().unwrap());
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
           }
        });

        // Check if provider or subscriber
        let command = rx_in.recv().await.unwrap();
        if command.r#type == Init {
            let init_command = command.init_command.unwrap();
            match init_command.r#type {
                Subscriber => {
                    subscriber_peers.write().await.insert(addr.to_string(), tx_out.clone());
                    Self::subscriber_loop(tx_in, rx_in, tx_out, data_store).await;
                }
                Provider => {
                    provider_peers.write().await.insert(addr.to_string(), tx_out.clone());
                    Self::provider_loop(addr.to_string(), tx_in, rx_in, data_store).await;
                }
            }
        }
    }

    async fn subscriber_loop(tx_in: Sender<Command>, mut rx_in: Receiver<Command>, tx_out: Sender<Command>, data_store_lock: Arc<RwLock<DataStore>>) {
        // Start by sending all data we have
        let data = data_store_lock.read().await.get_all_data();
        tx_out.send(Command::new_initial_data(data)).await.unwrap();

        loop {
            let command = rx_in.recv().await;
            if command.is_none() {
                debug!("Loop ended");
                break;
            }

        }
    }

    async fn provider_loop(addr: String, tx: Sender<Command>, mut rx: Receiver<Command>, data_store: Arc<RwLock<DataStore>>) {
        loop {
            let command = rx.recv().await;
            if command.is_none() {
                debug!("Loop ended");
                break;
            }
            let command = command.unwrap();
            match command.r#type {
                CommandType::Data => {
                    data_store.write().await.add_entry(addr.clone(), command.data.as_ref().unwrap()).await;
                }
                _ => {
                    error!("Unknown command: {:#?}", command);
                }
            }
        }
    }

    pub async fn start(&self, addr: &str) {
        let (event_tx, event_rx) = channel(16);
        let data_store = Arc::new(RwLock::new(DataStore::new(event_tx)));

        let provider_peers = PeerMap::new(RwLock::new(HashMap::new()));
        let subscriber_peers = PeerMap::new(RwLock::new(HashMap::new()));

        Self::start_event_listener(event_rx, subscriber_peers.clone());
        let try_socket = TcpListener::bind(&addr).await;
        let listener = try_socket.expect("Failed to bind");
        debug!("Listening on: {}", addr);
        // Let's spawn the handling of each connection in a separate task.
        while let Ok((stream, addr)) = listener.accept().await {
            tokio::spawn(Self::handle_connection(provider_peers.clone(), subscriber_peers.clone(), data_store.clone(), stream, addr));
        }
    }
}