use crate::Function;

use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;
use std::cell::RefCell;

type ClassRef = Rc<RefCell<LoxClass>>;

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


#[derive(Debug, Clone)]
pub struct LoxInstance {
    class: ClassRef,
}

impl LoxInstance {
    pub fn new(class: Rc<RefCell<LoxClass>>) -> Self {
        Self { class }
    }
}

impl Display for LoxInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<instance of {}>", self.class.borrow().name)
    }
}