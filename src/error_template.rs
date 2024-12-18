use error::Errors;
use http::status::StatusCode;
use leptos::control_flow::For;
use leptos::prelude::ElementChild;
use leptos::*;
use prelude::{GetUntracked, RwSignal};
use thiserror::Error;

#[derive(Clone, Debug, Error, Default)]
pub enum AppError {
    #[error("Not Found")]
    NotFound,
    #[error("Could not connect to database")]
    DatabaseError,
    #[error("No texts found")]
    NoTexts,
    #[default]
    #[error("General error")]
    General,
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    pub fn msg(&self) -> String {
        self.to_string()
    }
}

// A basic function to display errors served by the error boundaries.
// Feel free to do more complicated things here than just displaying the error.
#[component]
pub fn ErrorTemplate(
    #[prop(optional)] outside_errors: Option<Errors>,
    #[prop(optional)] errors: Option<RwSignal<Errors>>,
) -> impl IntoView {
    let errors = match outside_errors {
        Some(e) => RwSignal::new(e),
        None => match errors {
            Some(e) => e,
            None => panic!("No Errors found and we expected errors!"),
        },
    };

    // Get Errors from Signal
    let errors = errors.get_untracked();
    let errors = move || errors.into_iter().map(|(_, v)| v).collect::<Vec<_>>();

    view! {
        <h1>{if errors.clone()().len() > 1 {"Errors"} else {"Error"}}</h1>
        <For
            each= move || {errors.clone()().into_iter().enumerate()}
            key=|(index, _error)| *index
            children=move |error| {
                let error_string = error.1.to_string();
                view! {
                    <p>{error_string}</p>
                }
            }
        />
    }
}
