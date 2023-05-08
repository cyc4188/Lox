
pub fn set_logger() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug");
    }
    // env_logger::init();
    pretty_env_logger::init(); 
    // log show
    // log::trace!("test_trace");
    // log::debug!("test_debug");
    // log::info!("test_info");
    // log::warn!("test_warn");
    // log::error!("test_error");
}
