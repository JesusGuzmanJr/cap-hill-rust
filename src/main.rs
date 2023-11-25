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
    std::{
        net::{Ipv4Addr, Ipv6Addr, SocketAddr},
        path::Path,
    },
};

mod catalog;
mod config;
mod health_check;
mod logger;

#[actix_web::main]
async fn main() -> Result<()> {
    let config = config::parse_from_env()?;
    logger::init(config.logging);

    let leptos_options = {
        LeptosOptions {
            env: if cfg!(debug_assertions) {
                leptos_config::Env::DEV
            } else {
                leptos_config::Env::PROD
            },
            site_root: config.assets,
            ..leptos_config::get_config_from_env()
                .expect("failed to get leptos config")
                .leptos_options
        }
    };

    let routes = leptos_actix::generate_route_list(cap_hill_rust::App);
    let leptos_app_data = web::Data::new(leptos_options.clone());

    let mut server = HttpServer::new(move || {
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
    });

    if let Some(tls) = config.tls {
        let http = [
            SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 80),
            SocketAddr::new(Ipv6Addr::UNSPECIFIED.into(), 80),
        ];
        let https = [
            SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 443),
            SocketAddr::new(Ipv6Addr::UNSPECIFIED.into(), 443),
        ];
        server = server
            .bind(http.as_ref())
            .context("couldn't bind to port 80")?
            .bind_rustls_021(https.as_ref(), tls)
            .context("couldn't bind to port 443")?
    } else {
        tracing::warn!("TLS is not configured");
    };

    if let Some(debug_listening_address) = config.debug_listening_address {
        server = server
            .bind(debug_listening_address)
            .with_context(|| format!("couldn't bind to {debug_listening_address}"))?;
    }

    if let Some(url) = config.health_check_ping_url {
        health_check::spawn_success_loop(url);
    } else {
        tracing::warn!("health check ping url is not configured");
    }

    server
        .shutdown_timeout(config.shutdown_timeout_seconds.get())
        .run()
        .await?;
    Ok(())
}
