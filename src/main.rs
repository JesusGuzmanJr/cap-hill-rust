use actix_files::NamedFile;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{
    http::StatusCode,
    middleware::{self, ErrorHandlers},
    web, App, HttpResponse, HttpServer,
};
use anyhow::{Context, Result};
use std::{
    env,
    path::{Path, PathBuf},
    sync::OnceLock,
};

mod catalog;
mod logging;
mod pages;

type Date = chrono::NaiveDate;

const ORG_NAME: &str = "Cap Hill Rust";
const MEETUP_URL: &str = "https://www.meetup.com/Cap-Hill-Rust/";
const GITHUB_URL: &str = "https://github.com/JesusGuzmanJr/cap-hill-rust";

static ASSETS: OnceLock<PathBuf> = OnceLock::new();

#[inline]
fn assets() -> &'static Path {
    ASSETS.get().expect("not initialized")
}

#[actix_web::main]
async fn main() -> Result<()> {
    logging::init();
    catalog::init().await?;

    let bind_address = env::var("BIND_ADDRESS").with_context(|| "BIND_ADDRESS is not set")?;

    ASSETS
        .set(Path::new(&env::var("ASSETS_DIR").with_context(|| "ASSETS_DIR is not set")?).into())
        .expect("already initialized");

    HttpServer::new(move || {
        const FAVICON: &str = "favicon.ico";
        const ROBOTS: &str = "robots.txt";

        App::new()
            .route(
                "/health",
                web::get().to(|| async { HttpResponse::NoContent().finish() }),
            )
            .service(
                web::scope("")
                    .wrap(
                        ErrorHandlers::new()
                            .handler(StatusCode::NOT_FOUND, pages::not_found::handler),
                    )
                    .route(
                        FAVICON,
                        web::get().to(move || async {
                            NamedFile::open_async(assets().join("favicons").join("favicon.ico"))
                                .await
                        }),
                    )
                    .route(
                        ROBOTS,
                        web::get().to(move || async {
                            NamedFile::open_async(assets().join(ROBOTS)).await
                        }),
                    )
                    .service(pages::index::handler)
                    .service(pages::library::handler)
                    .service(catalog::get_catalog)
                    .service(actix_files::Files::new("/assets", &assets()))
                    .wrap(actix_web::middleware::Logger::new("%s for %r %a in %Ts"))
                    .wrap(middleware::Condition::new(
                        cfg!(not(debug_assertions)),
                        actix_web_lab::middleware::RedirectHttps::with_hsts(
                            actix_web_lab::header::StrictTransportSecurity::recommended(),
                        ),
                    )),
            )
            .wrap(Governor::new(
                &GovernorConfigBuilder::default()
                    .finish()
                    .expect("invalid rate limiter config"),
            ))
            .wrap(middleware::Compress::default())
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::Trim,
            ))
    })
    .bind(&bind_address)
    .with_context(|| format!("failed to bind to address: {}", bind_address))?
    .run()
    .await?;
    Ok(())
}
