use std::vec;

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
///                 | funDecl
///                 | statement
///                 | classDecl ;
/// funDecl        → "fun" function ;
/// function       → IDENTIFIER "(" parameters? ")" block ;
/// varDecl        → "var" IDENTIFIER ( "=" expression )? ";" ;
/// classDecl      → "class" IDENTIFIER ( "<" IDENTIFIER )? "{" function* "}" ;
/// statement      → exprStmt
///                | ifStmt
///                | printStmt
///                | block
///                | whileStmt
///                | forStmt
///                | returnStmt ;
/// exprStmt       → expression ";" ;
/// ifStmt         → "if" "(" expression ")" statement ( "else" statement )? ;
/// printStmt      → "print" expression ";" ;
/// block          | "{" declaration* "}" ;
/// whileStmt      | "while" "(" expression ")" statement ;
/// forStmt        | "for" "(" ( varDecl | exprStmt | ";" )
///                         expression? ";"
///                         expression? ")" statement ;
/// returnStmt     | "return" expression? ";" ;
/// expression     → assignment ;
/// assignment     → ( call "." )? IDENTIFIER "=" assignment
///                | logicOr ;
/// logicOr        → logicAnd ( "or" logicAnd )* ;
/// logicAnd       → equality ( "and" equality )* ;
/// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
/// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
/// term           → factor ( ( "-" | "+" ) factor )* ;
/// factor         → unary ( ( "/" | "*" ) unary )* ;
/// unary          → ( "!" | "-" ) unary
///                | call_index ;
/// call_index     → index ( "(" arguments? ")" | "." IDENTIFIER | "[" index "]")* ;
/// primary        → NUMBER | STRING | "true" | "false" | "nil"
///                | "(" expression ")"
///                | IDENTIFIER
///                | this
///                | super "." primary ;
/// arguments      | expression ( "," expression )* ;
/// parameters     | IDENTIFIER ( "," IDENTIFIER )* ;
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
        } else if matches!(self, Fun) {
            self.function("function")
        } else if matches!(self, Class) {
            self.class_decl()
        } else {
            self.statement()
        };

        if res.is_err() {
            self.synchronize();
        }

        res
    }

    /// classDecl      → "class" IDENTIFIER ( "<" IDENTIFIER )? "{" function* "}" ;
    fn class_decl(&mut self) -> Result<Stmt, Error> {
        let name = self.consume(Identifier, "Expect class name.")?.clone();
        let mut super_class: Option<Expr> = None;

        if matches!(self, Less) {
            self.consume(Identifier, "Expect superclass name.")?;
            super_class = Some(Expr::Variable {
                name: self.previous().clone(),
            });
        }

        self.consume(LeftBrace, "Expect '{' before class body.")?;

        // get methods
        let mut methods = Vec::new();
        while !self.check(RightBrace) {
            methods.push(self.function("method")?);
        }
        self.consume(RightBrace, "Expect '}' after class body.")?;

        Ok(Stmt::ClassStmt {
            name,
            super_class,
            methods,
        })
    }

    /// varDecl        → "var" IDENTIFIER ( "=" expression )? ";" ;
    fn var_decl(&mut self) -> Result<Stmt, Error> {
        let name = self.consume(Identifier, "Expect variable name.")?.clone();

        let mut initializer: Option<Expr> = None;

        if matches!(self, Equal) {
            initializer = Some(self.expression()?);
        }

        self.consume(Semicolon, "Expect ';' after variable declaration.")?;

        Ok(Stmt::VarStmt { name, initializer })
    }

    /// funDecl        → "fun" function ;
    /// function       → IDENTIFIER "(" parameters? ")" block ;
    fn function(&mut self, kind: &str) -> Result<Stmt, Error> {
        let name = self.consume(Identifier, "Expect function name.")?.clone();
        self.consume(LeftParen, "Expect '(' after function name.")?;
        let mut parameters: Vec<Token> = Vec::new();
        if !self.check(RightParen) {
            loop {
                if parameters.len() >= 255 {
                    return Err(self.error(self.peak(), "Can't have more than 255 parameters."));
                }
                parameters.push(self.consume(Identifier, "Expect parameter name.")?.clone());
                if !matches!(self, Comma) {
                    break;
                }
            }
        }
        self.consume(RightParen, "Expect ')' after parameters.")?;
        self.consume(
            LeftBrace,
            format!("Expect '{{' before {} body.", kind).as_str(),
        )?;
        let body = self.block_statement()?;

        Ok(Stmt::FunStmt {
            name,
            params: parameters,
            body,
        })
    }

    /// statement      → exprStmt
    ///                | ifStmt ;
    ///                | printStmt ;
    ///                | block ;
    ///                | whileStmt
    fn statement(&mut self) -> Result<Stmt, Error> {
        // printStmt
        if matches!(self, Print) {
            return self.print_statement();
        }

        // block
        if matches!(self, LeftBrace) {
            return Ok(Stmt::BlockStmt {
                statements: self.block_statement()?,
            });
        }

        // ifStmt
        if matches!(self, If) {
            return self.if_statement();
        }

        // whileStmt
        if matches!(self, While) {
            return self.while_statement();
        }

        // forStmt
        if matches!(self, For) {
            return self.for_statement();
        }

        // returnStmt
        if matches!(self, Return) {
            return self.return_statement();
        }

        self.expression_statement()
    }

    /// exprStmt       → expression ";" ;
    fn expression_statement(&mut self) -> Result<Stmt, Error> {
        let expr = self.expression()?;
        self.consume(Semicolon, "Expect ';' after value.")?;

        Ok(Stmt::ExprStmt { expression: expr })
    }

    /// ifStmt         → "if" "(" expression ")" statement ( "else" statement )? ;
    fn if_statement(&mut self) -> Result<Stmt, Error> {
        self.consume(LeftParen, "Expect '(' after 'if'.")?;
        let condition_expr = self.expression()?;
        self.consume(RightParen, "Expect ')' after if condition.")?;
        let branch_stmt = self.statement()?;
        let mut else_stmt: Option<Box<Stmt>> = None;
        if matches!(self, Else) {
            else_stmt = Some(Box::new(self.statement()?));
        }

        Ok(Stmt::IfStmt {
            condition: condition_expr,
            then_branch: Box::new(branch_stmt),
            else_branch: else_stmt,
        })
    }

    /// printStmt      → "print" expression ";" ;
    fn print_statement(&mut self) -> Result<Stmt, Error> {
        let expr = self.expression()?;
        self.consume(Semicolon, "Expect ';' after value.")?;

        Ok(Stmt::PrintStmt { expression: expr })
    }

    /// block          | "{" declaration* "}" ;
    fn block_statement(&mut self) -> Result<Vec<Stmt>, Error> {
        let mut stmts: Vec<Stmt> = Vec::new();
        while !self.check(RightBrace) && !self.is_end() {
            let stmt = self.declaration()?;
            stmts.push(stmt);
        }

        self.consume(RightBrace, "Expect '}' after block")?;

        Ok(stmts)
    }

    /// whileStmt      | "while" "(" expression ")" statement ;
    fn while_statement(&mut self) -> Result<Stmt, Error> {
        self.consume(LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(RightParen, "Expect ')' after condition.")?;
        let body = self.statement()?;
        Ok(Stmt::WhileStmt {
            condition,
            body: Box::new(body),
        })
    }

    /// forStmt        | "for" "(" ( varDecl | exprStmt | ";" )
    ///                         expression? ";"
    ///                         expression? ")" statement ;
    fn for_statement(&mut self) -> Result<Stmt, Error> {
        // 语法脱糖, convert to while loop
        self.consume(LeftParen, "Expect '(' after 'for'.")?;

        let initializer: Option<Stmt> = if matches!(self, Semicolon) {
            None
        } else if matches!(self, Var) {
            Some(self.var_decl()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition: Expr = if self.check(Semicolon) {
            Expr::Literal {
                value: Literal::Boolean(true),
            }
        } else {
            self.expression()?
        };
        self.consume(Semicolon, "Expect ';' after loop condition.")?;

        let increment: Option<Expr> = if self.check(RightParen) {
            None
        } else {
            Some(self.expression()?)
        };
        self.consume(RightParen, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;

        if let Some(increment) = increment {
            body = Stmt::BlockStmt {
                statements: vec![
                    body,
                    Stmt::ExprStmt {
                        expression: increment,
                    },
                ],
            };
        }

        body = Stmt::WhileStmt {
            condition,
            body: Box::new(body),
        };

        if let Some(initializer) = initializer {
            body = Stmt::BlockStmt {
                statements: vec![initializer, body],
            };
        }

        Ok(body)
    }

    fn return_statement(&mut self) -> Result<Stmt, Error> {
        let keyword = self.previous().clone();
        let mut value: Option<Expr> = None;
        if !self.check(Semicolon) {
            value = Some(self.expression()?);
        }
        self.consume(Semicolon, "Expect ';' after return value.")?;

        Ok(Stmt::ReturnStmt { keyword, value })
    }

    // ------------------------------------------------
    // ------------------------------------------------
    // expression parser

    /// expression     → assignment ;
    fn expression(&mut self) -> Result<Expr, Error> {
        self.assignment()
    }

    /// assignment     → ( call "." )? IDENTIFIER "=" assignment
    ///                | logic_or ;
    fn assignment(&mut self) -> Result<Expr, Error> {
        let expr = self.logic_or();

        if matches!(self, Equal) {
            let value = self.assignment()?;
            if let Ok(Expr::Variable { name }) = expr {
                return Ok(Expr::Assign {
                    name,
                    value: Box::new(value),
                });
            } else if let Ok(Expr::Get { object, name }) = expr {
                return Ok(Expr::Set {
                    object,
                    name,
                    value: Box::new(value),
                });
            }
            return Err(self.error(self.previous(), "Invalid assignment target."));
        }

        expr
    }

    fn logic_or(&mut self) -> Result<Expr, Error> {
        let mut expr = self.logic_and()?;
        while matches!(self, Or) {
            let operator = self.previous().clone();
            let right = self.logic_and()?;
            let left = expr; // give expr to left
            expr = Expr::Logical {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn logic_and(&mut self) -> Result<Expr, Error> {
        let mut expr = self.equality()?;
        while matches!(self, And) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            let left = expr; // give expr to left
            expr = Expr::Logical {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
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
    ///                | call_index ;
    fn unary(&mut self) -> Result<Expr, Error> {
        if matches!(self, Bang, Minus) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }
        self.call_index()
    }
    /// call_index           → primary ( "(" arguments? ")" | "." IDENTIFIER | "[" index "]")* ;
    fn call_index(&mut self) -> Result<Expr, Error> {
        let mut expr = self.primary()?;
        while matches!(self, LeftParen, Dot, LeftBracket) {
            let previous_token_type = self.previous().token_type.clone();
            if previous_token_type == LeftParen {
                expr = self.finish_call(expr)?;
            } else if previous_token_type == Dot {
                let name = self.consume(Identifier, "Expect property name after '.'.")?;
                expr = Expr::Get {
                    object: Box::new(expr),
                    name: name.clone(),
                };
            } else if previous_token_type == LeftBracket {
                expr = self.finish_index(expr)?;
            }
        }
        Ok(expr)
    }

    /// primary        → NUMBER | STRING | "true" | "false" | "nil"
    ///                | "(" expression ")"
    ///                | IDENTIFIER
    ///                | this ;
    fn primary(&mut self) -> Result<Expr, Error> {
        if matches!(self, False) {
            return Ok(Expr::Literal {
                value: Literal::Boolean(false),
            });
        }
        if matches!(self, True) {
            return Ok(Expr::Literal {
                value: Literal::Boolean(true),
            });
        }

        if matches!(self, Nil) {
            return Ok(Expr::Literal {
                value: Literal::Nil,
            });
        }

        if matches!(self, String) {
            return Ok(Expr::Literal {
                // value: Literal::String(self.previous().lexeme.clone())
                value: Literal::String(
                    self.previous().lexeme[1..self.previous().lexeme.len() - 1].to_string(),
                ),
            });
        }
        if matches!(self, Number) {
            return Ok(Expr::Literal {
                value: Literal::Number(if let Ok(number) = self.previous().lexeme.parse::<i64>() {
                    NumberType::Integer(number)
                } else if let Ok(number) = self.previous().lexeme.parse::<f64>() {
                    NumberType::Float(number)
                } else {
                    return Err(self.error(self.previous(), "Invalid number."));
                }),
            });
        }

        if matches!(self, Identifier) {
            return Ok(Expr::Variable {
                name: self.previous().clone(),
            });
        }

        if matches!(self, LeftParen) {
            let expr = self.expression()?;

            self.consume(RightParen, "Expect ')' after expression.")?;

            return Ok(Expr::Grouping {
                expression: Box::new(expr),
            });
        }

        if matches!(self, This) {
            return Ok(Expr::This {
                keyword: self.previous().clone(),
            });
        }

        if matches!(self, Super) {
            let keyword = self.previous().clone();
            self.consume(Dot, "Expect '.' after 'super'.")?;
            let method = self
                .consume(Identifier, "Expect superclass method name.")?
                .clone();
            return Ok(Expr::Super { keyword, method });
        }
        Err(self.error(self.peak(), "Expect expression."))
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
        Err(self.error(self.peak(), message))
        // Err(Error {
        //     message: message.to_string(),
        //     error_type: ErrorType::SyntaxError,
        // })
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

    fn finish_index(&mut self, expr: Expr) -> Result<Expr, Error> {
        let index = self.expression()?;
        let index_end: Option<Box<Expr>> = if matches!(self, Colon) {
            Some(Box::new(self.expression()?))
        } else {
            None
        };
        self.consume(RightBracket, "Expect ']' after index.")?;
        Ok(Expr::Index {
            left: Box::new(expr),
            operator: self.previous().clone(),
            index: Box::new(index),
            index_end,
        })
    }

    fn finish_call(&mut self, expr: Expr) -> Result<Expr, Error> {
        let mut arguments: Vec<Expr> = Vec::new();
        if !self.check(RightParen) {
            loop {
                if arguments.len() >= 255 {
                    return Err(self.error(self.peak(), "Can't have more than 255 arguments."));
                }
                arguments.push(self.expression()?);
                if !matches!(self, Comma) {
                    break;
                }
            }
        }
        self.consume(RightParen, "Expect ')' after arguments.")?;
        Ok(Expr::Call {
            callee: Box::new(expr),
            paren: self.previous().clone(),
            arguments,
        })
    }
}
