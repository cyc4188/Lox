/// environment
/// binding values to names

use std::{collections::HashMap, fmt::format};

use super::{Object, Error, ErrorType, Token};

pub struct Environment {
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &String, value: Object) {
        self.values.insert(name.clone(), value);
    }

    pub fn get(&self, name: &str) -> Option<&Object> {
        self.values.get(name)
    }

    pub fn assign(&mut self, name: &Token, value: &Object) -> Result<(), Error> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value.clone());
            Ok(())
        } else {
            Err(
                Error {
                    message: format!("Undefined variable '{}'.", name),
                    error_type: ErrorType::RuntimeError(name.clone()),
                }
            )
        }
    }

}