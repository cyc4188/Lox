use std::{cell::RefCell, rc::Rc};
use std::collections::HashMap;

use super::*;

pub struct Interpreter {
    environment: EnvironmentRef,
    pub globals: EnvironmentRef,
    pub locals: HashMap<Token, usize>,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::new(None)));

        let clock: Object = Object::Callable(Function::Native {
            name: "clock".to_string(), 
            arity: 0,
            body: Box::new(|_: &Vec<Object>| -> Object {
                Object::Number(
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs_f64(),
                )
            }),
        });
        globals.borrow_mut().define(&"clock".to_string(), clock);

        Self {
            environment: globals.clone(),
            globals,
            locals: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, stmts: &Vec<Stmt>) -> Result<(), Error> {
        for stmt in stmts {
            self.execute(stmt)?;
        }
        Ok(())
    }

    pub fn evaluate(&mut self, expr: &Expr) -> Result<Object, Error> {
        expr.accept(self)
    }

    pub fn execute(&mut self, stmt: &Stmt) -> Result<(), Error> {
        stmt.accept(self)
    }

    pub fn execute_block(
        &mut self,
        stmts: &Vec<Stmt>,
        environment: EnvironmentRef,
    ) -> Result<(), Error> {
        let previous = self.environment.clone();
        self.environment = environment;
        let mut steps = || -> Result<(), Error> {
            for statement in stmts {
                self.execute(statement)?;
            }
            Ok(())
        };
        let result = steps();
        self.environment = previous;
        result
    }

    fn number_operand_error(&self, operator: &Token) -> Result<Object, Error> {
        Err(Error {
            message: format!("Operand of {} must be a number.", operator.token_type),
            error_type: ErrorType::RuntimeError(operator.clone()),
        })
    }

    fn is_truthy(object: &Object) -> bool {
        match object {
            Object::Nil => false,
            Object::Boolean(b) => *b,
            _ => true,
        }
    }

    fn is_equal(left: &Object, right: &Object) -> bool {
        left.equals(right)
    }

    fn stringify(object: &Object) -> String {
        match object {
            Object::Nil => "nil".to_string(),
            Object::Boolean(b) => b.to_string(),
            Object::Number(n) => n.to_string(),
            Object::String(s) => s.clone(),
            Object::Callable(function) => function.to_string(),
        }
    }

    pub fn resolve(&mut self, token: &Token, depth: usize) {
        self.locals.insert(token.clone(), depth);
    }

    fn look_up_variable(&self, name: &Token) -> Result<Object, Error> {
        let result: Option<Object> = match self.locals.get(name) {
            Some(distance) => {
                self.environment.borrow().get_at(*distance, &name.lexeme)
            }
            None => {
                self.globals.borrow().get(&name.lexeme)
            }
        };
        if let Some(obj) = result {
            Ok(obj)
        }
        else {
            Err(Error {
                message: format!("Undefined variable {}.", name.lexeme),
                error_type: ErrorType::RuntimeError(name.clone()),
            })
        }
    }
}

