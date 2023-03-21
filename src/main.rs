use lox::*;

fn main() {
    set_logger();
    let mut lox = Loxer::new();
    lox.run_prompt().unwrap();
}
