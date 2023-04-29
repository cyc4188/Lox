use crate::{token::{Token, TokenType}, Object};

#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub error_type: ErrorType,
}

impl Error {
    pub fn new(message: &str, error_type: ErrorType) -> Self {
        Self {
            message: message.to_string(),
            error_type,
        }
    }
}

pub fn report(line: usize, whr: &str,message: &str) {
    eprintln!("[line {}] Error{}: {}", line, whr,message);
}

pub fn parse_error(token: &Token, msg: &str) {
    if token.token_type == TokenType::Eof {
        report(token.line, " at end", msg);
    } else {
        report(token.line, &format!(" at '{}'", token.lexeme), msg);
    }
}

#[derive(Debug)]
pub enum ErrorType {
    ScanError(usize),
    SyntaxError,
    RuntimeError(Token),
    Return(Object)
}


