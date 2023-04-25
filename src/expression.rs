use super::*;
use std::fmt::{self};

pub mod expr {
    use super::{Literal, Expr, Error};
    pub trait Visitor<T> {
        fn visit_literal_expr(&mut self, value: &Literal) -> Result<T, Error>;
        fn visit_unary_expr(&mut self, expr: &Expr) -> Result<T, Error>;
        fn visit_binary_expr(&mut self, expr: &Expr) -> Result<T, Error>;
        fn visit_grouping_expr(&mut self, expr: &Expr) -> Result<T, Error>;
        fn visit_variable_expr(&mut self, expr: &Expr) -> Result<T, Error>;
        fn visit_assign_expr(&mut self, expr: &Expr) -> Result<T, Error>;
        fn visit_logic_expr(&mut self, expr: &Expr) -> Result<T, Error>;
    }
}

///expression     → literal
//                | unary
//                | binary
//                | grouping ;
//
// literal        → NUMBER | STRING | "true" | "false" | "nil" ;
// grouping       → "(" expression ")" ;
// unary          → ( "-" | "!" ) expression ;
// binary         → expression operator expression ;
// operator       → "==" | "!=" | "<" | "<=" | ">" | ">="
//                | "+"  | "-"  | "*" | "/" ;
/// An expression is a node in the AST that represents a value.
/// have Arbitrary child nodes
#[derive(Debug, Clone)]
pub enum Expr {
    Literal{
        value: Literal,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Variable {
        name: Token,
    },
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    }
}


impl Expr {
    #[allow(unused_variables)]
    pub fn accept<T>(&self, visitor: &mut impl expr::Visitor<T>) -> Result<T, Error> {
        match self {
            Expr::Literal { value } => visitor.visit_literal_expr(value),
            Expr::Unary { operator, right } => visitor.visit_unary_expr(self),
            Expr::Binary { left, operator, right } => visitor.visit_binary_expr(self),
            Expr::Grouping { expression } => visitor.visit_grouping_expr(self),
            Expr::Variable { name } => visitor.visit_variable_expr(self),
            Expr::Assign { name, value } => visitor.visit_assign_expr(self),
            Expr::Logical { left, operator, right } => visitor.visit_logic_expr(self),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Literal { value } => write!(f, "{}", value),
            Expr::Unary { operator, right } => write!(f, "({} {})", operator, right),
            Expr::Binary { left, operator, right } => write!(f, "({} {} {})", left, operator, right),
            Expr::Grouping { expression } => write!(f, "({})", expression),
            Expr::Variable { name } => write!(f, "{}", name.lexeme),
            Expr::Assign { name, value } => write!(f, "({} = {})", name.lexeme, value),
            Expr::Logical { left, operator, right } => write!(f, "({} {} {})", left, operator, right),
        }
    }
}


pub struct AstPrinter;

impl AstPrinter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn print(&mut self, expr: &Expr) -> Result<String, Error> {
        expr.accept(self)
    }
}

impl Default for AstPrinter {
    fn default() -> Self {
        Self::new()
    }
}

impl expr::Visitor<String> for AstPrinter {
    fn visit_literal_expr(&mut self, value: &Literal) -> Result<String, Error> {
        Ok(format!("{}", value))
    }

    fn visit_unary_expr(&mut self, expr: &Expr) -> Result<String, Error> {
        match expr {
            Expr::Unary { operator, right } => {
                let right = right.accept(self)?;
                Ok(format!("({} {})", operator, right))
            }
            _ => Err(Error::new("Expected unary expression", ErrorType::SyntaxError)),
        }
    }

    fn visit_binary_expr(&mut self, expr: &Expr) -> Result<String, Error> {
        match expr {
            Expr::Binary { left, operator, right } => {
                let left = left.accept(self)?;
                let right = right.accept(self)?;
                Ok(format!("({} {} {})", left, operator, right))
            }
            _ => Err(Error::new("Expected binary expression", ErrorType::SyntaxError)),
        }
    }

    fn visit_grouping_expr(&mut self, expr: &Expr) -> Result<String, Error> {
        match expr {
            Expr::Grouping { expression } => {
                let expression = expression.accept(self)?;
                Ok(format!("({})", expression))
            }
            _ => Err(Error::new("Expected grouping expression", ErrorType::SyntaxError)),
        }
    }
    fn visit_variable_expr(&mut self, expr: &Expr) -> Result<String, Error> {
        match expr {
            Expr::Variable { name } => {
                Ok(name.lexeme.to_string())
            }
            _ => Err(Error::new("Expected variable expression", ErrorType::SyntaxError)),
        }
    }
    fn visit_assign_expr(&mut self, expr: &Expr) -> Result<String, Error> {
        match expr {
            Expr::Assign { name, value }  => {
                Ok(format!("({} = {})", name.lexeme, value.accept(self)?))
            }
            _ => Err(Error::new("Expected assign expression", ErrorType::SyntaxError)),
        }
    }
    fn visit_logic_expr(&mut self, expr: &Expr) -> Result<String, Error> {
        match expr {
            Expr::Logical { left, operator, right } => {
                let left = left.accept(self)?;
                let right = right.accept(self)?;
                Ok(format!("({} {} {})", left, operator, right))
            }
            _ => Err(Error::new("Expected logic expression", ErrorType::SyntaxError)),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Unary {
                operator: Token::new("-", TokenType::Minus, Literal::Nil, 1),
                right: Box::new(Expr::Literal {
                    value: Literal::Number(123.0),
                }),
            }),
            operator: Token::new("*", TokenType::Star,  Literal::Nil, 1),
            right: Box::new(Expr::Grouping {
                expression: Box::new(Expr::Literal {
                    value: Literal::Number(45.67),
                }),
            }),
        };
        println!("{}", expr);
        println!("{}", expr.accept(&mut AstPrinter).unwrap());
    }
}
