use std::cell::RefCell;
use std::fmt::{Debug, Display};
use std::rc::Rc;

use crate::Error;
use crate::Interpreter;
use crate::Object;
use crate::Stmt;
use crate::Token;
use crate::{Environment, EnvironmentRef, ErrorType};

#[derive(Clone)]
pub enum Function {
    Native {
        name: String,
        arity: usize,
        body: Box<fn(&Vec<Object>) -> Object>,
    },
    UserDefined {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
        closure: EnvironmentRef,
    },
}

impl Function {
    pub fn call(&self, interpreter: &mut Interpreter, args: &Vec<Object>) -> Result<Object, Error> {
        match self {
            Function::Native { body, .. } => Ok(body(args)),
            Function::UserDefined {
                params,
                body,
                closure,
                ..
            } => {
                // new environment for function call
                let environment = Rc::new(RefCell::new(Environment::new(Some(closure.clone()))));

                // define parameters
                for (i, param) in params.iter().enumerate() {
                    environment
                        .borrow_mut()
                        .define(&param.lexeme, args[i].clone());
                }

                if let Err(err) = interpreter.execute_block(body, environment) {
                    match err.error_type {
                        ErrorType::Return(value) => Ok(value),
                        _ => Err(err),
                    }
                } else {
                    Ok(Object::Nil)
                }
            }
        }
    }

    pub fn arity(&self) -> usize {
        match self {
            Function::Native { arity, .. } => *arity,
            Function::UserDefined { params, .. } => params.len(),
        }
    }

    pub fn bind(&self, instance: Object) -> Function {
        match self {
            Function::UserDefined {
                name,
                params,
                body,
                closure,
            } => {
                let mut environment_inner = Environment::new(Some(closure.clone()));
                environment_inner.define(&String::from("this"), instance);
                let environment = Rc::new(RefCell::new(environment_inner));
                return Function::UserDefined {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                    closure: environment,
                };
            }
            _ => unreachable!(),
        }
    }
}

impl Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Function::Native { name, .. } => write!(f, "native <fn {}>", name),
            Function::UserDefined { name, params, .. } => {
                write!(
                    f,
                    "user define <fn {}({})>",
                    name.lexeme,
                    params
                        .iter()
                        .map(|param| param.lexeme.clone())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Function::Native { name, .. } => write!(f, "native <fn {}>", name),
            Function::UserDefined { name, params, .. } => {
                write!(
                    f,
                    "user define <fn {}({})>",
                    name.lexeme,
                    params
                        .iter()
                        .map(|param| param.lexeme.clone())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
        }
    }
}
