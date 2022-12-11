use std::collections::HashMap;
use serde_json::Value;

pub trait DataType {
    fn new() -> Self where Self: Sized;
    fn name(&self) -> String;
    fn process_data(&self, data: &Value, current_object: &mut HashMap<String, Value>);
}