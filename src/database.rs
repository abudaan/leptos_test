use std::time::Duration;

use leptos::{server, ServerFnError};
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::{postgres::PgPoolOptions, Error, PgPool};

#[cfg(feature = "ssr")]
#[derive(Clone)]
pub struct AppState {
    pub db: Option<PgPool>,
    pub error: Option<String>,
}

// impl AppState {
//     pub
// }

#[cfg(feature = "ssr")]
pub async fn init_database() -> Result<PgPool, Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .idle_timeout(Duration::from_secs(2))
        .connect("http://diabetes@localhost:5432/diabetes")
        .await
}

#[cfg(feature = "ssr")]
pub async fn get_app_state() -> AppState {
    let result = init_database().await;
    match result {
        Ok(value) => AppState {
            db: Some(value),
            error: None,
        },
        Err(error) => AppState {
            db: None,
            error: Some(error.to_string()),
        },
    }
}
