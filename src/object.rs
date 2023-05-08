use std::fmt::Display;
use std::rc::Rc;
use std::cell::RefCell;

use crate::Function;
use crate::LoxClass;
use crate::LoxInstance;
type ClassRef = Rc<RefCell<LoxClass>>;
type InstanceRef = Rc<RefCell<LoxInstance>>;

#[derive(Debug, Clone)]
pub enum Object {
    Number(f64),
    String(String),
    Boolean(bool),
    Callable(Function),
    Class(ClassRef),
    Instance(InstanceRef),
    Nil,
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Number(n) => write!(f, "{}", n),
            Object::String(s) => write!(f, "{}", s),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::Nil => write!(f, "nil"),
            Object::Callable(_) => write!(f, "<callable>"),
            Object::Class(c) => write!(f, "{}", c.borrow()),
            Object::Instance(i) => write!(f, "{}", i.borrow()),
        }
    }
}

impl Object {
    pub fn equals(&self, other: &Object) -> bool {
        match (self, other) {
            (Object::Number(n1), Object::Number(n2)) => n1 == n2,
            (Object::String(s1), Object::String(s2)) => s1 == s2,
            (Object::Boolean(b1), Object::Boolean(b2)) => b1 == b2,
            (Object::Nil, Object::Nil) => true,
            _ => false,
        }
    }
}
