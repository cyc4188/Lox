use crate::token::*;

#[derive(Debug)]
pub struct Scanner {
    source: String, // source code
    start: usize,   // start of current token
    current: usize, // current position in source code
    line: usize,    // current line
    pub tokens: Vec<Token>,
    pub had_error: bool,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            start: 0,
            current: 0,
            line: 1,
            tokens: Vec::new(),
            had_error: false,
        }
    }

    /// return a vector of tokens
    pub fn scan_tokens(&mut self) {
        // loop until we reach the end of the source code
        while !self.is_end() {
            self.start = self.current;
            if let Some(token) = self.scan_token() {
                self.tokens.push(token);
            }
            else {
                self.had_error = true;
            }
        } 
    }


    /// return a token, this is where the magic happens
    pub fn scan_token(&mut self) -> Option<Token> {
        let c = self.consume();

        // check if the character is a single character token
        if let Some(token_type) = Token::check_single_character_token(c) {
            return Some( self.get_token(token_type, Literal::Nil));
        }

        // deal with operators
        if let Some(token_type) = Token::check_operator(c, self.peak()) {
            match token_type {
                TokenType::BangEqual | TokenType::EqualEqual | TokenType::GreaterEqual | TokenType::LessEqual => {
                    self.consume();
                }
                _ => {}
            }
            return Some(self.get_token(token_type, Literal::Nil));
        } 

        return None;
    }

    /// return a token, according to token_type and literal
    pub fn get_token(&self, token_type: TokenType, literal: Literal) -> Token {
        log::debug!("{}", &self.source[self.start..self.current]);
        Token::new(&self.source[self.start..self.current], token_type, literal, self.line)
    }
    
    /// return true if we have reached the end of the source code
    pub fn is_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// return the next character without advancing the current position
    pub fn peak(&self) -> char {
        if self.is_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }
    
    /// return the next character and advance the current position
    pub fn consume(&mut self) -> char {
        let c = self.peak();
        self.current += 1;
        c
    }


    /// return true if the next character is expected
    /// if true, advance the current position
    /// if false, do nothing
    pub fn mat(&mut self, expected: char) -> bool {
        if self.is_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_single_character_token() {
        let mut scanner = Scanner::new("(){},.");
        scanner.scan_tokens();
        assert_eq!(scanner.tokens.len(), 6);

    }
    #[test]
    fn test_scan_operator() {
        let mut scanner = Scanner::new("== != > >= < <=");
        scanner.scan_tokens();
        assert_eq!(scanner.tokens.len(), 6);
        println!("{:?}", scanner.tokens);
    }
}

