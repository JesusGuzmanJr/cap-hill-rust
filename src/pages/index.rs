use actix_web::{get, Responder};

use askama::Template;

use crate::{GITHUB_URL, MEETUP_URL, ORG_NAME};

#[derive(Template)]
#[template(path = "index.html")]
struct Index<'a> {
    title: &'a str,
    meetup_url: &'a str,
    github_url: &'a str,
}

#[get("/")]
async fn handler() -> impl Responder {
    Index {
        title: ORG_NAME,
        meetup_url: MEETUP_URL,
        github_url: GITHUB_URL,
    }
}
