use tracing_subscriber::{
    filter::LevelFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

/// Initialize logging based on the RUST_LOG environment variable.
pub fn init() {
    #[cfg(debug_assertions)]
    let formatter = fmt::Layer::new().without_time().with_line_number(true);

    #[cfg(not(debug_assertions))]
    let formatter = fmt::Layer::new();

    tracing_subscriber::registry()
        .with(formatter)
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::DEBUG.into())
                .from_env_lossy(),
        )
        .init();
}
