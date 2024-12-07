use axum::routing::post;

#[cfg(feature = "ssr")]
mod ssr_imports {
    use axum::extract::State;
    pub use axum::{
        body::Body as AxumBody,
        extract::Path,
        http::Request,
        response::{IntoResponse, Response},
        routing::get,
        Router,
    };
    use diabetes_game_admin::app::shell;
    pub use diabetes_game_admin::app::App;
    use leptos::{config::LeptosOptions, context::provide_context};
    pub use leptos_axum::{generate_route_list, LeptosRoutes};

    // This custom handler lets us provide Axum State via context
    pub async fn custom_handler(
        Path(id): Path<String>,
        State(options): State<LeptosOptions>,
        req: Request<AxumBody>,
    ) -> Response {
        leptos::logging::log!("ID {}", id.clone());
        let handler = leptos_axum::render_app_to_stream_with_context(
            move || {
                provide_context(id.clone());
            },
            move || shell(options.clone()),
        );
        handler(req).await.into_response()
    }
}

#[tokio::main]
async fn main() {
    use axum::Router;
    use config::get_configuration;
    use diabetes_game_admin::app::*;
    use diabetes_game_admin::database::AppState;
    use leptos::*;
    use leptos_axum::generate_route_list;
    use prelude::provide_context;
    use sqlx::postgres::PgPoolOptions;
    use ssr_imports::*;
    use std::time::Duration;

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    // logging::log!("where do I run??? {:?} ", leptos_options);
    let addr = leptos_options.site_addr;
    // let routes = generate_route_list(App);

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .idle_timeout(Duration::from_secs(2))
        .connect("http://diabetes@localhost:5432/diabetes")
        .await;

    // migrate!()
    //     .run(&pool)
    //     .await
    //     .expect("could not run SQLx migrations");

    let mut state = AppState::new();
    match pool {
        Ok(p) => {
            state.pool = Some(p);
            state.db_connected = true;
        }
        Err(error) => {
            state.db_error = Some(error.to_string());
        }
    }
    let db_error = state.db_error.clone().unwrap_or_default();
    logging::log!("state {}", &db_error);

    // let state = AppState::new();
    let context = move || provide_context(state.clone());
    let routes = generate_route_list(App);

    let app = Router::new()
        .route("/text-form/:id", get(custom_handler))
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // build our application with a route
    // let app = Router::new()
    //     .leptos_routes_with_context(&leptos_options, routes, context, {
    //         let leptos_options = leptos_options.clone();
    //         move || shell(leptos_options.clone())
    //     })
    //     .fallback(leptos_axum::file_and_error_handler(shell))
    //     .with_state(leptos_options);

    // let app = Router::new()
    //     .leptos_routes_with_handler(leptos_options.clone(), custom_handler)
    //     .fallback(leptos_axum::file_and_error_handler);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    logging::log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
