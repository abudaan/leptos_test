use error::Errors;
use http::status::StatusCode;
use leptos::control_flow::For;
use leptos::prelude::ElementChild;
use leptos::*;
use prelude::{use_context, GetUntracked, RwSignal, ServerFnError};
use thiserror::Error;

#[derive(Clone, Debug, Error, Default)]
pub enum AppError {
    // #[error("Server Error")]
    // ServerFnError,
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

// impl From<ServerFnError> for AppError {
//     fn from(err: ServerFnError) -> Self {
//         AppError::General
//     }
// }

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    pub fn msg(&self) -> String {
        // match self {
        //     AppError::NotFound => StatusCode::NOT_FOUND,
        // }
        self.to_string()
    }

    // #[must_use]
    // pub fn try_into_server_error(self) -> Result<ServerFnError, Self> {
    //     if let Self::ServerError(v) = self {
    //         Ok(v)
    //     } else {
    //         Err(self)
    //     }
    // }
}

// A basic function to display errors served by the error boundaries.
// Feel free to do more complicated things here than just displaying the error.
#[component]
pub fn ErrorTemplate(
    #[prop(optional)] outside_errors: Option<Errors>,
    #[prop(optional)] errors: Option<RwSignal<Errors>>,
) -> impl IntoView {
    // logging::log!("outside_errors {:?} ", outside_errors);
    // logging::log!("errors {:?} ", errors);

    let errors = match outside_errors {
        Some(e) => RwSignal::new(e),
        None => match errors {
            Some(e) => e,
            None => panic!("No Errors found and we expected errors!"),
        },
    };

    // Get Errors from Signal
    let errors = errors.get_untracked();

    // Downcast lets us take a type that implements `std::error::Error`
    // let errors: Vec<AppError> = errors
    //     .into_iter()
    //     .filter_map(|(k, v)| {
    //         logging::log!("Error {:?} ", v);
    //         let e = v.downcast_ref::<AppError>().cloned();
    //         // logging::log!("ErrorId {} Msg {}", k, e.clone().unwrap_or_default().msg());
    //         logging::log!("Error downcast {:?}", e);
    //         e
    //     })
    //     .collect();

    let errors = move || errors.into_iter().map(|(_, v)| v).collect::<Vec<_>>();
    // println!("Errors: {:?}", errors.clone()());

    // Only the response code for the first error is actually sent from the server
    // this may be customized by the specific application
    // #[cfg(feature = "ssr")]
    // {
    //     use leptos_axum::ResponseOptions;
    //     let response = use_context::<ResponseOptions>();
    //     if let Some(response) = response {
    //         response.set_status(errors[0].status_code());
    //     }
    // }

    view! {
        <h1>{if errors.clone()().len() > 1 {"Errors"} else {"Error"}}</h1>
        <For
            // a function that returns the items we're iterating over; a signal is fine
            each= move || {errors.clone()().into_iter().enumerate()}
            // a unique key for each item as a reference
            key=|(index, _error)| *index
            // renders each item to a view
            children=move |error| {
                let error_string = error.1.to_string();
                // let error_code= error.1.status_code();
                view! {
                    // <h2>{error_code.to_string()}</h2>
                    <p>{error_string}</p>
                }
            }
        />
    }
}
