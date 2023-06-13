#[cfg(not(debug_assertions))]
pub fn init_logger() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .without_time()
        .with_thread_ids(true)
        .init()
}

#[cfg(debug_assertions)]
pub fn init_logger() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_thread_ids(true)
        .init()
}
