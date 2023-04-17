// use log::{info, debug};
// use crate::{scanner::Scanner};
// use crate::parser::Parser;
use super::*;
use std::fs;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

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
            scanner.report_errors();
            return;
        }

        let mut parser = Parser::new(tokens);
        let res = parser.parse();

        if let Ok(expr) = res {
            info!("Parsed expression: {}", expr);
            let mut interpreter = Interpreter::new();
            
            let res = interpreter.interpret(&expr);
            if let Ok(value) = res {
                println!("{}", value);
            } else {
                let error = res.err().unwrap();
                log::error!("{}", error.message);
            }

        } else {
            let error = res.err().unwrap();
            log::error!("Error parsing expression: {:?}", error);
        }
         
    }

    // Run in the command line
    pub fn run_prompt(&mut self) -> Result<()>{
        log::info!("Running in prompt mode");

        // `()` can be used when no completer is required
        let mut rl = DefaultEditor::new()?;
        #[cfg(feature = "with-file-history")]
        if rl.load_history("history.txt").is_err() {
            println!("No previous history.");
        }

        loop {
            let readline = rl.readline(">> ");

            match readline {
                Ok(line) => {
                    log::debug!("Read line: {}", line);
                    if line.is_empty() {
                        continue;
                    }
                    self.run(line.as_str());
                    self.had_error = false; // Reset error flag
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break
                },
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break
                },
                Err(err) => {
                    println!("Error: {:?}", err);
                    break
                }
            }
        }
        Ok(())
    }

    pub fn run_file(&self, path: &str) {
        let source = fs::read_to_string(path)
            .expect("Could not read file");
        self.run(source.as_str());
    }
}

impl Default for Loxer {
    fn default() -> Self {
        Self::new()
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
