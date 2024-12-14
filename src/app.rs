use crate::component::text_form::TextForm;
use crate::database::AppState;
use leptos::prelude::*;
use leptos::*;
use leptos_meta::MetaTags;
use leptos_router::components::*;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                // id=leptos means cargo-leptos will hot-reload this stylesheet
                <link rel="stylesheet" id="leptos" href="/pkg/diabetes-game-admin.css"/>
                <link rel="stylesheet" id="bootstrap" href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css" />
                <link rel="stylesheet" id="bootstrap-icons" href="https://cdn.jsdelivr.net/npm/bootstrap-icons@1.11.3/font/bootstrap-icons.min.css" />
                <MetaTags/>
                <title>Leptos Admin</title>
            </head>
            <body>
                <App/>
                <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/js/bootstrap.bundle.min.js"/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    use crate::component::text_table::TextTable;
    use crate::error_template::{AppError, ErrorTemplate};
    use leptos_meta::*;
    use leptos_router::*;
    // Provides context that manages stylesheets, titles, meta tags, etc.
    // provide_meta_context();

    view! {
        <main>
        <Router>

        // <Suspense fallback = move || {
        //     view!{<div>"Trying to connect to the database..."</div>}.into_view()
        // }>
        // {
        //     db.get().map(|data| match data {
        //         Ok(value) => view!{
        //             <div>"Connected to database: " {value}</div>
        //             <Routes>
        //                 <Route path="" view=HomePage/>
        //                 <Route path="texts" view=TextTable/>
        //             </Routes>
        //         }.into_view(),
        //         Err(error) => view!{<div>{error.to_string()}</div>}.into_view()
        //     })
        // }
        // </Suspense>

            <Routes fallback=|| {
                let mut outside_errors = Errors::default();
                outside_errors.insert_with_default_key(AppError::NotFound);
                view! {
                    <ErrorTemplate outside_errors/>
                }
            }>
                <Route path=path!("") view=HomePage/>
                <Route path=path!("texts") view=TextTable/>
                <ParentRoute path=path!("text-form") view=TextForm>
                    <Route path=path!("/") view=TextForm/>
                    <Route path=path!("/:id") view=TextForm/>
                </ParentRoute>


                // <Route path=path!("text-form/:id?") view=TextForm/>
                // <Route path=path!("text-form/:id") view=TextForm/>
                // <Route path=path!("text-form/:id") view=TextForm2/>
            </Routes>

        </Router>
        </main>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // if let Some(state) = use_context::<AppState>() {
    //     logging::log!("Homepage has context {}", state.db_connected);
    // } else {
    //     logging::log!("Homepage has no context");
    // }
    view! {
        <h1>"Admin"</h1>
        <A href="/texts">"Show all texts"</A>
    }
}

#[component]
fn TextForm2() -> impl IntoView {
    if let Some(state) = use_context::<AppState>() {
        logging::log!("TextForm2 has context {:?}", state);
    } else {
        logging::log!("TextForm2 has no context");
    }
    view! {
        <h1>"TextForm2"</h1>
    }
}

#[component]
pub fn Test() -> impl IntoView {
    let data = if cfg!(target_arch = "wasm32") {
        logging::log!("where do I run??? {}", cfg!(target_arch = "wasm32"));
        vec![0, 1, 2]
    } else {
        logging::log!(
            "where do I run??? {} {}",
            cfg!(target_arch = "x86_64"),
            cfg!(target_os = "linux")
        );
        vec![]
    };
    data.into_iter()
        .map(|value| view! { <span>{value}</span> })
        .collect_view()
}
