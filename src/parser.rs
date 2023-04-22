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



/// program        → declaration* EOF ;
/// declaration    → varDecl
///                 | statement ;
/// varDecl        → "var" IDENTIFIER ( "=" expression )? ";" ;
/// statement      → exprStmt
///                | printStmt ;
/// exprStmt       → expression ";" ;
/// printStmt      → "print" expression ";" ;
/// 
/// expression     → equality ;
/// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
/// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
/// term           → factor ( ( "-" | "+" ) factor )* ;
/// factor         → unary ( ( "/" | "*" ) unary )* ;
/// unary          → ( "!" | "-" ) unary
///                | primary ;
/// primary        → NUMBER | STRING | "true" | "false" | "nil"
///                | "(" expression ")" 
///                | IDENTIFIER ;
impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, Error> {
        let mut stmts: Vec<Stmt> = Vec::new();
        while !self.is_end() {
            let stmt = self.declaration()?; 
            stmts.push(stmt);
        }
        Ok(stmts)
    }


// statement parser
    
    /// declaration    → varDecl
    ///                 | statement ;
    fn declaration(&mut self) -> Result<Stmt, Error> {
        let res: Result<Stmt, Error> = if matches!(self, Var) {
            self.var_decl()
        } else {
            self.statement()
        };

        if res.is_err() {
            self.synchronize();
        }

        res
    }

    /// varDecl        → "var" IDENTIFIER ( "=" expression )? ";" ;
    fn var_decl(&mut self) -> Result<Stmt, Error> {
        let name = self.consume(Identifier, "Expect variable name")?.clone();

        let mut initializer: Option<Expr> = None; 

        if matches!(self, Equal) {
            initializer = Some(self.expression()?);
        }

        self.consume(Semicolon, "Expect ';' after variable declaration")?;

        Ok(Stmt::VarStmt { name, initializer })
    }

    /// statement      → exprStmt
    ///                | printStmt ;
    fn statement(&mut self) -> Result<Stmt, Error> {
        if matches!(self, Print) {
            return self.print_statement();
        }

        self.expression_statement()
    }

    /// exprStmt       → expression ";" ;
    fn expression_statement(&mut self) -> Result<Stmt, Error> {
        let expr = self.expression()?;
        self.consume(Semicolon, "Expected ';' after value")?;

        Ok(Stmt::ExprStmt { expression: expr })
    }

    /// printStmt      → "print" expression ";" ;
    fn print_statement(&mut self) -> Result<Stmt, Error> {
        let expr = self.expression()?;
        self.consume(Semicolon, "Expected ';' after value")?;

        Ok(Stmt::PrintStmt { expression: expr }) 
    }

// ------------------------------------------------
// ------------------------------------------------
// expression parser

    /// expression     → equality ;
    fn expression(&mut self) -> Result<Expr, Error> {
        self.equality()
    }
    
    /// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Result<Expr, Error> {
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

    /// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> Result<Expr, Error> {
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

    /// term           → factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Result<Expr, Error> {
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

    /// factor         → unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> Result<Expr, Error> {
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

    /// unary          → ( "!" | "-" ) unary
    ///                | primary ;
    fn unary(&mut self) -> Result<Expr, Error> {
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

    /// primary        → NUMBER | STRING | "true" | "false" | "nil"
    ///                | "(" expression ")" 
    ///                | IDENTIFIER ;
    fn primary(&mut self) -> Result<Expr, Error> {
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

        if matches!(self, Identifier) {
            return Ok(Expr::Variable { name: self.previous().clone() });
        }

        if matches!(self, LeftParen) {
            let expr = self.expression()?;
            
            self.consume(RightParen, "Expect ')' after expression.")?;

            return Ok(Expr::Grouping { expression: Box::new(expr) });
        }
        Err(self.error(self.peak(), "Expect expression"))
        // Err(Error {
        //     message: "Expect expression".to_string(),
        //     error_type: ErrorType::SyntaxError
        // })
    }


// expression parser
// ------------------------------------------------
// ------------------------------------------------
    
    fn peak(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn is_end(&self) -> bool {
        self.peak().token_type == TokenType::Eof
    }

    fn advance(&mut self) -> &Token {
        if !self.is_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_end() {
            return false;
        }
        if self.peak().token_type != token_type {
            return false;
        }
        true
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, Error> {
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

    // until we reach a semicolon ';' or a statement keyword
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peak().token_type {
                Class | Fun | Var | For | If | While | Print | Return => return,
                _ => (),
            }

            self.advance();
        }
    }

}

