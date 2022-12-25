mod data_types;
mod utils;

use std::collections::HashMap;
use std::hash::Hash;

use chrono::prelude::*;
use log::debug;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::mpsc::Sender;
use crate::data_store::data_types::categorical_number_data::{CategoricalNumberData, CategoricalNumberDataEntry};
pub use crate::data_store::data_types::chart::Chart;
use crate::data_store::data_types::data_block_key::DataBlockKey;
pub use crate::data_store::data_types::data_type::DataType;
use crate::structs::UniversalNumber;
use crate::types::StateHashmap;

pub struct DataStore {
    event_tx: Sender<DataStoreEvent>,
    pub current_state: StateHashmap
}

impl DataStore {
    pub fn new(event_tx: Sender<StateHashmap>) -> Self {
        return Self {
            event_tx,
            current_state: HashMap::new()
        };
    }

    pub async fn add_entry(&mut self, from: String, json: &Value) {
        let mut changed_event = DataStoreEvent {
            ..Default::default()
        };
        let data_types: Vec<Box<dyn DataType + Send>> = vec![
            Box::new(Chart::new())
        ];
        let mut current_object: HashMap<String, Value> = HashMap::new();
        for data_type in data_types.iter() {
            data_type.process_data(json, &mut current_object);
        }
        if changed_event.contains_data() {
            self.event_tx.send(changed_event).await.unwrap();
        }
    }
}