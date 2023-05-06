use crate::Function;

use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct LoxClass {
    name: String,
    methods: HashMap<String, Function>,
}

impl LoxClass {
    pub fn new(name: String) -> Self {
        Self { name, methods: HashMap::new() }
    }
}

impl Display for LoxClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<class {}>", self.name)
    }
}


pub struct LoxInstance {
    class: Rc<RefCell<LoxClass>>,
}