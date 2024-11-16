#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use diabetes_game_admin::app::*;
    use diabetes_game_admin::fileserv::file_and_error_handler;
    // use diabetes_game_admin::text;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use sqlx::postgres::PgPoolOptions;
    use std::time::Duration;

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let db: sqlx::Pool<sqlx::Postgres> = PgPoolOptions::new()
        .max_connections(10)
        .idle_timeout(Duration::from_secs(2))
        .connect("http://diabetes@localhost:5432/diabetes")
        .await
        .expect("can't connect to database");

    // build our application with a route
    let app = Router::new()
        // .leptos_routes(&leptos_options, routes, App)
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            move || provide_context(db.clone()),
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
