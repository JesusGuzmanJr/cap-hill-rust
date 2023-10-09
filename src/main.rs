use actix_web::{get, App, HttpResponse, HttpServer};
use anyhow::{Context, Result};
use askama::Template;
use std::env;

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

#[actix_web::main]
async fn main() -> Result<()> {
    let bind_address = env::var("BIND_ADDRESS")?;
    HttpServer::new(|| App::new().service(index))
        .bind(&bind_address)
        .with_context(|| format!("failed to bind to address: {}", bind_address))?
        .run()
        .await?;
    Ok(())
}
