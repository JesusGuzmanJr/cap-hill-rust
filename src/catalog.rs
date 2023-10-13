use crate::Date;
use actix_web::{error::ErrorInternalServerError, get, HttpResponse, Responder};
use anyhow::{Context, Result};
use serde::Serialize;
use sqlx::{Pool, Postgres};
use std::sync::OnceLock;

#[derive(Debug, Serialize)]
pub struct Book {
    title: String,
    authors: Vec<String>,
    publisher: String,
    published: Date,
    summary: String,
}

static DB: OnceLock<Pool<Postgres>> = OnceLock::new();

fn db() -> &'static Pool<Postgres> {
    DB.get().expect("not initialized")
}

/// Initialize the database connection pool and run migrations.
pub async fn init() -> Result<()> {
    let database_url = std::env::var("DATABASE_URL").with_context(|| "DATABASE_URL is not set")?;

    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect(&database_url)
        .await
        .context("unable to create postgres connection pool")?;

    tracing::info!("running migrations");
    sqlx::migrate!()
        .run(&pool)
        .await
        .context("failed to run migrations")?;

    DB.set(pool).expect("already initialized");
    Ok(())
}

#[get("/catalog")]
pub async fn get_catalog() -> impl Responder {
    let catalog = sqlx::query!(
        "
        SELECT book.id AS book_id, title, published, publisher.name AS publisher, summary, json_agg(author.name) AS authors
        FROM book
        LEFT JOIN book_author
        ON book.id = book_author.book_id
        LEFT JOIN author
        ON book_author.author_id = author.id
        LEFT JOIN publisher
        ON book.publisher_id = publisher.id
        GROUP BY book.id, publisher.name"
    )
    .fetch_all(db())
    .await
    .map_err(|error| {
        tracing::error!(?error, "unable to fetch catalog");
        ErrorInternalServerError("unable to fetch catalog")
    })?
    .into_iter()
    .map(|record| {
        let authors = match record.authors {
            None => {
                tracing::error!(title = record.title, book_id = ?record.book_id, "no authors for book");
                vec![]
            }
            Some(authors) => match  serde_json::from_value::<Vec<String>>(authors) {
                Ok(authors) => authors,
                Err(error) => {
                    tracing::error!(title = record.title, book_id = ?record.book_id, ?error, "unable to parse authors for book");
                    vec![]
                }
            }
        };

        Book {
            title: record.title,
            authors,
            publisher: record.publisher,
            published: record.published.into(),
            summary: record.summary,
        }
    })
    .collect::<Vec<Book>>();

    Ok::<_, actix_web::error::Error>(HttpResponse::Ok().json(catalog))
}
