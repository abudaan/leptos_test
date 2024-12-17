use crate::component::text_form::TextForm;
use crate::database::AppState;
use crate::error_template::ErrorTemplate;
use crate::model::test::Test;
use crate::model::text::Add;
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
                <Route path=path!("test") view=Test/>
                <Route path=path!("test1") view=Test1/>


                // <Route path=path!("text-form/:id?") view=TextForm/>
                // <Route path=path!("text-form/:id") view=TextForm/>
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

fn create_test_view(t: Option<String>) -> impl IntoView {
    let add_text = ServerAction::<Add>::new();
    let (name, set_name) = signal(String::new());
    if let Some(t) = t {
        set_name.set(t);
    }
    view! {
        <ActionForm  action=add_text>
        <input type="text"
        // adding :target gives us typed access to the element
        // that is the target of the event that fires
        on:input:target=move |ev| {
            // .value() returns the current value of an HTML input element
            set_name.set(ev.target().value());
        }

        // the `prop:` syntax lets you update a DOM property,
        // rather than an attribute.
        prop:value=name
        />
        <p>"Name is: " {name}</p>
        </ActionForm>
    }
}

#[component]
fn Test() -> impl IntoView {
    // create_test_view(None)
    create_test_view(Some("Aap en Beer".to_string()))
}

#[component]
fn Test1() -> impl IntoView {
    let test_action = ServerAction::<Test>::new();

    let action_value = Signal::derive(move || {
        let r = test_action.value().get();
        if let Some(r) = r {
            match r {
                Err(error) => error.to_string(),
                Ok(value) => value,
            }
        } else {
            String::new()
        }
    });

    let (error, set_error) = signal(true);
    let mut index = 0;
    let print_error = move || {
        let a = action_value.get();
        let b = error.get();
        if b {
            a
        } else {
            index += 1;
            format!("xx {}", index)
        }
    };

    let on_submit = move |ev: ev::SubmitEvent| {
        // tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        set_error(true)
    };

    // Effect::new_isomorphic(move |_| {
    //     logging::log!("Got value = {:?}", action_value.get());
    // });

    // Effect::watch(
    //     move || action_value.get(),
    //     move |val, _prev_val, _| {
    //         set_error.set(val.to_owned());
    //     },
    //     false,
    // );

    view! {
        // <ErrorBoundary
        //     fallback=move |error| {
        //         move || format!("{:#?}", error.get())
        //     }>
            <pre>{print_error}</pre>
            <ActionForm
                action=test_action
                // on:submit=on_submit
            >
                <input type="text" name="value"
                    on:focus=move|_ev|set_error.set(false)
                    on:blur=move|_ev|set_error.set(true)
                />
                <button type="submit" on:hover=move|_:leptos::ev::Event|{
                    logging::log!("hover!");
                    set_error.set(true);
                }>"Submit"</button>
            </ActionForm>
        // </ErrorBoundary>
    }
}

// #[component]
// pub fn ReturnsError() -> impl IntoView {
//     Err::<String, AppError>(AppError::InternalServerError)
// }

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
pub fn Test2() -> impl IntoView {
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
