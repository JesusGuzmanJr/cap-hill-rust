#![cfg(feature = "ssr")]

use {
    crate::Date,
    actix_web::{error::ErrorInternalServerError, get, HttpResponse},
    anyhow::{Context, Result},
    serde::Serialize,
    std::sync::OnceLock,
};

#[derive(Debug, Serialize)]
pub struct Book {
    title: String,
    authors: Vec<String>,
    publisher: String,
    published: Date,
    summary: String,
}
