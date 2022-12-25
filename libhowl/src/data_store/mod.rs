use std::collections::HashMap;
use std::hash::Hash;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::mpsc::Sender;

pub use crate::data_store::data_types::chart::Chart;
pub use crate::data_store::data_types::data_type::DataType;
use crate::structs::Command;

mod data_types;
mod utils;

pub struct DataStore {
    event_tx: Sender<Command>,
    pub current_state: Value
}

impl DataStore {
    pub fn new(event_tx: Sender<Command>) -> Self {
        return Self {
            event_tx,
            current_state: Value::Array(vec![])
        };
    }

    pub async fn add_entry(&mut self, from: String, json: &Value) {
        let data_types: Vec<Box<dyn DataType + Send>> = vec![
            Box::new(Chart::new())
        ];
        let mut current_object: HashMap<String, Value> = HashMap::new();
        for data_type in data_types.iter() {
            data_type.process_data(json, &mut current_object);
        }
        let data_event = Command::Data(Value::from(1));
        self.event_tx.send(data_event).await.unwrap();
    }
}