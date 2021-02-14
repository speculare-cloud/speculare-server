/// Init the logger (env_logger) and define the debug level
/// based on debug or release build.
pub fn configure() {
    // Check if the RUST_LOG already exist in the sys
    if std::env::var_os("RUST_LOG").is_none() {
        // if it doesn't, assign a default value to RUST_LOG
        // Define RUST_LOG as trace for debug and error for prod
        std::env::set_var(
            "RUST_LOG",
            if cfg!(debug_assertions) {
                "info,actix_server=info,actix_web=info"
            } else {
                "error,actix_server=error,actix_web=error"
            },
        );
    }
    // Init the logger
    env_logger::init();
}
