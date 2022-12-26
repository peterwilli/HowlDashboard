use std::error::Error;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::data_store::data_types::chart::chart_object::ChartAxisXType::Number;

#[derive(Serialize, Deserialize)]
pub enum ChartAxisXType {
    DateTime,
    Number
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartObject {
    pub title: String,
    pub x_type: ChartAxisXType,
    pub x_points: Value,
    pub values: Vec<Value>
}

impl ChartObject {
    pub fn add_entries(&mut self, mut x: Value, mut y: Vec<Value>) -> Result<(), Box<dyn Error>> {
        self.x_points.as_array_mut().ok_or("x_points not an array")?.append(x.as_array_mut().ok_or("x not an array")?);
        if self.values.len() < y.len() {
            self.values.resize(y.len(), Value::Array(vec![]))
        }
        for (idx, value) in self.values.iter_mut().enumerate() {
            value.as_array_mut().ok_or("value not an array")?.append(y[idx].as_array_mut().ok_or("y[idx] not an array")?);
        }
        return Ok(());
    }
}

impl Default for ChartObject {
    fn default() -> Self {
        return Self {
            title: "No title".to_string(),
            x_type: Number,
            x_points: Value::Array(vec![]),
            values: vec![]
        }
    }
}