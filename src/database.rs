use leptos::{expect_context, server, ServerFnError};
#[cfg(feature = "ssr")]
use sqlx::{Error, PgPool};

#[cfg(feature = "ssr")]
#[derive(Clone)]
pub struct AppState {
    pub db: Option<PgPool>,
    pub error: Option<String>,
}

// #[cfg(feature = "ssr")]
// impl AppState {
//     async fn init_database(&mut self) -> bool {
//         let result = PgPoolOptions::new()
//             .max_connections(10)
//             .idle_timeout(Duration::from_secs(2))
//             .connect("http://diabetes@localhost:5432/diabetes")
//             .await;

//         match result {
//             Ok(value) => {
//                 self.db = Some(value);
//                 self.error = None;
//                 true
//             }
//             Err(error) => {
//                 self.db = None;
//                 self.error = Some(error.to_string());
//                 false
//             }
//         }
//     }
// }

// #[cfg(feature = "ssr")]
// pub fn get_app_state() -> AppState {
//     AppState {
//         db: None,
//         error: None,
//     }
// }

#[cfg(feature = "ssr")]
pub async fn init_database() -> Result<PgPool, Error> {
    use sqlx::postgres::PgPoolOptions;
    use std::time::Duration;

    let result = PgPoolOptions::new()
        .max_connections(10)
        .idle_timeout(Duration::from_secs(2))
        .connect("http://diabetes@localhost:5432/diabetes")
        .await;

    match result {
        Ok(value) => Ok(value),
        Err(error) => Err(error),
    }
}

#[server]
pub async fn init_database2() -> Result<bool, ServerFnError> {
    let mut state = expect_context::<AppState>();
    use sqlx::postgres::PgPoolOptions;
    use std::time::Duration;
    use tokio::time::sleep;
    sleep(Duration::from_millis(1000)).await;

    let result = PgPoolOptions::new()
        .max_connections(10)
        .idle_timeout(Duration::from_secs(2))
        .connect("http://diabetes@localhost:5432/diabetes")
        .await;

    match result {
        Ok(value) => {
            state.db = Some(value);
            Ok(true)
        }
        Err(error) => Err(ServerFnError::ServerError(error.to_string())),
    }
}

// #[cfg(feature = "ssr")]
// pub async fn get_app_state() -> AppState {
//     let result = PgPoolOptions::new()
//         .max_connections(10)
//         .idle_timeout(Duration::from_secs(2))
//         .connect("http://diabetes@localhost:5432/diabetes")
//         .await;

//     match result {
//         Ok(value) => AppState {
//             db: Some(value),
//             error: None,
//         },
//         Err(error) => AppState {
//             db: None,
//             error: Some(error.to_string()),
//         },
//     }
// }
