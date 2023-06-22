use std::fmt::Display;

use crate::NumberType;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    pub lexeme: String,
    pub token_type: TokenType,
    // pub literal: Literal,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(lexeme: &str, token_type: TokenType, line: usize, column: usize) -> Self {
        Self {
            lexeme: lexeme.to_string(),
            token_type,
            line,
            column,
        }
    }
    pub fn check_single_character_token(ch: char) -> Option<TokenType> {
        match ch {
            '(' => Some(TokenType::LeftParen),
            ')' => Some(TokenType::RightParen),
            '[' => Some(TokenType::LeftBracket),
            ']' => Some(TokenType::RightBracket),
            '{' => Some(TokenType::LeftBrace),
            '}' => Some(TokenType::RightBrace),
            ',' => Some(TokenType::Comma),
            '.' => Some(TokenType::Dot),
            '-' => Some(TokenType::Minus),
            '+' => Some(TokenType::Plus),
            ';' => Some(TokenType::Semicolon),
            '*' => Some(TokenType::Star),
            _ => None,
        }
    }

    pub fn check_operator(ch: char, next_ch: char) -> Option<TokenType> {
        match (ch, next_ch) {
            ('!', '=') => Some(TokenType::BangEqual),
            ('=', '=') => Some(TokenType::EqualEqual),
            ('>', '=') => Some(TokenType::GreaterEqual),
            ('<', '=') => Some(TokenType::LessEqual),
            ('!', _) => Some(TokenType::Bang),
            ('=', _) => Some(TokenType::Equal),
            ('>', _) => Some(TokenType::Greater),
            ('<', _) => Some(TokenType::Less),
            _ => None,
        }
    }
    pub fn check_keyword(text: &str) -> Option<TokenType> {
        match text {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "for" => Some(TokenType::For),
            "fun" => Some(TokenType::Fun),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),
            _ => None,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.lexeme)
        // write!(f, "{} {} {} ", self.token_type, self.lexeme, self.literal)
    }
}

#[derive(Display, Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Clone, Debug)]
pub enum Literal {
    String(String),
    Number(NumberType),
    Boolean(bool),
    Nil,
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::String(s) => write!(f, "\"{}\"", s),
            Literal::Number(n) => write!(f, "{}", n),
            Literal::Boolean(b) => write!(f, "{}", b),
            Literal::Nil => write!(f, "nil"),
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::NumberType;

    #[test]
    fn test_literal() {
        let literal = super::Literal::String("hello".to_string());
        let num = super::Literal::Number(NumberType::Float(1.0));
        println!("literal: {}", literal);
        println!("num: {}", num)
    }
}
