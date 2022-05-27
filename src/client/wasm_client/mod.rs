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
    base_client: Arc<RwLock<BaseClient>>
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(js_name="subscriber")]
    pub fn as_subscriber() -> Self {
        #[cfg(feature = "debugging")]
        utils::init_logging();
        utils::set_panic_hook();
        let base_client = BaseClient::new(InitCommandType::Subscriber);
        Client {
            base_client: Arc::new(RwLock::new(base_client))
        }
    }

    #[wasm_bindgen(js_name="provider")]
    pub fn as_provider() -> Self {
        Client {
            r#type: InitCommandType::Provider,
            ..Default::default()
        }
    }

    #[wasm_bindgen(js_name="listenForData")]
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
            let (tx_in, mut rx_in) = mpsc::channel::<Command>(16);

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

            let (tx_command_in, rx_command_in) = mpsc::channel(16);
            spawn_local(async move {
                base_client_lock.write().await.command_runner_loop(rx_command_in);
            });

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
                            tx_command_in.send(command).await.unwrap();
                        },
                        _ => {
                            warn!("Unknown WsMessage format")
                        }
                    }
                }
            });
        });
    }
}