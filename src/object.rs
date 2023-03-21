use std::fmt::Display;

#[derive(Debug, Clone, Display)]
pub enum Object {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
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
