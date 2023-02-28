use std::fmt::Display;

#[derive(Debug)]
pub struct Scanner {
    source: String,

}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string()
        }
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        self.source.split_whitespace().map(|c| Token::new(
                &c.to_string(),
                TokenType::Identifier, 
                Literal::Nil,
                0
                ))
            .collect() 
    }
}

pub struct Token {
    lexeme: String,
    token_type: TokenType,
    literal: Literal,
    line: usize, 
}

impl Token {
    pub fn new(lexeme: &str, token_type: TokenType, literal : Literal, line: usize) -> Self {
        Self {
            lexeme: lexeme.to_string(),
            token_type,
            literal,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} ", self.token_type, self.lexeme, self.literal)
    }
}


#[derive(Display)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals.
    Identifier, String, Number,

    // Keywords.
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    Eof,
}

#[derive(Display)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}
