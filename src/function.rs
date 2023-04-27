use std::fmt::{Debug, Display};

use crate::Interpreter;
use crate::Object;
use crate::Error;
use crate::interpreter;


#[derive(Clone)]
pub enum Function {
    Native {
        arity: usize,
        body: Box<fn (&Vec<Object>) -> Object>,
    }
}

impl Function {

    pub fn call(&self, interpreter: &mut Interpreter, args: &Vec<Object>) -> Result<Object, Error> {
        match self {
            Function::Native { body, .. } => {
                Ok(body(args))
            }
        }
    }
    
    pub fn arity(&self) -> usize {
        match self {
            Function::Native { arity, .. } => *arity, 
        }    
    }    
}

impl Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn>")
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn>")
    } 
}