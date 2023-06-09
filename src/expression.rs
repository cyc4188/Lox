use super::*;
use std::fmt;

pub mod expr {
    use super::{Error, Expr, Literal};
    pub trait Visitor<T> {
        fn visit_literal_expr(&mut self, value: &Literal) -> Result<T, Error>;
        fn visit_unary_expr(&mut self, expr: &Expr) -> Result<T, Error>;
        fn visit_binary_expr(&mut self, expr: &Expr) -> Result<T, Error>;
        fn visit_grouping_expr(&mut self, expr: &Expr) -> Result<T, Error>;
        fn visit_variable_expr(&mut self, expr: &Expr) -> Result<T, Error>;
        fn visit_assign_expr(&mut self, expr: &Expr) -> Result<T, Error>;
        fn visit_logic_expr(&mut self, expr: &Expr) -> Result<T, Error>;
        fn visit_index_expr(&mut self, expr: &Expr) -> Result<T, Error>;
        fn visit_call_expr(&mut self, expr: &Expr) -> Result<T, Error>;
        fn visit_get_expr(&mut self, expr: &Expr) -> Result<T, Error>;
        fn visit_set_expr(&mut self, expr: &Expr) -> Result<T, Error>;
        fn visit_index_set_expr(&mut self, expr: &Expr) -> Result<T, Error>;
        fn visit_this_expr(&mut self, expr: &Expr) -> Result<T, Error>;
        fn visit_super_expr(&mut self, expr: &Expr) -> Result<T, Error>;
        fn visit_list_expr(&mut self, expr: &Expr) -> Result<T, Error>;
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
    Literal {
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
    },
    Index {
        object: Box<Expr>,
        operator: Token,
        index: Box<Expr>,
        index_end: Option<Box<Expr>>,
    },
    Call {
        callee: Box<Expr>,
        paren: Token, // right paren
        arguments: Vec<Expr>,
    },
    Get {
        object: Box<Expr>,
        name: Token,
    },
    Set {
        object: Box<Expr>,
        name: Token,
        value: Box<Expr>,
    },
    IndexSet {
        object: Box<Expr>,
        index: Box<Expr>,
        index_end: Option<Box<Expr>>,
        value: Box<Expr>,
        operator: Token,
    },
    This {
        keyword: Token,
    },
    Super {
        keyword: Token,
        method: Token,
    },
    List {
        keyword: Token,
        elements: Vec<Expr>,
    },
}

impl Expr {
    #[allow(unused_variables)]
    pub fn accept<T>(&self, visitor: &mut impl expr::Visitor<T>) -> Result<T, Error> {
        match self {
            Expr::Literal { value } => visitor.visit_literal_expr(value),
            Expr::Unary { operator, right } => visitor.visit_unary_expr(self),
            Expr::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary_expr(self),
            Expr::Grouping { expression } => visitor.visit_grouping_expr(self),
            Expr::Variable { name } => visitor.visit_variable_expr(self),
            Expr::Assign { name, value } => visitor.visit_assign_expr(self),
            Expr::Logical {
                left,
                operator,
                right,
            } => visitor.visit_logic_expr(self),
            Expr::Index {
                object: left,
                operator,
                index: right,
                index_end,
            } => visitor.visit_index_expr(self),
            Expr::Call {
                callee,
                paren,
                arguments,
            } => visitor.visit_call_expr(self),
            Expr::Get { object, name } => visitor.visit_get_expr(self),
            Expr::Set {
                object,
                name,
                value,
            } => visitor.visit_set_expr(self),
            Expr::IndexSet { .. } => visitor.visit_index_set_expr(self),
            Expr::This { keyword } => visitor.visit_this_expr(self),
            Expr::Super { keyword, method } => visitor.visit_super_expr(self),
            Expr::List { keyword, elements } => visitor.visit_list_expr(self),
        }
    }
}

impl fmt::Display for Expr {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Literal { value } => write!(f, "{}", value),
            Expr::Unary { operator, right } => write!(f, "({} {})", operator, right),
            Expr::Binary {
                left,
                operator,
                right,
            } => write!(f, "({} {} {})", left, operator, right),
            Expr::Grouping { expression } => write!(f, "({})", expression),
            Expr::Variable { name } => write!(f, "{}", name.lexeme),
            Expr::Assign { name, value } => write!(f, "({} = {})", name.lexeme, value),
            Expr::Logical {
                left,
                operator,
                right,
            } => write!(f, "({} {} {})", left, operator, right),
            Expr::Index {
                object: left,
                operator,
                index: right,
                index_end,
            } => {
                write!(f, "{}", self.accept(&mut AstPrinter).unwrap())
            }
            Expr::Call {
                callee,
                paren,
                arguments,
            } => {
                // println!("{}", self.accept(&mut AstPrinter).unwrap());
                write!(f, "{}", self.accept(&mut AstPrinter).unwrap())
            }
            Expr::Get { object, name } => {
                write!(f, "{}", self.accept(&mut AstPrinter).unwrap())
            }
            Expr::Set {
                object,
                name,
                value,
            } => {
                write!(f, "{}", self.accept(&mut AstPrinter).unwrap())
            }
            Expr::IndexSet { .. } => {
                write!(f, "{}", self.accept(&mut AstPrinter).unwrap())
            }
            Expr::This { keyword } => {
                write!(f, "{}", self.accept(&mut AstPrinter).unwrap())
            }
            Expr::Super { keyword, method } => {
                write!(f, "{}", self.accept(&mut AstPrinter).unwrap())
            }
            Expr::List { keyword, elements } => {
                write!(f, "{}", self.accept(&mut AstPrinter).unwrap())
            }
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
            _ => Err(Error::new(
                "Expected unary expression",
                ErrorType::SyntaxError,
            )),
        }
    }

    fn visit_binary_expr(&mut self, expr: &Expr) -> Result<String, Error> {
        match expr {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = left.accept(self)?;
                let right = right.accept(self)?;
                Ok(format!("({} {} {})", left, operator, right))
            }
            _ => Err(Error::new(
                "Expected binary expression",
                ErrorType::SyntaxError,
            )),
        }
    }

    fn visit_grouping_expr(&mut self, expr: &Expr) -> Result<String, Error> {
        match expr {
            Expr::Grouping { expression } => {
                let expression = expression.accept(self)?;
                Ok(format!("({})", expression))
            }
            _ => Err(Error::new(
                "Expected grouping expression",
                ErrorType::SyntaxError,
            )),
        }
    }
    fn visit_variable_expr(&mut self, expr: &Expr) -> Result<String, Error> {
        match expr {
            Expr::Variable { name } => Ok(name.lexeme.to_string()),
            _ => Err(Error::new(
                "Expected variable expression",
                ErrorType::SyntaxError,
            )),
        }
    }
    fn visit_assign_expr(&mut self, expr: &Expr) -> Result<String, Error> {
        match expr {
            Expr::Assign { name, value } => {
                Ok(format!("({} = {})", name.lexeme, value.accept(self)?))
            }
            _ => Err(Error::new(
                "Expected assign expression",
                ErrorType::SyntaxError,
            )),
        }
    }
    fn visit_logic_expr(&mut self, expr: &Expr) -> Result<String, Error> {
        match expr {
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let left = left.accept(self)?;
                let right = right.accept(self)?;
                Ok(format!("({} {} {})", left, operator, right))
            }
            _ => Err(Error::new(
                "Expected logic expression",
                ErrorType::SyntaxError,
            )),
        }
    }
    fn visit_index_expr(&mut self, expr: &Expr) -> Result<String, Error> {
        match expr {
            Expr::Index {
                object: left,
                index: right,
                ..
            } => Ok(format!("{}[{}]", left.accept(self)?, right.accept(self)?,)),
            _ => Err(Error::new(
                "Expected index expression",
                ErrorType::SyntaxError,
            )),
        }
    }
    fn visit_call_expr(&mut self, expr: &Expr) -> Result<String, Error> {
        match expr {
            Expr::Call {
                callee, arguments, ..
            } => {
                let callee = callee.accept(self)?;
                let arguments = arguments
                    .iter()
                    .map(|arg| arg.accept(self))
                    .collect::<Result<Vec<String>, Error>>()?;
                Ok(format!("{}({})", callee, arguments.join(",")))
            }
            _ => Err(Error::new(
                "Expected call expression",
                ErrorType::SyntaxError,
            )),
        }
    }
    fn visit_get_expr(&mut self, expr: &Expr) -> Result<String, Error> {
        match expr {
            Expr::Get { object, name } => Ok(format!("({}.{})", object.accept(self)?, name.lexeme)),
            _ => Err(Error::new(
                "Expected get expression",
                ErrorType::SyntaxError,
            )),
        }
    }
    fn visit_set_expr(&mut self, expr: &Expr) -> Result<String, Error> {
        match expr {
            Expr::Set {
                object,
                name,
                value,
            } => Ok(format!(
                "(set: {}.{} = {})",
                object.accept(self)?,
                name,
                value.accept(self)?
            )),
            _ => Err(Error::new(
                "Expected set expression",
                ErrorType::SyntaxError,
            )),
        }
    }
    fn visit_index_set_expr(&mut self, expr: &Expr) -> Result<String, Error> {
        match expr {
            Expr::IndexSet {
                object,
                index,
                index_end,
                value,
                ..
            } => Ok(format!("{}[{}] = {}", object, index, value)),
            _ => unreachable!(),
        }
    }
    fn visit_this_expr(&mut self, expr: &Expr) -> Result<String, Error> {
        match expr {
            Expr::This { .. } => Ok("this ".to_string()),
            _ => unreachable!(),
        }
    }
    fn visit_super_expr(&mut self, expr: &Expr) -> Result<String, Error> {
        match expr {
            Expr::Super { .. } => Ok("super ".to_string()),
            _ => unreachable!(),
        }
    }
    fn visit_list_expr(&mut self, expr: &Expr) -> Result<String, Error> {
        match expr {
            Expr::List { elements, .. } => {
                let elements = elements
                    .iter()
                    .map(|e| e.accept(self))
                    .collect::<Result<Vec<String>, Error>>()?;
                Ok(format!("[{}]", elements.join(",")))
            }
            _ => unreachable!(),
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
                operator: Token::new("-", TokenType::Minus, 1, 0),
                right: Box::new(Expr::Literal {
                    value: Literal::Number(NumberType::Integer(123)),
                }),
            }),
            operator: Token::new("*", TokenType::Star, 1, 0),
            right: Box::new(Expr::Grouping {
                expression: Box::new(Expr::Literal {
                    value: Literal::Number(NumberType::Float(45.67)),
                }),
            }),
        };
        println!("{}", expr);
        println!("{}", expr.accept(&mut AstPrinter).unwrap());
    }
}
