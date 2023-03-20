use log::{info, debug};
use crate::{scanner::Scanner};
use crate::parser::Parser;
use std::{fs, io};

pub struct Loxer {
    had_error: bool, 
}

impl Loxer {
    pub fn new() -> Self {
        Self {had_error: false}
    }

    /// Execute the source code
    pub fn run (&self, source: &str) {
        info!("Running source code: {}", source);
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();
        let tokens = &scanner.tokens;

        if scanner.had_error {
            for error in &scanner.errors {
                println!("[line {}] Error: {}", error.line, error.message);
            }
            return;
        }

        let mut parser = Parser::new(tokens);
        
        let res = parser.parse();

        if let Ok(expr) = res {
            debug!("Parsed expression: {}", expr);
        } else {
            debug!("Error parsing expression: {:?}", res);
        }
         
    }

    // Run in the command line
    pub fn run_prompt(&mut self) {
        log::info!("Running in prompt mode");
        loop {
            print!("> ");
            io::Write::flush(&mut io::stdout()).unwrap();

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(0) => {
                    log::info!("EOF");
                    break;
                }
                Ok(_) => {
                    log::debug!("Read line: {}", input);
                }
                Err(error) => {
                    eprintln!("Error reading line: {}", error);
                    continue;
                }
            }
            self.run(input.as_str());
            self.had_error = false; // Reset error flag
        }
    }

    pub fn run_file(&self, path: &str) {
        let source = fs::read_to_string(path)
            .expect("Could not read file");
        self.run(source.as_str());
    }
}




#[cfg(test)]
mod test {
    use crate::set_logger;

    use super::*;

    #[test]
    fn test_run() {
        set_logger();
        info!("Running test_run())");
        let loxer = Loxer::new();
        loxer.run("1+2*(3*4 - 6 / 2)");
    }
}