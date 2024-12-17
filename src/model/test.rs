#[cfg(feature = "ssr")]
use crate::database::ssr::db;
use leptos::{logging, prelude::ServerFnError, server};

// #[server(Json)]
#[server(name = Test,
    prefix = "/api",
    endpoint = "test")]
pub async fn test(value: String) -> Result<String, ServerFnError> {
    // let pool = get_pool().await?;
    // let pool = db().await?;
    if value.is_empty() {
        // logging::log!("TEST ERROR");
        Err(ServerFnError::ServerError("TEST ERROR".into()))
    } else {
        Ok("everything is O.K.".to_string())
    }
}
