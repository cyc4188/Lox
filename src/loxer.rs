// use log::{info, debug};
// use crate::{scanner::Scanner};
// use crate::parser::Parser;
use super::*;
use std::fs;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

#[derive(PartialEq)]
pub enum MODE {
    PROMPT,
    FILE,
}

pub struct Loxer {
    had_error: bool, 
    interpreter: Interpreter,
}

impl Loxer {
    pub fn new() -> Self {
        Self {
            had_error: false,
            interpreter: Interpreter::new(),
        }
    }

    /// Execute the source code
    pub fn run (&mut self, source: &str, mode: MODE) {
        info!("Running source code: {}", source);
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();
        let tokens = &scanner.tokens;

        if scanner.had_error {
            scanner.report_errors();
            return;
        }

        let mut parser = Parser::new(tokens);
        let stmts = parser.parse();

        if let Ok(stmts) = stmts {
            info!("Parsed expression: {:?}", stmts);
            let mut resolver = Resolver::new(&mut self.interpreter);
            resolver.resolve_stmts(&stmts).unwrap();
            if resolver.has_error {
                std::process::exit(65);
            }
            let res: std::result::Result<(), Error> = self.interpreter.interpret(&stmts);
            if let Ok(()) = res {

            } else {
                let error = res.err().unwrap();
                if let ErrorType::RuntimeError(token) = error.error_type {
                    eprintln!("{}",error.message);
                    eprintln!("[line {}] Error at {}", token.line, token.lexeme);
                } else {
                    eprintln!("{}",error.message);
                }

                // Runtime error
                if mode == MODE::FILE {
                    std::process::exit(70);
                }
            }

        } else {
            // Parse error
            if mode == MODE::FILE {
                std::process::exit(65);
            }
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
                    self.run(line.as_str(), MODE::PROMPT);
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

    pub fn run_file(&mut self, path: &str) {
        info!("Running file: {}", path);
        let source = fs::read_to_string(path)
            .expect("Could not read file");
        self.run(source.as_str(), MODE::FILE);
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
        let mut loxer = Loxer::new();
        loxer.run("print 1+2*(3*4 - 6 / 2);", MODE::PROMPT);
    }
}
