use leptos::{logging, prelude::*};
use leptos::{server::ServerAction, IntoView};

use crate::model::test::Test;

#[component]
pub fn Test5() -> impl IntoView {
    let test_action = ServerAction::<Test>::new();

    view! {
        <ErrorBoundary
            fallback=move |error| {
                logging::log!("error {:?}", error.get());
                view!{<div>"ERROR"</div>}
            }>
            <ActionForm
                action=test_action
            >
                <input type="text" name="value"/>
                <input type="submit" value="Submit"/>
            </ActionForm>
        </ErrorBoundary>
    }
}
