use crate::component::text_table::TextTable;
// use crate::database::init_database2;
use crate::database::{init_database, AppState};
use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // create_resource(|| (), |_| async move { init_database().await });

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/diabetes-game-admin.css"/>
        // <Stylesheet id="leptos" href="/pkg/style/main.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
        <main>
        {
            if let Some(state) = use_context::<AppState>()  {
                if state.db_connected {
                    view!{<Routes>
                        // <Route path="" view=HomePage/>
                        // <Route path="texts" view=TextTable/>
                        <Route path="" view=TextTable/>
                    </Routes>}.into_view()
                } else {
                    view!{<div>"NO DATABASE " {state.db_error.unwrap_or_default()}</div>}.into_view()
                }
            } else {
                view!{<div>"ERROR"</div>}.into_view()
            }
        }
                    // <Routes>
                    //     // <Route path="" view=HomePage/>
                    //     // <Route path="texts" view=TextTable/>
                    //     <Route path="" view=TextTable/>
                    // </Routes>

        </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    if let Some(state) = use_context::<AppState>() {
        logging::log!("has context {}", state.db_connected);
    } else {
        logging::log!("no context");
    }

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <A href="/texts">"Show all texts"</A>
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
