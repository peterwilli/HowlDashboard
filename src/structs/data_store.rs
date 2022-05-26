use std::collections::HashMap;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::mpsc::Sender;

use crate::structs::universal_number::UniversalNumber;

#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, PartialEq)]
pub struct DataStoreEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categorical_number_data: Option<HashMap<String, UniversalNumber>>
}

impl DataStoreEvent {
    pub fn contains_data(&self) -> bool {
        return self.categorical_number_data.is_some();
    }
}

pub struct DataStore {
    categorical_number_data: HashMap<String, HashMap<String, UniversalNumber>>,
    categorical_line_date_chart: HashMap<String, HashMap<String, Vec<(DateTime<Utc>, Vec<UniversalNumber>)>>>,
    event_tx: Sender<DataStoreEvent>
}

impl DataStore {
    pub fn new(event_tx: Sender<DataStoreEvent>) -> Self {
        return Self {
            event_tx,
            categorical_number_data: HashMap::new(),
            categorical_line_date_chart: HashMap::new()
        };
    }

    pub fn get_all_data(&self) -> DataStoreEvent {
        let categorical_data = self.get_categorical_number_data();
        let mut event = DataStoreEvent {
            ..Default::default()
        };
        if !categorical_data.is_empty() {
            event.categorical_number_data = Some(categorical_data);
        }
        return event;
    }

    pub fn get_categorical_number_data(&self) -> HashMap<String, UniversalNumber> {
        let mut result: HashMap<String, UniversalNumber> = HashMap::new();
        for (_, data_per_node) in self.categorical_number_data.iter() {
            for (category, number) in data_per_node.iter() {
                if result.contains_key(category) {
                    *result.get_mut(category).unwrap() += *number;
                }
                else {
                    result.insert(category.clone(), *number);
                }
            }
        }
        return result;
    }

    fn read_json_number_as_string(json_num: &Value) -> String {
        return if json_num.is_number() {
            if json_num.is_i64() {
                format!("{}", json_num.as_i64().unwrap())
            }
            else {
                format!("{}", json_num.as_f64().unwrap())
            }
        }
        else {
            json_num.as_str().unwrap().to_string()
        }
    }

    pub async fn add_entry(&mut self, from: String, json: &Value) {
        let mut changed_event = DataStoreEvent {
            ..Default::default()
        };
        if json["category"] != Value::Null {
            // Data is categorical
            if json["number"] != Value::Null {
                let key = json["category"].as_str().unwrap();
                let new_unum = UniversalNumber::from_str(&Self::read_json_number_as_string(&json["number"])).unwrap();
                let data_per_node = self.categorical_number_data.get_mut(key);
                if data_per_node.is_some() {
                    data_per_node.unwrap().insert(key.to_string(), new_unum);
                }
                else {
                    let mut new_data_per_node = HashMap::new();
                    new_data_per_node.insert(key.to_string(), new_unum);
                    self.categorical_number_data.insert(from, new_data_per_node);
                }
            }
            changed_event.categorical_number_data = Some(self.get_categorical_number_data());
        }
        if changed_event.contains_data() {
            self.event_tx.send(changed_event).await.unwrap();
        }
    }
}