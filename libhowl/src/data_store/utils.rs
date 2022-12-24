use serde_json::Value;

pub fn filter_title_objects(value: &Value, f: &mut dyn FnMut(&str, &Value)) {
    if value.is_object() {
        let obj = value.as_object().unwrap();
        if obj.contains_key("title") && obj.contains_key("data") {
            f(obj.get("title").unwrap().as_str().unwrap(), obj.get("data").unwrap());
        }
    }
}

pub fn filter_recursively(value: &Value, f: &dyn Fn(&Value), filter: &dyn Fn(&Value) -> bool) {
    match value {
        Value::Object(map) => {
            for (_, value) in map {
                if filter(value) {
                    f(value);
                }
                filter_recursively(value, f, filter);
            }
        }
        Value::Array(array) => {
            for value in array {
                filter_recursively(value, f, filter);
            }
        }
        _ => {
            if filter(value) {
                f(value);
            }
        }
    }
}
