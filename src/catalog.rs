use crate::DateTime;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use sqlx::{types::Json, Pool, Postgres};
use std::sync::OnceLock;

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    CheckedOut { until: DateTime },
    OnHold { until: DateTime },
    Available,
}

pub struct Entry {
    title: String,
    authors: Vec<String>,
    publisher: String,
    published: DateTime,
    status: Json<Status>,
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
