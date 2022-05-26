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

use crate::structs::{Command, CommandType, InitCommand, InitCommandType};

mod utils;

#[wasm_bindgen]
#[derive(Default)]
pub struct Client {
    r#type: InitCommandType,
    data_listeners: Arc<RwLock<HashMap<String, Function>>>
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(js_name="subscriber")]
    pub fn as_subscriber() -> Self {
        #[cfg(feature = "debugging")]
        utils::init_logging();
        utils::set_panic_hook();
        Client {
            r#type: InitCommandType::Subscriber,
            ..Default::default()
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
        let r#type = self.r#type.clone();
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
                            let this = JsValue::null();
                            if command.r#type == CommandType::InitialData {
                                for js_callback in data_listeners.read().await.values() {
                                    js_callback.call1(
                                        &this,
                                        JsValue::from_serde(command.initial_data.as_ref().unwrap()).as_ref().unwrap()
                                    ).unwrap();
                                }
                            }
                            else if command.r#type == CommandType::DataStoreEvent {
                                for js_callback in data_listeners.read().await.values() {
                                    js_callback.call1(
                                        &this,
                                        JsValue::from_serde(command.event.as_ref().unwrap()).as_ref().unwrap()
                                    ).unwrap();
                                }
                            }
                        },
                        _ => {
                            warn!("Unknown WsMessage format")
                        }
                    }
                }
            });

            tx_out.send(Command::new_init(InitCommand {
                r#type
            })).await.unwrap();
        });
    }
}