use leptos::{logging, prelude::*};
use leptos::{server::ServerAction, IntoView};
#[cfg(feature = "ssr")]
use sqlx::PgPool;

use crate::model::text::Add;

#[derive(Clone, Debug)]
pub struct AppState {
    pub db_error: Option<String>,
    pub db_connected: bool,
    #[cfg(feature = "ssr")]
    pub pool: Option<PgPool>,
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
pub fn Test() -> impl IntoView {
    // create_test_view(None)
    create_test_view(Some("Test test test".to_string()))
}

// #[component]
// pub fn ReturnsError() -> impl IntoView {
//     Err::<String, AppError>(AppError::InternalServerError)
// }

#[component]
pub fn TextForm2() -> impl IntoView {
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
pub fn Test200() -> impl IntoView {
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
