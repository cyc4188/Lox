/// environment
/// binding values to names

use std::collections::HashMap;
use super::Object;

pub struct Environment {
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&Object> {
        self.values.get(name)
    }
}