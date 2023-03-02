use crate::token::*;

#[derive(Debug)]
pub struct Scanner {
    source: String, // source code
    start: usize,   // start of current token
    current: usize, // current position in source code
    line: usize,    // current line
    pub tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            start: 0,
            current: 0,
            line: 1,
            tokens: Vec::new(),
        }
    }

    /// return a vector of tokens
    pub fn scan_tokens(&mut self) {
        // loop until we reach the end of the source code
        while !self.is_end() {
            self.start = self.current;
            let token = self.scan_token().unwrap();
            self.tokens.push(token);
        } 
    }


    /// return a token, this is where the magic happens
    pub fn scan_token(&mut self) -> Option<Token> {
        let c = self.consume();

        // check if the character is a single character token
        if let Some(token_type) = Token::is_single_char_token(c) {
            return Some( self.get_token(token_type, Literal::Nil));
        }
        return None;
    }

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_tokens() {
        let mut scanner = Scanner::new("(){},.");
        scanner.scan_tokens();
        assert_eq!(scanner.tokens.len(), 6);
    }
}

