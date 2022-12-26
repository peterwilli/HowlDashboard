use serde_json::{json, Value};

pub trait Difference {
    fn difference(&self, other: &Value) -> Value;
}

impl Difference for Value {
    fn difference(&self, other: &Value) -> Value {
        // If the values are equal, return an empty object
        if self == other {
            return json!({});
        }

        // If the values are not equal, we need to check their types
        match (self, other) {
            // If the types are different, return the entire second value
            (Value::Null, _) | (_, Value::Null) => other.clone(),
            (Value::Bool(b1), Value::Bool(b2)) if b1 != b2 => other.clone(),
            (Value::Number(n1), Value::Number(n2)) if n1 != n2 => other.clone(),
            (Value::String(s1), Value::String(s2)) if s1 != s2 => other.clone(),

            // If the values are objects, recursively compare their fields
            (Value::Object(o1), Value::Object(o2)) => {
                let mut diff = json!({});
                for (k, v) in o2 {
                    if let Some(v1) = o1.get(k) {
                        diff[k] = v1.difference(v);
                    } else {
                        diff[k] = v.clone();
                    }
                }
                diff
            },

            // If the values are arrays, recursively compare their elements
            (Value::Array(a1), Value::Array(a2)) => {
                let mut diff = json!([]);
                for (i, v) in a2.iter().enumerate() {
                    if let Some(v1) = a1.get(i) {
                        diff.as_array_mut().unwrap().push(v1.difference(v));
                    } else {
                        diff.as_array_mut().unwrap().push(v.clone());
                    }
                }
                diff
            },

            // In all other cases, return the entire second value
            (_, _) => other.clone(),
        }
    }
}
