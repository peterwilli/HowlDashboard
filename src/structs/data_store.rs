use std::collections::HashMap;
use chrono::prelude::*;
use serde_json::Value;
use crate::structs::universal_number::UniversalNumber;

pub struct DataStore {
    categorical_number_data: HashMap<String, UniversalNumber>,
    categorical_line_date_chart: HashMap<String, Vec<(DateTime<Utc>, Vec<UniversalNumber>)>>
}

impl DataStore {
    pub fn new() -> Self {
        return Self {
            categorical_number_data: HashMap::new(),
            categorical_line_date_chart: HashMap::new()
        };
    }

    pub fn add_entry(&mut self, json: &Value) {
        if json["category"] != Value::Null {
            // Data is categorical
            if json["number"] != Value::Null {
                let key = json["category"].to_string();
                let mut new_unum = UniversalNumber::from_str(json["number"].as_str().unwrap()).unwrap();
                if self.categorical_number_data.contains_key(&key) {
                    let unum = self.categorical_number_data.get(&key).unwrap();
                    new_unum = new_unum + *unum;
                }
                self.categorical_number_data.insert(key, new_unum);
            }
        }
    }
}