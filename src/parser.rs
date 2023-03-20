use super::*;
use TokenType::*;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

macro_rules! matches {
    ( $self:ident, $( $token_type:expr ),* ) => {
        {
            if $($self.check($token_type)) || * {
                $self.advance();
                true
            }
            else {
                false
            }
        }
    };
}

///expression     → equality ;
/// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
/// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
/// term           → factor ( ( "-" | "+" ) factor )* ;
/// factor         → unary ( ( "/" | "*" ) unary )* ;
/// unary          → ( "!" | "-" ) unary
///                | primary ;
/// primary        → NUMBER | STRING | "true" | "false" | "nil"
///                | "(" expression ")" ;
impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }


// expression parser[
// ------------------------------------------------
// ------------------------------------------------

    pub fn expression(&mut self) -> Result<Expr, Error> {
        self.equality()
    }
    
    pub fn equality(&mut self) -> Result<Expr, Error> {
        let mut expr = self.comparison()?;

        while matches!(self, BangEqual, EqualEqual) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            let left = expr; // give expr to left
            expr = Expr::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    pub fn comparison(&mut self) -> Result<Expr, Error> {
        let mut expr = self.term()?;
        while matches!(self, Greater, GreaterEqual, Less, LessEqual) {
            let operator = self.previous().clone();
            let right = self.term()?;
            let left = expr;
            expr = Expr::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    pub fn term(&mut self) -> Result<Expr, Error> {
        let mut expr = self.factor()?;
        while matches!(self, Minus, Plus) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            let left = expr;
            expr = Expr::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    pub fn factor(&mut self) -> Result<Expr, Error> {
        let mut expr = self.unary()?;
        while matches!(self, Slash, Star) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            let left = expr;
            expr = Expr::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    pub fn unary(&mut self) -> Result<Expr, Error> {
        if matches!(self, Bang, Minus) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        } 
        self.primary()
    }

    pub fn primary(&mut self) -> Result<Expr, Error> {
        if matches!(self, False) {
            return Ok(Expr::Literal { value: Literal::Boolean(false) });
        }
        if matches!(self, True) {
            return Ok(Expr::Literal { value: Literal::Boolean(true) });
        }

        if matches!(self, Nil) {
            return Ok(Expr::Literal { value: Literal::Nil });
        }

        if matches!(self, Number, String) {
            return Ok(
                Expr::Literal { 
                    value: self.previous().literal.clone()
                    }
                )
        }

        if matches!(self, LeftParen) {
            let expr = self.expression()?;
            
            self.consume(RightParen, "Expect ')' after expression.")?;

            return Ok(Expr::Grouping { expression: Box::new(expr) });
        }
        Err(Error {
            message: "Expect expression".to_string(),
            error_type: ErrorType::SyntaxError
        })
    }


// expression parser]
// ------------------------------------------------
// ------------------------------------------------
    
    pub fn peak(&self) -> &Token {
        &self.tokens[self.current]
    }

    pub fn is_end(&self) -> bool {
        self.peak().token_type == TokenType::Eof
    }

    pub fn advance(&mut self) -> &Token {
        if !self.is_end() {
            self.current += 1;
        }
        self.previous()
    }

    pub fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    pub fn check(&mut self, token_type: TokenType) -> bool {
        if self.is_end() {
            return false;
        }
        if self.peak().token_type != token_type {
            return false;
        }
        true
    }

    pub fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, Error> {
        if self.check(token_type) {
            return Ok(self.advance());
        }
        Err(Error {
            message: message.to_string(),
            error_type: ErrorType::SyntaxError,
        })
    }

    pub fn error(&self, token: &Token, message: &str) -> Error {
        parse_error(token, message);
        Error {
            message: message.to_string(),
            error_type: ErrorType::SyntaxError,
        }
    }

}

