use crate::Function;
use crate::Object;

use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;
use std::cell::RefCell;

pub type ClassRef = Rc<RefCell<LoxClass>>;

#[derive(Debug, Clone)]
pub struct LoxClass {
    name: String,
    pub methods: HashMap<String, Function>,
    pub super_class: Option<ClassRef>,
}

impl LoxClass {
    pub fn new(name: String, methods: HashMap<String, Function>, super_class: Option<ClassRef>) -> Self {
        Self { name, methods, super_class }
    }

    pub fn get_method(&self, name: &str) -> Option<Function> {
        self.methods.get(name).cloned().or_else(|| {
            self.super_class.clone().and_then(|super_class| super_class.borrow().get_method(name))
        })
    }

    pub fn arity(&self) -> usize {
        if let Some(initializer) = self.methods.get("init") {
            initializer.arity()
        } else {
            0
        }
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
    fields: HashMap<String, Object>,
}

impl LoxInstance {
    pub fn new(class: Rc<RefCell<LoxClass>>) -> Self {
        Self { 
            class ,
            fields: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str, instance: &Object) -> Option<Object> {
        if let Some(value) = self.fields.get(name) {
            return Some(value.clone());
        }

        if let Some(method) = self.class.borrow().get_method(name) {
            return Some(
                Object::Callable(method.bind(instance.clone()))
            );
        }

        None
    }
    pub fn set(&mut self, name: &str, value: &Object) {
        self.fields.insert(name.to_string(), value.clone());
    }
}

impl Display for LoxInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<instance of {}>", self.class.borrow().name)
    }
}