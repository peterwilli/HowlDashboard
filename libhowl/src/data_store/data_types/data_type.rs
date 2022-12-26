use serde_json::Value;
use crate::types::StateObject;

pub trait DataType {
    fn new() -> Self where Self: Sized;
    fn name(&self) -> String;
    fn process_data(&self, data: &Value, current_object: &mut StateObject);
}