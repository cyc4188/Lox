use std::fmt::Display;

#[derive(Debug)]
pub struct Token {
    lexeme: String,
    token_type: TokenType,
    literal: Literal,
    line: usize,
}

impl Token {
    pub fn new(lexeme: &str, token_type: TokenType, literal: Literal, line: usize) -> Self {
        Self {
            lexeme: lexeme.to_string(),
            token_type,
            literal,
            line,
        }
    }
    pub fn check_single_character_token(ch: char) -> Option<TokenType> {
        match ch {
            '(' => Some(TokenType::LeftParen),
            ')' => Some(TokenType::RightParen),
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
        write!(f, "{} {} {} ", self.token_type, self.lexeme, self.literal)
    }
}

#[derive(Display, Debug)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or two character tokens.
    Bang, BangEqual, Equal, EqualEqual,
    Greater, GreaterEqual, Less, LessEqual,

    // Literals.
    Identifier, String, Number,

    // Keywords.
    And, Class, Else, False, Fun,
    For, If, Nil, Or, Print, Return,
    Super, This, True, Var, While,

    Eof,
}

#[derive(Display, Debug)]
pub enum Literal {
    String(String),
    Identifier(String),
    Number(f64),
    Boolean(bool),
    Nil,
}
