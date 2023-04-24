use lox::*;
use std::env;

fn main() {
    set_logger();

    let args: Vec<String> = env::args().collect();
    
    let mut lox = Loxer::new();

    match args.len() {
        1 => lox.run_prompt().unwrap(),
        2 => lox.run_file(&args[1]),
        _ => {
            eprintln!("Usage: lox [script]");
            std::process::exit(64);
        } 
    };
    // lox.run_prompt().unwrap();

    // lox.run_file(path).unwrap()
}
