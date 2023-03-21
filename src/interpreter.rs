use super::*;

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    fn number_operand_error(&self, operator: &Token) -> Result<Object, Error> {
        Err(
            Error {
                message: format!("Operand of {} must be a number.", operator.token_type),
                error_type: ErrorType::RuntimeError,
            }
        )
    }

    fn is_truthy(&self, object: &Object) -> bool {
        match object {
            Object::Nil => false,
            Object::Boolean(b) => *b,
            _ => true,
        }
    }

    fn is_equal(&self, left: &Object, right: &Object) -> bool {
        todo!();
    }
}

impl Visitor<Object> for Interpreter {
    fn visit_literal_expr(&mut self, value: &Literal) -> Result<Object, Error> {
        match value {
            Literal::Boolean(b) => Ok(Object::Boolean(b.clone())),
            Literal::Nil => Ok(Object::Nil),
            Literal::Number(n) => Ok(Object::Number(n.clone())),
            Literal::String(s) => Ok(Object::String(s.clone())),
        }
    }
    fn visit_unary_expr(&mut self, expr: &Expr) -> Result<Object, Error> {
        match expr {
            Expr::Unary { operator, right } => {
                let right = right.accept(self)?;
                
                // -, !
                match operator.token_type {
                    TokenType::Minus => {
                        match right {
                            // check if right is a number
                            Object::Number(n) => Ok(Object::Number(-n)),
                            _ => self.number_operand_error(operator),
                        }
                    }
                    TokenType::Bang => Ok(Object::Boolean(!self.is_truthy(&right))),
                    _ => unreachable!(),
                }
            }
            _ => unreachable!()
        }
    }
    fn visit_binary_expr(&mut self, expr: &Expr) -> Result<Object, Error> {
        match expr {
            Expr::Binary { left, operator, right } => {
                let left = left.accept(self)?;
                let right = right.accept(self)?;
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
                                error_type: ErrorType::RuntimeError,
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
                        _ => self.number_operand_error(operator)
                    }

                    TokenType::GreaterEqual => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l >= r)),
                        _ => self.number_operand_error(operator)
                    }

                    TokenType::Less => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l < r)),
                        _ => self.number_operand_error(operator)
                    }

                    TokenType::LessEqual => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l <= r)),
                        _ => self.number_operand_error(operator)
                    }

                    TokenType::BangEqual => Ok(Object::Boolean(self.is_equal(&left, &right))),

                    TokenType::And => {
                        if !self.is_truthy(&left) {
                            Ok(left)
                        } else {
                            Ok(right)
                        }
                    }

                    // todo!
                    // not complete

                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        }
    }
    fn visit_grouping_expr(&mut self, expr: &Expr) -> Result<Object, Error> {
        match expr {
            Expr::Grouping { expression } => expression.accept(self),
            _ => unreachable!(),
        }
    }
}