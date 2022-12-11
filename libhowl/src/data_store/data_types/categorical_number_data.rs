use std::collections::HashMap;
use crate::data_store::data_types::data_block_key::DataBlockKey;
use crate::structs::UniversalNumber;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CategoricalNumberDataEntry {
    pub number: UniversalNumber,
    pub converted_values: HashMap<String, UniversalNumber>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>
}

pub struct CategoricalNumberData {
    pub numbers: HashMap<DataBlockKey, CategoricalNumberDataEntry>
}

impl CategoricalNumberData {
    pub fn new() -> Self {
        Self {
            numbers: HashMap::new()
        }
    }

    pub fn add_number(&mut self, key: DataBlockKey, new_entry: CategoricalNumberDataEntry) {
        let entry = self.numbers.entry(key).or_default();
        entry.number = new_entry.number;
        entry.suffix = new_entry.suffix;
        entry.converted_values = new_entry.converted_values;
    }
}