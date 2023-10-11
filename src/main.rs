use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use anyhow::{Context, Result};

use std::env;
use tracing_subscriber::{
    filter::LevelFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

mod catalog;
mod pages;

type DateTime = chrono::DateTime<chrono::Utc>;

const ORG_NAME: &str = "Cap Hill Rust";
const MEETUP_URL: &str = "https://www.meetup.com/Cap-Hill-Rust/";
const GITHUB_URL: &str = "https://github.com/JesusGuzmanJr/cap-hill-rust";

#[actix_web::main]
async fn main() -> Result<()> {
    init_logging();

    let bind_address = env::var("BIND_ADDRESS").with_context(|| "BIND_ADDRESS is not set")?;
    let assets_dir = env::var("ASSETS_DIR").with_context(|| "ASSETS_DIR is not set")?;
    HttpServer::new(move || {
        App::new()
            .route(
                "/health",
                web::get().to(|| async { HttpResponse::NoContent().finish() }),
            )
            .service(pages::index::handler)
            .service(pages::library::handler)
            .service(actix_files::Files::new("/assets", &assets_dir))
            .wrap(actix_web::middleware::Logger::new("%s for %r %a in %Ts").exclude("/health"))
            .wrap(middleware::Condition::new(
                cfg!(not(debug_assertions)),
                actix_web_lab::middleware::RedirectHttps::with_hsts(
                    actix_web_lab::header::StrictTransportSecurity::recommended(),
                ),
            ))
    })
    .bind(&bind_address)
    .with_context(|| format!("failed to bind to address: {}", bind_address))?
    .run()
    .await?;
    Ok(())
}

/// Initialize logging based on the RUST_LOG environment variable.
fn init_logging() {
    // show line numbers and hide timestamps in debug builds
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
