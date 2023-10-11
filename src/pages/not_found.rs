use actix_web::{dev::ServiceResponse, middleware::ErrorHandlerResponse, Result};

use askama::Template;

use crate::{GITHUB_URL, MEETUP_URL, ORG_NAME};

#[derive(Template)]
#[template(path = "not_found.html")]
struct NotFound<'a> {
    title: &'a str,
    meetup_url: &'a str,
    github_url: &'a str,
}

pub fn handler<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let not_found = NotFound {
        title: ORG_NAME,
        meetup_url: MEETUP_URL,
        github_url: GITHUB_URL,
    };

    let (req, res) = res.into_parts();

    let res = res.set_body(
        not_found
            .render()
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?,
    );

    let res = ServiceResponse::new(req, res)
        .map_into_boxed_body()
        .map_into_right_body();

    Ok(ErrorHandlerResponse::Response(res))
}
