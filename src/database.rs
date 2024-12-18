#[cfg(feature = "ssr")]
pub mod ssr {
    use std::time::Duration;

    // use http::{header::SET_COOKIE, HeaderMap, HeaderValue, StatusCode};
    use leptos::server_fn::ServerFnError;
    use sqlx::{postgres::PgPoolOptions, PgPool};

    // migrate!()
    //     .run(&pool)
    //     .await
    //     .expect("could not run SQLx migrations");

    pub async fn db() -> Result<PgPool, ServerFnError> {
        Ok(PgPoolOptions::new()
            .max_connections(10)
            .idle_timeout(Duration::from_secs(2))
            .connect("http://diabetes@localhost:5432/diabetes")
            .await?)
    }
}
