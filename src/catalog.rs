#![cfg(feature = "ssr")]

use {chrono::NaiveDate, serde::Serialize};

#[derive(Debug, Serialize)]
pub struct Book {
    title: String,
    authors: Vec<String>,
    publisher: String,
    published: NaiveDate,
    summary: String,
}
