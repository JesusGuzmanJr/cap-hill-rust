use actix_web::{get, App, HttpResponse, HttpServer};
use anyhow::{Context, Result};
use askama::Template;
use std::env;
use tracing_subscriber::{
    filter::LevelFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

mod error;

const ORG_NAME: &str = "Cap Hill Rust";
const MEETUP_URL: &str = "https://www.meetup.com/Cap-Hill-Rust/";
const GITHUB_URL: &str = "https://github.com/JesusGuzmanJr/cap-hill-rust";

type Response = Result<actix_web::HttpResponse, error::Error>;

#[derive(Template)]
#[template(path = "index.html")]
struct Index<'a> {
    org_name: &'a str,
    meetup_url: &'a str,
    github_url: &'a str,
}

#[get("/")]
async fn index() -> Response {
    let index = Index {
        org_name: ORG_NAME,
        meetup_url: MEETUP_URL,
        github_url: GITHUB_URL,
    };

    Ok(HttpResponse::Ok().body(index.render()?))
}

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::NoContent().finish()
}

#[actix_web::main]
async fn main() -> Result<()> {
    init_logging();

    let bind_address = env::var("BIND_ADDRESS")?;
    HttpServer::new(|| App::new().service(health).service(index))
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

    // show line numbers and hide timestamps in debug builds
    #[cfg(not(debug_assertions))]
    let formatter = fmt::Layer::new();

    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::DEBUG.into())
        .from_env_lossy();

    tracing_subscriber::registry()
        .with(formatter)
        .with(env_filter)
        .init();
}
