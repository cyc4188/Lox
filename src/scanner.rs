use super::*;

#[derive(Debug)]
pub struct Scanner {
    source: String, // source code
    start: usize,   // start of current token
    current: usize, // current position in source code
    line: usize,    // current line
    pub tokens: Vec<Token>,
    pub had_error: bool,
    pub errors: Vec<ScanError>,
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
            errors: Vec::new(),
        }
    }

    pub fn scan_tokens(&mut self) {
        // loop until we reach the end of the source code
        while !self.is_end() {
            self.start = self.current;
            self.scan_token();
        } 
    }


    /// return a token, this is where the magic happens
    fn scan_token(&mut self)  {
        let c = self.consume();

        // check if the character is a single character token
        if let Some(token_type) = Token::check_single_character_token(c) {
            self.add_token(token_type, Literal::Nil);
            return;
        }

        // deal with operators
        if let Some(token_type) = Token::check_operator(c, self.peak()) {
            match token_type {
                TokenType::BangEqual | TokenType::EqualEqual | TokenType::GreaterEqual | TokenType::LessEqual => {
                    self.consume();
                }
                _ => {}
            }
            self.add_token(token_type, Literal::Nil);
            return;
        } 

        // longer tokens
        // /, //, \r, \t, ' ', \n
        // 'string'
        match c {
            '/' => { 
                if self.mat('/') {
                    // a comment goes until the end of the line
                    while self.peak() != '\n' && !self.is_end() {
                        self.consume();
                    }
                } else {
                    self.add_token(TokenType::Slash, Literal::Nil);
                }
            }
            ' ' | '\r' | '\t' => {
                return;
            }
            '\n' => {
                self.line += 1;
            }
            '"' => { // String
                self.check_string();
            }
            '0'..='9' => { // Number
                self.check_number();
            }
            'a'..='z' | 'A'..='Z' | '_' => { // Identifier or Keyword
                self.check_identifier();
            }
            _ => {
                self.error(self.line, "Unexpected character.");
            }
        };

    }

    /// return a token, according to token_type and literal
    fn get_token(&self, token_type: TokenType, literal: Literal) -> Token {
        log::debug!("{}", &self.source[self.start..self.current]);
        Token::new(&self.source[self.start..self.current], token_type, literal, self.line)
    }

    /// add a token to the tokens vector
    fn add_token(&mut self, token_type: TokenType, literal: Literal) {
        self.tokens.push(self.get_token(token_type, literal));
    }
    
    /// return true if we have reached the end of the source code
    fn is_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// return the current character without advancing the current position
    fn peak(&self) -> char {
        if self.is_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }

    /// return the next next character without advancing the current position
    fn peak_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }
    
    /// return the current character and advance the current position
    fn consume(&mut self) -> char {
        let c = self.peak();
        self.current += 1;
        c
    }


    /// return true if the next character is expected
    /// if true, advance the current position
    /// if false, do nothing
    fn mat(&mut self, expected: char) -> bool {
        if self.is_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn check_string(&mut self) {
        while self.peak() != '"' && !self.is_end() {
            if self.peak() == '\n' {
                self.line += 1;
            }
            self.consume();
        }

        if self.is_end() {
            self.error(self.line, "Unterminated string.");
            return;
        }

        // the closing "
        self.consume();

        // trim the surrounding quotes
        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token(TokenType::String, Literal::String(value.to_string()));
    }

    fn check_number(&mut self) {
        while is_digit(self.peak()) {
            self.consume();
        }

        // look for a fractional part
        if self.peak() == '.' && is_digit(self.peak_next()){
            // consume the "."
            self.consume();

            while is_digit(self.peak()) {
                self.consume();
            }
        }

        let value = &self.source[self.start..self.current];
        self.add_token(TokenType::Number, Literal::Number(value.parse().unwrap()));
    }

    fn check_identifier(&mut self) {
        while is_alpha_numeric(self.peak()) {
            self.consume();
        }

        let text = &self.source[self.start..self.current];
        // let token_type = Token::check_keyword(text).unwrap_or(TokenType::Identifier);
        if let Some(token_type) = Token::check_keyword(text) {
            // keyword
            let literal = match token_type {
                TokenType::True => Literal::Boolean(true),
                TokenType::False => Literal::Boolean(false),
                TokenType::Nil => Literal::Nil,
                _ => Literal::Nil,
            };
            self.add_token(token_type, literal);
        }
        else {
            //identifier
            self.add_token(TokenType::Identifier, Literal::Nil);
        }
    }


    fn error(&mut self, line: usize, message: &str) {
        self.errors.push(ScanError::new(line, message));
        self.had_error = true;
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
        for token in scanner.tokens.iter() {
            println!("{:?}", token);
        } 
    }

    #[test]
    fn test_longer_tokens() {
        let mut scanner = Scanner::new("// this is a comment
(( )){} // grouping stuff
!*+-/=<> <= == // operators");
        scanner.scan_tokens();
        for token in scanner.tokens.iter() {
            println!("{:?}", token);
        } 
    }

    #[test]
    fn test_string() {
        let mut scanner = Scanner::new("\"this is a string\"");
        scanner.scan_tokens();
        for token in scanner.tokens.iter() {
            println!("{:?}", token);
        } 
    }
    #[test]
    fn test_number() {
        let mut scanner = Scanner::new("123 123.456");
        scanner.scan_tokens();
        for token in scanner.tokens.iter() {
            println!("{:?}", token);
        } 
    }
    #[test]
    fn test_keyword() {
        let mut scanner = Scanner::new("and class else false fun for if nil or print return super this true var while");
        scanner.scan_tokens();
        for token in scanner.tokens.iter() {
            println!("{:?}", token);
        } 
    }
    #[test]
    fn test_identifier() {
        let mut scanner = Scanner::new("a+b");
        scanner.scan_tokens();
        for token in scanner.tokens.iter() {
            println!("{:?}", token);
        } 
    }
}

