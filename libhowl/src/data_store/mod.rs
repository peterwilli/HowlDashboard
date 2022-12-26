use std::hash::Hash;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::sync::mpsc::Sender;

pub use crate::data_store::data_types::chart::Chart;
pub use crate::data_store::data_types::data_type::DataType;
use crate::structs::Command;
use crate::utils::json_diff::Difference;

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
            current_state: json!({})
        };
    }

    pub async fn add_entry(&mut self, from: String, json: &Value) {
        let data_types: Vec<Box<dyn DataType + Send>> = vec![
            Box::new(Chart::new())
        ];
        let old_object = self.current_state.clone();
        for data_type in data_types.iter() {
            data_type.process_data(json, self.current_state.as_object_mut().unwrap());
        }
        let diff = old_object.difference(&self.current_state);
        let data_event = Command::Data(diff);
        self.event_tx.send(data_event).await.unwrap();
    }
}