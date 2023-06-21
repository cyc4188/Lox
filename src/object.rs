use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

use crate::Error;
use crate::ErrorType;
use crate::Function;
use crate::LoxClass;
use crate::LoxInstance;
use crate::Token;
use crate::TokenType;
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

pub enum NumberType {
    Integer(i64),
    Float(f64),
}

impl NumberType {
    pub fn as_float(&self) -> f64 {
        match self {
            NumberType::Integer(i) => *i as f64,
            NumberType::Float(f) => *f,
        }
    }
    pub fn binary_op(&self, op: Token, other: &NumberType) -> Result<Self, Error> {
        match op.token_type {
            _ => Err(Error {
                message: format!("Unsupported binary operation",),
                error_type: ErrorType::RuntimeError(op.clone()),
            }),
        }
    }
    pub fn add(&self, other: &NumberType) -> Result<Self, Error> {
        use NumberType::{Float, Integer};
        Ok(match self {
            Integer(i) => match other {
                Integer(j) => Integer(i + j),
                Float(f) => Float(*i as f64 + f),
            },
            Float(f) => match other {
                Integer(i) => Float(f + *i as f64),
                Float(g) => Float(f + g),
            },
        })
    }
    pub fn sub(&self, other: &NumberType) -> Result<Self, Error> {
        use NumberType::{Float, Integer};
        Ok(match self {
            Integer(i) => match other {
                Integer(j) => Integer(i - j),
                Float(f) => Float(*i as f64 - f),
            },
            Float(f) => match other {
                Integer(i) => Float(f - *i as f64),
                Float(g) => Float(f - g),
            },
        })
    }

    pub fn mul(&self, other: &NumberType) -> Result<Self, Error> {
        use NumberType::{Float, Integer};
        Ok(match self {
            Integer(i) => match other {
                Integer(j) => Integer(i * j),
                Float(f) => Float(*i as f64 * f),
            },
            Float(f) => match other {
                Integer(i) => Float(f * *i as f64),
                Float(g) => Float(f * g),
            },
        })
    }
    pub fn div(&self, other: &NumberType) -> Result<Self, Error> {
        use NumberType::{Float, Integer};
        let result = match self {
            Integer(i) => match other {
                Integer(j) => {
                    if j == &0 {
                        return Err(Error {
                            message: String::from("Division by zero."),
                            error_type: ErrorType::SyntaxError,
                        });
                    }
                    Integer(i / j)
                }
                Float(f) => Float(*i as f64 / f),
            },
            Float(f) => match other {
                Integer(i) => Float(f / *i as f64),
                Float(g) => Float(f / g),
            },
        };
        Ok(result)
    }
}
