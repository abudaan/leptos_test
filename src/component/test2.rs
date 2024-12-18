use leptos::{ev, logging, prelude::*};
use leptos::{server::ServerAction, IntoView};

use crate::model::test::Test;

#[component]
pub fn Test2() -> impl IntoView {
    let test_action = ServerAction::<Test>::new();
    view! {
        <ActionForm
            on:submit=move |ev| {
                let data = Test::from_event(&ev).expect("to parse form data");
                if data.value.is_empty() {
                // if data.value == "aap" {
                    logging::log!("prevent default");
                    ev.prevent_default();
                }
            }
            action=test_action
        >
            <input type="text" name="value" required="true"/>
            <input type="submit" value="Submit"/>
        </ActionForm>
    }
}
