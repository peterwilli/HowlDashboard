use std::collections::HashMap;
use chrono::prelude::*;
use log::debug;
use serde_json::Value;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use crate::structs::universal_number::UniversalNumber;

pub struct DataStoreEvent {
    categorical_number_data: Option<HashMap<String, UniversalNumber>>
}

pub struct DataStore {
    categorical_number_data: HashMap<String, HashMap<String, UniversalNumber>>,
    categorical_line_date_chart: HashMap<String, HashMap<String, Vec<(DateTime<Utc>, Vec<UniversalNumber>)>>>,
    event_tx: Sender<DataStoreEvent>
}

impl DataStore {
    pub fn new() -> Self {
        let tx = Self::start_event_loop();
        return Self {
            event_tx: tx,
            categorical_number_data: HashMap::new(),
            categorical_line_date_chart: HashMap::new()
        };
    }

    fn start_event_loop() -> Sender<DataStoreEvent> {
        let (event_tx, mut event_rx) = mpsc::channel(16);
        tokio::spawn(async move {
            loop {
                let event = event_rx.recv().await;
                if event.is_none() {
                    debug!("Event loop stopped");
                    break;
                }
            }
        });
        return event_tx;
    }

    pub fn get_categorical_number_data(&self) -> HashMap<String, UniversalNumber> {
        let mut result: HashMap<String, UniversalNumber> = HashMap::new();
        for (category, data_per_node) in self.categorical_number_data.iter() {
            let mut total = UniversalNumber::zero();
            for number in data_per_node.values() {
                total += *number;
            }
            result.insert(category.to_string(), total);
        }
        return result;
    }

    pub fn add_entry(&mut self, from: String, json: &Value) {
        if json["category"] != Value::Null {
            // Data is categorical
            if json["number"] != Value::Null {
                let key = json["category"].to_string();
                let new_unum = UniversalNumber::from_str(json["number"].as_str().unwrap()).unwrap();
                let data_per_node = self.categorical_number_data.get_mut(&key);
                if data_per_node.is_some() {
                    data_per_node.unwrap().insert(key, new_unum);
                }
                else {
                    let mut new_data_per_node = HashMap::new();
                    new_data_per_node.insert(key, new_unum);
                    self.categorical_number_data.insert(from, new_data_per_node);
                }
            }
        }
    }
}