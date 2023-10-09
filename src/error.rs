/// Error type for this crate.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Askama template error.
    #[error(transparent)]
    TemplateError(#[from] askama::Error),
}

impl actix_web::ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Error::TemplateError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponseBuilder::new(self.status_code()).body(self.to_string())
    }
}
