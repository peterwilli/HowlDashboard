use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use futures_util::{AsyncReadExt, SinkExt, StreamExt};
use js_sys::Function;
use log::{debug, warn};
use pharos::Observable;
use rand::distributions::Alphanumeric;
use rand::Rng;
use tokio::sync::{mpsc, RwLock};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use ws_stream_wasm::*;
use crate::client::base_client::BaseClient;

use crate::structs::{Command, CommandType, InitCommand, InitCommandType};

mod utils;

#[wasm_bindgen]
pub struct Client {
    base_client: Arc<RwLock<BaseClient>>,
    data_listeners: Arc<RwLock<HashMap<String, Function>>>,
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(js_name = "subscriber")]
    pub fn as_subscriber() -> Self {
        #[cfg(feature = "debugging")]
        utils::init_logging();
        utils::set_panic_hook();
        let base_client = BaseClient::new(InitCommandType::Subscriber);
        Client {
            base_client: Arc::new(RwLock::new(base_client)),
            data_listeners: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    #[wasm_bindgen(js_name = "provider")]
    pub fn as_provider() -> Self {
        #[cfg(feature = "debugging")]
        utils::init_logging();
        utils::set_panic_hook();
        let base_client = BaseClient::new(InitCommandType::Provider);
        Client {
            base_client: Arc::new(RwLock::new(base_client)),
            data_listeners: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    #[wasm_bindgen(js_name = "listenForData")]
    pub fn listen_for_data(&self, on_data: &Function) -> JsValue {
        let on_data = on_data.clone();
        let token: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();
        let token_clone = token.clone();
        let data_listeners = self.data_listeners.clone();
        spawn_local(async move {
            data_listeners.write().await.insert(token_clone, on_data);
        });
        return JsValue::from_str(&token);
    }

    #[wasm_bindgen]
    pub fn connect(&mut self, addr: String) {
        let base_client_lock = self.base_client.clone();
        let data_listeners = self.data_listeners.clone();

        spawn_local(async move {
            let (mut ws, mut wsio) = WsMeta::connect(addr, None).await
                .expect_throw("assume the connection succeeds");
            let (mut ws_write, mut ws_read) = wsio.split();
            let (tx_out, mut rx_out) = mpsc::channel::<Command>(16);
            base_client_lock.write().await.set_command_out_tx(tx_out);

            spawn_local(async move {
                loop {
                    let command = rx_out.recv().await;
                    if command.is_none() {
                        debug!("rx_out loop ended!");
                        break;
                    }
                    let command = command.unwrap();
                    let json = serde_json::to_string(&command).unwrap();
                    debug!(">> {}", json);
                    ws_write.send(WsMessage::Text(json)).await.unwrap();
                }
            });

            let base_client_lock_clone = base_client_lock.clone();
            spawn_local(async move {
                loop {
                    let msg = ws_read.next().await;
                    if msg.is_none() {
                        debug!("ws_read loop ended!");
                        break;
                    }
                    let msg = msg.unwrap();
                    match msg {
                        WsMessage::Text(ref str) => {
                            debug!("<< {}", str);
                            let command: Command = serde_json::from_str(str).unwrap();
                            base_client_lock_clone.read().await.execute_command(command).await;
                        }
                        _ => {
                            warn!("Unknown WsMessage format")
                        }
                    }
                }
            });

            let (tx_initial_data, mut rx_initial_data) = mpsc::channel(16);
            base_client_lock.write().await.set_on_initial_data(tx_initial_data);
            let data_listeners_clone = data_listeners.clone();
            spawn_local(async move {
                let this = JsValue::null();
                loop {
                    let initial_data = rx_initial_data.recv().await;
                    if initial_data.is_none() {
                        debug!("initial_data loop ended!");
                    }
                    let initial_data = initial_data.unwrap();
                    for js_callback in data_listeners_clone.read().await.values() {
                        js_callback.call1(
                            &this,
                            JsValue::from_serde(&initial_data).as_ref().unwrap(),
                        ).unwrap();
                    }
                }
            });

            let (tx_new_data, mut rx_new_data) = mpsc::channel(16);
            base_client_lock.write().await.set_on_new_data(tx_new_data);
            spawn_local(async move {
                let this = JsValue::null();
                loop {
                    let new_data = rx_new_data.recv().await;
                    if new_data.is_none() {
                        debug!("new_data loop ended!");
                    }
                    let initial_data = new_data.unwrap();
                    for js_callback in data_listeners.read().await.values() {
                        js_callback.call1(
                            &this,
                            JsValue::from_serde(&initial_data).as_ref().unwrap(),
                        ).unwrap();
                    }
                }
            });

            base_client_lock.read().await.after_connection().await;
        });
    }
}