impl expr::Visitor<Object> for Interpreter {
    fn visit_literal_expr(&mut self, value: &Literal) -> Result<Object, Error> {
        match value {
            Literal::Boolean(b) => Ok(Object::Boolean(*b)),
            Literal::Nil => Ok(Object::Nil),
            Literal::Number(n) => Ok(Object::Number(*n)),
            Literal::String(s) => Ok(Object::String(s.clone())),
        }
    }
    fn visit_unary_expr(&mut self, expr: &Expr) -> Result<Object, Error> {
        match expr {
            Expr::Unary { operator, right } => {
                let right = self.evaluate(right)?;

                // -, !
                match operator.token_type {
                    TokenType::Minus => {
                        match right {
                            // check if right is a number
                            Object::Number(n) => Ok(Object::Number(-n)),
                            _ => self.number_operand_error(operator),
                        }
                    }
                    TokenType::Bang => Ok(Object::Boolean(!Interpreter::is_truthy(&right))),
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
    fn visit_binary_expr(&mut self, expr: &Expr) -> Result<Object, Error> {
        match expr {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(left)?;
                let right = self.evaluate(right)?;
                match operator.token_type {
                    TokenType::Minus => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l - r)),
                        _ => self.number_operand_error(operator),
                    },
                    TokenType::Plus => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l + r)),
                        (Object::String(l), Object::String(r)) => Ok(Object::String(l + &r)),
                        _ => Err(Error {
                            message: format!(
                                "Operands of {} must be two numbers or two strings.",
                                operator.token_type
                            ),
                            error_type: ErrorType::RuntimeError(operator.clone()),
                        }),
                    },
                    TokenType::Slash => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l / r)),
                        _ => self.number_operand_error(operator),
                    },
                    TokenType::Star => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l * r)),
                        _ => self.number_operand_error(operator),
                    },
                    TokenType::Greater => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l > r)),
                        (Object::String(l), Object::String(r)) => Ok(Object::Boolean(l > r)),
                        _ => self.number_operand_error(operator),
                    },
                    TokenType::GreaterEqual => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l >= r)),
                        (Object::String(l), Object::String(r)) => Ok(Object::Boolean(l >= r)),
                        _ => self.number_operand_error(operator),
                    },
                    TokenType::Less => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l < r)),
                        (Object::String(l), Object::String(r)) => Ok(Object::Boolean(l < r)),
                        _ => self.number_operand_error(operator),
                    },
                    TokenType::LessEqual => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l <= r)),
                        (Object::String(l), Object::String(r)) => Ok(Object::Boolean(l <= r)),
                        _ => self.number_operand_error(operator),
                    },
                    TokenType::BangEqual => {
                        Ok(Object::Boolean(!Interpreter::is_equal(&left, &right)))
                    }

                    TokenType::EqualEqual => {
                        Ok(Object::Boolean(Interpreter::is_equal(&left, &right)))
                    }

                    TokenType::And => {
                        if !Interpreter::is_truthy(&left) {
                            Ok(left)
                        } else {
                            Ok(right)
                        }
                    }

                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
    fn visit_grouping_expr(&mut self, expr: &Expr) -> Result<Object, Error> {
        match expr {
            Expr::Grouping { expression } => self.evaluate(expression),
            _ => unreachable!(),
        }
    }
    fn visit_variable_expr(&mut self, expr: &Expr) -> Result<Object, Error> {
        match expr {
            Expr::Variable { name } => self.look_up_variable(name),
            _ => unreachable!(),
        }
    }

    fn visit_assign_expr(&mut self, expr: &Expr) -> Result<Object, Error> {
        match expr {
            Expr::Assign { name, value } => {
                let value = self.evaluate(value)?;
                
                let distance = self.locals.get(name);
                if let Some(distance) = distance {
                    self.environment.borrow_mut().assign_at(*distance, name, &value)?;
                    Ok(value)
                }
                else {
                    self.environment.borrow_mut().assign(name, &value)?;
                    Ok(value)
                }
            }
            _ => unreachable!(),
        }
    }

    fn visit_logic_expr(&mut self, expr: &Expr) -> Result<Object, Error> {
        match expr {
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let left_value = self.evaluate(left)?;
                if operator.token_type == TokenType::Or {
                    if Interpreter::is_truthy(&left_value) {
                        Ok(left_value)
                    } else {
                        Ok(self.evaluate(right)?)
                    }
                } else if Interpreter::is_truthy(&left_value) {
                    Ok(self.evaluate(right)?)
                } else {
                    Ok(left_value)
                }
                
            }
            _ => unreachable!(),
        }
    }

    fn visit_call_expr(&mut self, expr: &Expr) -> Result<Object, Error> {
        trace!("visit_call_expr");
        match expr {
            Expr::Call {
                callee,
                paren,
                arguments,
            } => {
                let callee = self.evaluate(callee)?;

                let mut args = Vec::new();
                for arg in arguments {
                    args.push(self.evaluate(arg)?);
                }

                // check if callee is a function
                if let Object::Callable(function) = callee {
                    // check if number of arguments matches number of parameters
                    trace!("function arity: {}", function.arity(), );
                    trace!("args.len: {}", args.len());
                    if function.arity() != args.len() {
                        return Err(Error {
                            message: format!(
                                "Expected {} arguments but got {}.",
                                function.arity(),
                                args.len()
                            ),
                            error_type: ErrorType::RuntimeError(paren.clone()),
                        });
                    }
                    // call function
                    Ok(function.call(self, &args)?)
                } else {
                    Err(Error {
                        message: "Can only call functions and classes.".to_string(),
                        error_type: ErrorType::RuntimeError(paren.clone()),
                    })
                }
            }
            _ => unreachable!(),
        }
    }
}

impl stmt::Visitor<()> for Interpreter {
    fn visit_expr_stmt(&mut self, stmt: &Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::ExprStmt { expression } => {
                self.evaluate(expression)?;
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::PrintStmt { expression } => {
                let value = self.evaluate(expression)?;
                println!("{}", Interpreter::stringify(&value));
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: &Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::VarStmt { name, initializer } => {
                let value = match initializer {
                    Some(expr) => self.evaluate(expr)?,
                    None => Object::Nil,
                };
                self.environment.borrow_mut().define(&name.lexeme, value);
            }
            _ => unreachable!(),
        }
        Ok(())
    }

    fn visit_block_stmt(&mut self, stmt: &Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::BlockStmt { statements } => {
                // create a new scope
                let sub_env = Rc::new(RefCell::new(Environment::new(Some(
                    self.environment.clone(),
                ))));

                self.execute_block(statements, sub_env)
            }
            _ => unreachable!(),
        }
    }

    fn visit_if_stmt(&mut self, stmt: &Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::IfStmt {
                condition,
                then_branch,
                else_branch,
            } => {
                if Interpreter::is_truthy(&self.evaluate(condition)?) {
                    self.execute(then_branch)?;
                } else if let Some(else_branch) = else_branch {
                    self.execute(else_branch)?;
                }
                Ok(())
            }
            _ => unreachable!(),
        }
    }

    fn visit_while_stmt(&mut self, stmt: &Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::WhileStmt { condition, body } => {
                while Interpreter::is_truthy(&self.evaluate(condition)?) {
                    self.execute(body)?;
                }
                Ok(())
            }
            _ => unreachable!(),
        }
    }

    fn visit_func_stmt(&mut self, stmt: &Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::FunStmt { name, params, body } => {
                let function = Object::Callable(Function::UserDefined {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                    closure: self.environment.clone(),
                });

                self.environment
                .borrow_mut()
                .define(&name.lexeme, function);

                Ok(())
            }
            _ => unreachable!(),
        }
    }
    fn visit_return_stmt(&mut self, stmt: &Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::ReturnStmt {value , ..} => {
                let value = match value {
                    Some(expr) => self.evaluate(expr)?,
                    None => Object::Nil,
                };
                Err(Error {
                    message: String::from("Return statement"),
                    error_type: ErrorType::Return(value),
                })
            }
            _ => unreachable!()
        } 
    }
}
