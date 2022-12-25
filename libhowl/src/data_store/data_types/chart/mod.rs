use std::collections::HashMap;
use std::str::FromStr;

use serde_json::Value;

use crate::data_store::data_types::chart::chart_object::{ChartAxisXType, ChartObject};
use crate::data_store::data_types::data_type::DataType;
use crate::data_store::utils::filter_title_objects;
use crate::utils::{Slug, TimestampExt};

mod chart_object;

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
        filter_title_objects(data, &mut |title, value| {
            let mut chart: ChartObject = serde_json::from_value(current_object.entry(title.to_slug()).or_insert(serde_json::to_value(ChartObject {
                title: title.to_string(),
                ..Default::default()
            }).unwrap()).to_owned()).unwrap();
            if value.is_array() {
                let arr = value.as_array().unwrap();
                if arr.len() > 0 {
                    let first_obj = arr.first().unwrap();
                    if first_obj.is_number() {
                        chart.add_entries((0..arr.len()).map(|idx| {
                            return Value::from(idx)
                        }).collect(), arr.to_vec()).unwrap();
                        let chart_value = serde_json::to_value(chart).unwrap();
                        current_object.insert(title.to_slug(), chart_value);
                    }
                    else if first_obj.is_object() {
                        let obj = first_obj.as_object().unwrap();
                        // Key-value pair (2)
                        if obj.keys().len() == 2 {
                            let allowed_keys = ["timestamp"];
                            if let Some(x_key) = obj.keys().find(|x| allowed_keys.contains(&x.as_str())) {
                                chart.x_type = ChartAxisXType::DateTime;
                                chart.add_entries(arr.iter().filter_map(|obj| {
                                    if x_key == "timestamp" {
                                        return Some(Value::from(obj.get(x_key).unwrap().as_u64().unwrap()))
                                    }
                                    return None;
                                }).collect(), vec![obj.keys().filter(|key| x_key != *key).filter_map(|key| {
                                    let obj = obj.get(key).unwrap();
                                    if obj.is_number() {
                                        return Some(obj.clone());
                                    }
                                    return None;
                                }).collect()]).unwrap();
                                let chart_value = serde_json::to_value(chart).unwrap();
                                current_object.insert(title.to_slug(), chart_value);
                            }
                        }
                    }
                }
            }
        })

    }
}