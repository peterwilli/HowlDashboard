mod data_types;

use std::collections::HashMap;
use std::hash::Hash;

use chrono::prelude::*;
use log::debug;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::mpsc::Sender;
use crate::data_store::data_types::categorical_number_data::{CategoricalNumberData, CategoricalNumberDataEntry};
use crate::data_store::data_types::chart::Chart;
use crate::data_store::data_types::data_block_key::DataBlockKey;
use crate::data_store::data_types::data_type::DataType;
use crate::structs::UniversalNumber;

#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, PartialEq)]
pub struct DataStoreEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categorical_number_data: Option<HashMap<String, HashMap<String, CategoricalNumberDataEntry>>>
}

impl DataStoreEvent {
    pub fn contains_data(&self) -> bool {
        return self.categorical_number_data.is_some();
    }
}

pub struct DataStore {
    categorical_number_data: CategoricalNumberData,
    categorical_line_date_chart: HashMap<String, HashMap<String, Vec<(DateTime<Utc>, Vec<UniversalNumber>)>>>,
    event_tx: Sender<DataStoreEvent>
}

impl DataStore {
    pub fn new(event_tx: Sender<DataStoreEvent>) -> Self {
        return Self {
            event_tx,
            categorical_number_data: CategoricalNumberData::new(),
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

    pub fn get_categorical_number_data(&self) -> HashMap<String, HashMap<String, CategoricalNumberDataEntry>> {
        // Merge all by "from" and "title"
        let mut result: HashMap<String, HashMap<String, CategoricalNumberDataEntry>> = HashMap::new();
        for (key, self_entry) in self.categorical_number_data.numbers.iter() {
            let entries = result.entry(key.title.clone()).or_default();
            let entry = entries.entry(key.category.clone()).or_default();
            entry.number += self_entry.number;
            entry.suffix = self_entry.suffix.clone();

            for suffix in self_entry.converted_values.keys() {
                if entry.converted_values.contains_key(suffix) {
                    let new_value = *self_entry.converted_values.get(suffix).unwrap() + *entry.converted_values.get(suffix).unwrap();
                    entry.converted_values.insert(suffix.to_string(), new_value);
                }
                else {
                    entry.converted_values.insert(suffix.to_string(), self_entry.converted_values.get(suffix).unwrap().clone());
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

    /*
    {title: "Portfolio (across bots)", data: {"Bitcoin": [{ value: "120", suffix: "btc" }, { value: "12000", suffix: "$" }]}}
    let data = {
		categorical_number_data: {
			"test": [
				{ number: "120", suffix: "BTC" },
				{ number: "12990", suffix: "USD" },
			],
			"test2": [
				{ number: "110", suffix: "ETH" },
				{ number: "12990", suffix: "USD" },
			],
		},
	};
     */

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