use std::time::Duration;

use leptos::{server, ServerFnError};
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::{postgres::PgPoolOptions, PgPool};

#[derive(Clone)]
pub struct AppState {
    pub db: Option<PgPool>,
}

pub async fn init_database() {
    let db: sqlx::Pool<sqlx::Postgres> = PgPoolOptions::new()
        .max_connections(10)
        .idle_timeout(Duration::from_secs(2))
        .connect("http://diabetes@localhost:5432/diabetes")
        .await
        .unwrap();
}

pub async fn get_app_state() -> AppState {
    init_database().await;
    AppState { db: None }
}
