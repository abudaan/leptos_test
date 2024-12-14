use leptos::{
    logging,
    prelude::{use_context, ServerFnError},
    server,
};
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::{Error, PgPool};

// #[derive(Clone)]
// pub struct AppState {
//     pub db_error: Option<String>,
//     #[cfg(feature = "ssr")]
//     pub db: Option<PgPool>,
// }
#[derive(Clone, Debug)]
pub struct AppState {
    pub db_error: Option<String>,
    pub db_connected: bool,
    #[cfg(feature = "ssr")]
    pub pool: Option<PgPool>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            db_error: None,
            db_connected: false,
            // db_error: create_rw_signal(Some("No error".to_string())),
            // db_connected: create_rw_signal(false),
            #[cfg(feature = "ssr")]
            pool: None,
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
        if state.pool.is_some() {
            logging::log!("database exists {:?}", state.pool);
            Ok(true)
        } else {
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
                    state.pool = Some(value);
                    // state.db_connected.set(true);
                    state.db_connected = true;
                    // logging::log!("database connected {:?}", state.db);
                    Ok(true)
                }
                Err(error) => {
                    // state.db_error.set(Some(error.to_string()));
                    state.db_error = Some(error.to_string());
                    logging::log!("error {:?}", error);
                    Err(ServerFnError::ServerError(error.to_string()))
                }
            }
        }
    } else {
        Err(ServerFnError::ServerError("no context".to_owned()))
    }
}

#[server]
pub async fn database_connected() -> Result<bool, ServerFnError> {
    if let Some(state) = use_context::<AppState>() {
        if state.pool.is_none() {
            init_database().await
        } else {
            Ok(state.pool.is_none())
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
