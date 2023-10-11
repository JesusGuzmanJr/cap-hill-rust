use actix_web::{get, Responder};
use askama::Template;
use const_format::formatcp;

use crate::{GITHUB_URL, MEETUP_URL, ORG_NAME};

#[derive(Template)]
#[template(path = "library.html")]
struct Library<'a> {
    title: &'a str,
    meetup_url: &'a str,
    github_url: &'a str,
}

#[get("/library")]
pub async fn handler() -> impl Responder {
    Library {
        title: formatcp!("{} - Library", ORG_NAME),
        meetup_url: MEETUP_URL,
        github_url: GITHUB_URL,
    }
}
