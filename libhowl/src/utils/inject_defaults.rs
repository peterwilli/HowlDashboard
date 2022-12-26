use serde_json::Value;
use crate::data_store::DataType;

pub trait InjectDefaults {
    fn inject_defaults(&mut self, creator: &dyn DataType);
}

impl InjectDefaults for Value {
    fn inject_defaults(&mut self, creator: &dyn DataType) {
        self.as_object_mut().unwrap().insert("dataType".to_owned(), Value::from(creator.name()));
    }
}
