use super::*;

pub struct Interpreter;

impl Interpreter {

    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(&mut self, expr: &Expr) -> Result<Object, Error> {
        self.evaluate(expr)
    }

    pub fn evaluate(&mut self, expr: &Expr) -> Result<Object, Error> {
        expr.accept(self)
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

impl Visitor<Object> for Interpreter {
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
            Expr::Grouping { expression } => self.evaluate(expression),
            _ => unreachable!(),
        }
    }
}
