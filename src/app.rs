use crate::error_template::{AppError, ErrorTemplate};
use crate::model::text::get_all_texts;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// #[server(AllQuestions, "/api/question")]
// pub async fn all_quesions() -> Result<(), ServerFnError> {
//     let mut conn = db().await?;

//     match sqlx::query("INSERT INTO todos (title, completed) VALUES ($1, false)")
//         .bind(title)
//         .execute(&mut conn)
//         .await
//     {
//         Ok(_row) => Ok(()),
//         Err(e) => Err(ServerFnError::ServerError(e.to_string())),
//     }
// }

#[component]
pub fn BusyButton() -> impl IntoView {
    view! {
        <button on:click=move |_| {
            spawn_local(async {
                let r = get_all_texts(42).await;
                logging::log!("all texts {:?}", r);

            });
        }>
            "Get questions"
        </button>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/diabetes-game-admin.css"/>

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
                <Routes>
                    <Route path="" view=HomePage/>
                    // <Route path="test" view=Test />
                </Routes>
                <BusyButton />
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

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
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
