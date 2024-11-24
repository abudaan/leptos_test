#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    // let db = PgPool::connect("your_database_url").await.unwrap();
    // let app_state = get_app_state().await;
    // let app_state = get_app_state();
    // let app_state = init_database().await;
    use axum::Router;
    use diabetes_game_admin::database::AppState;
    // use diabetes_game_admin::database::init_database;
    use diabetes_game_admin::app::*;
    use diabetes_game_admin::fileserv::file_and_error_handler;
    // use diabetes_game_admin::{app::*, database::get_app_state};
    // use diabetes_game_admin::text;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use sqlx::postgres::PgPoolOptions;
    use std::time::Duration;

    // let pool = PgPoolOptions::new()
    //     .max_connections(10)
    //     .idle_timeout(Duration::from_secs(2))
    //     .connect("http://diabetes@localhost:5432/diabetes")
    //     .await
    //     .expect("could not connect to database");

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    // logging::log!("where do I run??? {:?} ", leptos_options);
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // build our application with a route
    let app = Router::new()
        // .leptos_routes(&leptos_options, routes, App)
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            move || {
                provide_context(AppState {
                    db: None,
                    error: None,
                })
            },
            App,
        )
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

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
