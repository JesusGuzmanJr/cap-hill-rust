// no main needed for non-ssr because
// wasm is loaded using `hydrate()` from lib
#![cfg_attr(not(feature = "ssr"), no_main)]
#![cfg(feature = "ssr")]

use {
    actix_files::NamedFile,
    actix_governor::{Governor, GovernorConfigBuilder},
    actix_web::{middleware, web, App, HttpResponse, HttpServer},
    anyhow::{Context, Result},
    leptos::*,
    leptos_actix::LeptosRoutes,
    std::{env, path::Path},
};

mod catalog;
mod logging;

type Date = chrono::NaiveDate;

#[actix_web::main]
async fn main() -> Result<()> {
    logging::init();
    catalog::init().await?;

    let bind_address = env::var("BIND_ADDRESS").with_context(|| "BIND_ADDRESS is not set")?;

    let leptos_options = {
        LeptosOptions {
            env: if cfg!(debug_assertions) {
                leptos_config::Env::DEV
            } else {
                leptos_config::Env::PROD
            },
            site_root: env::var("ASSETS").with_context(|| "ASSETS is not set")?,
            ..leptos_config::get_config_from_env()
                .expect("failed to get leptos config")
                .leptos_options
        }
    };

    let routes = leptos_actix::generate_route_list(cap_hill_rust::App);
    let leptos_app_data = web::Data::new(leptos_options.clone());

    HttpServer::new(move || {
        const FAVICON: &str = "favicon.ico";
        const ROBOTS: &str = "robots.txt";

        App::new()
            .route(
                "/health",
                web::get().to(|| async { HttpResponse::NoContent().finish() }),
            )
            .route(
                FAVICON,
                web::get().to(|data: web::Data<LeptosOptions>| async move {
                    NamedFile::open_async(
                        Path::new(&data.get_ref().site_root)
                            .join("favicons")
                            .join(FAVICON),
                    )
                    .await
                }),
            )
            .route(
                ROBOTS,
                web::get().to(|data: web::Data<LeptosOptions>| async move {
                    NamedFile::open_async(Path::new(&data.get_ref().site_root).join(ROBOTS)).await
                }),
            )
            .leptos_routes(
                leptos_options.to_owned(),
                routes.to_owned(),
                cap_hill_rust::App,
            )
            .service(actix_files::Files::new("/", &leptos_options.site_root).use_hidden_files())
            .app_data(leptos_app_data.clone())
            .service(actix_files::Files::new(
                "/assets",
                leptos_options.site_root.clone(),
            ))
            .wrap(actix_web::middleware::Logger::new("%s for %r %a in %Ts"))
            .wrap(middleware::Condition::new(
                cfg!(not(debug_assertions)),
                actix_web_lab::middleware::RedirectHttps::with_hsts(
                    actix_web_lab::header::StrictTransportSecurity::recommended(),
                ),
            ))
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
