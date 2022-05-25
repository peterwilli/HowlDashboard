mod utils;

use std::str::FromStr;
use std::sync::Arc;
use crate::structs::InitCommandType;
use ws_stream_wasm::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use pharos::{Observable, ObserveConfig};
use wasm_bindgen_futures::spawn_local;
use futures_util::{SinkExt, StreamExt};
use log::{debug, warn};
use js_sys::{Promise, Uint8Array};
use tokio::sync::RwLock;

#[wasm_bindgen]
#[derive(Default)]
pub struct Client {
    r#type: InitCommandType,
    stream: Arc<RwLock<Option<WsStream>>>
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
    pub fn listen_for_data(&self) -> Result<Promise, JsValue> {
        let stream = self.stream.clone();
        let promise: Promise = future_to_promise(async move {
            loop {
                let msg = stream.write().await.as_mut().unwrap().next().await;
                if msg.is_some() {
                    let msg = msg.unwrap();
                    debug!("json: {:?}", msg);
                }
                else {
                    warn!("listen_for_data loop closed");
                    return Ok(JsValue::from_str("Closed"));
                }
            }
        });
        Ok(promise)
    }

    #[wasm_bindgen]
    pub fn connect(&mut self, addr: String) {
        let stream = self.stream.clone();
        let program = async move
        {
            let (mut ws, mut wsio) = WsMeta::connect( addr, None ).await

                .expect_throw( "assume the connection succeeds" );
            *stream.write().await = Some(wsio);
        };
        spawn_local( program );
    }
}