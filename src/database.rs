use leptos::{
    create_rw_signal, expect_context, logging, server, use_context, RwSignal, ServerFnError,
};
use leptos::{SignalGet, SignalSet};
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::{Error, PgPool};

// #[derive(Clone)]
// pub struct AppState {
//     pub db_error: Option<String>,
//     #[cfg(feature = "ssr")]
//     pub db: Option<PgPool>,
// }

#[derive(Clone)]
pub struct AppState {
    pub db_error: RwSignal<Option<String>>,
    pub db_connected: RwSignal<bool>,
    #[cfg(feature = "ssr")]
    pub db: Option<PgPool>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            db_error: create_rw_signal(Some("No error".to_string())),
            db_connected: create_rw_signal(false),
            #[cfg(feature = "ssr")]
            db: None,
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
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

/*
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
*/
#[server]
pub async fn init_database() -> Result<bool, ServerFnError> {
    if let Some(mut state) = use_context::<AppState>() {
        if state.db.is_some() {
            Ok(true)
        } else {
            // logging::log!("database 1 {:?}", state.db);

            use sqlx::postgres::PgPoolOptions;
            use std::time::Duration;
            use tokio::time::sleep;
            sleep(Duration::from_millis(2000)).await;

            let result = PgPoolOptions::new()
                .max_connections(10)
                .idle_timeout(Duration::from_secs(2))
                .connect("http://diabetes@localhost:5432/diabetes")
                .await;

            match result {
                Ok(value) => {
                    state.db = Some(value);
                    state.db_connected.set(true);
                    Ok(true)
                }
                Err(error) => {
                    state.db_error.set(Some(error.to_string()));
                    logging::log!("error {:?}", error);
                    Err(ServerFnError::ServerError(error.to_string()))
                }
            }
        }
    } else {
        Err(ServerFnError::ServerError(
            "No context, no AppState, no nothing".to_owned(),
        ))
    }
}

#[server]
pub async fn database_connected() -> Result<bool, ServerFnError> {
    if let Some(state) = use_context::<AppState>() {
        if state.db.is_none() {
            init_database().await
        } else {
            Ok(state.db.is_none())
        }
    } else {
        Err(ServerFnError::ServerError(
            "No AppState found in context".to_owned(),
        ))
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
