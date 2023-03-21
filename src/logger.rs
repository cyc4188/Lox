
pub fn set_logger() {
    if let Err(_) = std::env::var("RUST_LOG") {
        std::env::set_var("RUST_LOG", "info");
    }
    // env_logger::init();
    pretty_env_logger::init(); 
    // log show
    log::trace!("test_trace");
    log::debug!("test_debug");
    log::info!("test_info");
    log::warn!("test_warn");
    log::error!("test_error");
}
