//! Logging module.
#![cfg(feature = "ssr")]

use {
    crate::config::LoggingConfig,
    anyhow::Context,
    std::time::Duration,
    tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter},
};

/// Initializes the logger and sentry.
#[deny(dead_code)]
pub fn init(config: LoggingConfig) {
    // show line numbers and hide timestamps in debug builds
    #[cfg(debug_assertions)]
    let formatter = fmt::Layer::new().without_time().with_line_number(true);

    #[cfg(not(debug_assertions))]
    let formatter = fmt::Layer::new();

    let log = tracing_subscriber::registry()
        .with(formatter)
        .with(config.directives)
        .with(sentry_tracing::layer());

    if let Ok(layer) = tracing_journald::Layer::new() {
        log.with(layer).init();
    } else {
        log.init();
        tracing::warn!("logging to journald is not available");
    }

    let guard = sentry::init(sentry::ClientOptions {
        dsn: config.sentry_data_source_name,
        ..Default::default()
    });

    if !guard.is_enabled() {
        tracing::warn!("sentry is not configured");
    }

    // keep the guard for the lifetime of the program
    std::mem::forget(guard);
}
