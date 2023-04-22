/// environment
/// binding values to names

use std::{collections::HashMap};

use super::{Object, Error, ErrorType, Token};

pub struct Environment {
    enclosing: Option<Box<Environment>>,
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new(enclosing: Option<Box<Environment>>) -> Self {
        Self {
            enclosing: enclosing,
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &String, value: Object) {
        self.values.insert(name.clone(), value);
    }

    pub fn get(&self, name: &str) -> Option<&Object> {
        if let Some(query) = self.values.get(name) {
            Some(query)
        }
        else {
            if let Some(enclosing_inner) = self.enclosing.as_ref() {
                enclosing_inner.get(name)
            }
            else {
                None
            }
        }
    }

    pub fn assign(&mut self, name: &Token, value: &Object) -> Result<(), Error> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value.clone());
            Ok(())
        } else {
            if let Some(enclosing_inner) = self.enclosing.as_mut() {
                enclosing_inner.assign(name, value)    
            }            
            else {
                Err(
                    Error {
                        message: format!("Undefined variable '{}'.", name),
                        error_type: ErrorType::RuntimeError(name.clone()),
                    }
                )
            }
        }
    }

}