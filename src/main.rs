use lox::*;

fn set_logger() {
    if let Err(_) = std::env::var("RUST_LOG") {
        std::env::set_var("RUST_LOG", "debug");
    }
    // env_logger::init();
    pretty_env_logger::init(); 
    // log show
    log::debug!("test_debug");
    log::info!("test_info");
    log::warn!("test_warn");
    log::error!("test_error");

}

fn main() {
    set_logger();
    let mut lox = Loxer::new();
    lox.run_prompt();
}
