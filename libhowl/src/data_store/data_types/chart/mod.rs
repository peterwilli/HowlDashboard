use std::collections::HashMap;
use serde_json::Value;
use crate::data_store::data_types::data_type::DataType;

pub struct Chart {

}

impl DataType for Chart {
    fn new() -> Self where Self: Sized {
        return Self {

        }
    }

    fn name(&self) -> String {
        return "Chart".to_string();
    }

    fn process_data(&self, data: &Value, current_object: &mut HashMap<String, Value>) {

    }
}