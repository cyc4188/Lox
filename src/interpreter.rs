use std::{rc::Rc, cell::RefCell};

use super::*;

pub struct Interpreter {
    environment: EnvironmentRef,
}

impl Interpreter {

    pub fn new() -> Self {
        Self {
            environment: Rc::new(RefCell::new(Environment::new(None))),
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

    fn number_operand_error(&self, operator: &Token) -> Result<Object, Error> {
        Err(
            Error {
                message: format!("Operand of {} must be a number.", operator.token_type),
                error_type: ErrorType::RuntimeError(operator.clone()),
            }
        )
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
            _ => unreachable!()
        }
    }
    fn visit_binary_expr(&mut self, expr: &Expr) -> Result<Object, Error> {
        match expr {
            Expr::Binary { left, operator, right } => {
                let left = self.evaluate(left)?;
                let right = self.evaluate(right)?;
                match operator.token_type {
                    TokenType::Minus => match (left, right) {
                            (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l - r)),
                            _ => self.number_operand_error(operator)
                    }
                    TokenType::Plus => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l + r)),
                        (Object::String(l), Object::String(r)) => Ok(Object::String(l + &r)),
                        _ => Err(
                            Error {
                                message: format!("Operands of {} must be two numbers or two strings.", operator.token_type),
                                error_type: ErrorType::RuntimeError(operator.clone()),
                            }
                        )
                    }
                    TokenType::Slash => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l / r)),
                        _ => self.number_operand_error(operator)
                    }
                    TokenType::Star => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l * r)),
                        _ => self.number_operand_error(operator)
                    }
                    TokenType::Greater => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l > r)),
                        (Object::String(l), Object::String(r)) => Ok(Object::Boolean(l > r)),
                        _ => self.number_operand_error(operator)
                    }
                    TokenType::GreaterEqual => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l >= r)),
                        (Object::String(l), Object::String(r)) => Ok(Object::Boolean(l >= r)),
                        _ => self.number_operand_error(operator)
                    }
                    TokenType::Less => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l < r)),
                        (Object::String(l), Object::String(r)) => Ok(Object::Boolean(l < r)),
                        _ => self.number_operand_error(operator)
                    }
                    TokenType::LessEqual => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l <= r)),
                        (Object::String(l), Object::String(r)) => Ok(Object::Boolean(l <= r)),
                        _ => self.number_operand_error(operator)
                    }
                    TokenType::BangEqual => Ok(Object::Boolean(!Interpreter::is_equal(&left, &right))),

                    TokenType::EqualEqual => Ok(Object::Boolean(Interpreter::is_equal(&left, &right))),

                    TokenType::And => {
                        if !Interpreter::is_truthy(&left) {
                            Ok(left)
                        } else {
                            Ok(right)
                        }
                    }

                    _ => unreachable!()
                }
            },
            _ => unreachable!()
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
            Expr::Variable { name } => {
                match self.environment.borrow().get(name.lexeme.as_str()) {
                    Some(value) => Ok(value),
                    None => Err(
                        Error {
                            message: format!("Undefined variable '{}'.", name.lexeme),
                            error_type: ErrorType::RuntimeError(name.clone()),
                        }
                    )
                }
            }
            _ => unreachable!()
        }
    }
    
    fn visit_assign_expr(&mut self, expr: &Expr) -> Result<Object, Error> {
        match expr {
            Expr::Assign { name, value }  => {
                let value = self.evaluate(value)?;
                self.environment.borrow_mut().assign(name, &value)?;
                return Ok(value);
            }
            _ => unreachable!()
        }
    }
}

impl stmt::Visitor<()> for Interpreter {
    fn visit_expr_stmt(&mut self, stmt: &Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::ExprStmt { expression } => {
                self.evaluate(expression)?;
            }
            _ => unreachable!()
        }

        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::PrintStmt { expression } => {
                let value = self.evaluate(expression)?;
                println!("{}", Interpreter::stringify(&value));
            }
            _ => unreachable!() 
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
            _ => unreachable!()
        }
        Ok(())
    }

    fn visit_block_stmt(&mut self, stmt: &Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::BlockStmt { statements }  => {
                // create a new scope
                let sub_env = Rc::new(RefCell::new(Environment::new(Some(self.environment.clone()))));

                self.environment = sub_env.clone();
                
                // iterate through the statements and execute them
                let mut steps = || -> Result<(), Error> {
                    for statement in statements {
                        self.execute(statement)?;
                    }
                    Ok(())
                };
                let result = steps();

                // remove the scope
                self.environment = sub_env.borrow().enclosing.clone().unwrap();

                result
            }
            _ => unreachable!()
        }
    }

    fn visit_if_stmt(&mut self, stmt: &Stmt) -> Result<(), Error> {
        // TODO
        match stmt {
            Stmt::IfStmt { condition, then_branch, else_branch } => {
                if Interpreter::is_truthy(&self.evaluate(condition)?) {
                    self.execute(then_branch)?;
                } else if let Some(else_branch) = else_branch {
                    self.execute(else_branch)?;
                }
                Ok(())
            }
            _ => unreachable!()
        } 
    }
}