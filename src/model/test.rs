use leptos::{logging, prelude::ServerFnError, server};

#[server(name = Test,
    prefix = "/api",
    endpoint = "test"
)]
pub async fn test(value: String) -> Result<String, ServerFnError> {
    if value.is_empty() {
        // logging::log!("TEST ERROR");
        Err(ServerFnError::ServerError("TEST ERROR".into()))
    } else {
        Ok("everything is O.K.".to_string())
    }
}
