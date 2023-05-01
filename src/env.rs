use std::cell::RefCell;
/// environment
/// binding values to names

use std::{collections::HashMap};
use std::rc::Rc;

use super::{Object, Error, ErrorType, Token};

pub type EnvironmentRef = Rc<RefCell<Environment>>;

pub struct Environment {
    pub enclosing: Option<EnvironmentRef>,
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new(enclosing: Option<EnvironmentRef>) -> Self {
        Self {
            enclosing,
            values: HashMap::new(),
        }
    }


    pub fn define(&mut self, name: &String, value: Object) {
        self.values.insert(name.clone(), value);
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        if let Some(query) = self.values.get(name) {
            Some(query.clone())
        }
        else if let Some(enclosing_inner) = self.enclosing.as_ref() {
            enclosing_inner.borrow().get(name)
        }
        else {
            None
        }
    }

    pub fn assign(&mut self, name: &Token, value: &Object) -> Result<(), Error> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value.clone());
            Ok(())
        } else if let Some(enclosing_inner) = self.enclosing.as_mut() {
            enclosing_inner.borrow_mut().assign(name, value)    
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

    fn ancestor(&self, distance: usize) -> EnvironmentRef {
        let mut parent = self.enclosing.clone().expect("No enclosing environment found.");
        let mut environment = Rc::clone(&parent);
        for _ in 1..distance {
            parent = environment
            .borrow()
            .enclosing
            .clone()
            .expect("No enclosing environment found.");
            environment = Rc::clone(&parent);
        }
        environment
    }

    pub fn get_at(&self, distance: usize, name: &String) -> Option<Object> {
        if distance > 0 {
            self.ancestor(distance).borrow().get(name)
        }
        else {
            self.get(name)
        }
    }

    pub fn assign_at(&mut self, distance: usize, name: &Token, value: &Object) -> Result<(), Error> {
        if distance > 0 {
            self.ancestor(distance).borrow_mut().assign(name, value)
        }
        else {
            self.assign(name, value)
        }
    }
}