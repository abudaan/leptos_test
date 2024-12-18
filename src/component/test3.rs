use leptos::{ev, logging, prelude::*};
use leptos::{server::ServerAction, IntoView};
use leptos_router::components::Form;
use wasm_bindgen::JsCast;

use crate::model::test::Test;

#[component]
pub fn Test3() -> impl IntoView {
    let test_action = ServerAction::<Test>::new();
    view! {
        <Form
            action="/api/test"
            method="post"
            on:submit=move |ev| {
                let data = Test::from_event(&ev).expect("to parse form data");
                if data.value.is_empty() {
                    logging::log!("prevent default");
                    ev.prevent_default();
                } else {

                    // test_action.dispatch(data);
                    let form = ev.target();
                    let form_element = form.unwrap().dyn_into::<web_sys::HtmlFormElement>().unwrap();
                    form_element.submit().unwrap();
                }
            }
        >
            <input type="text" name="value"/>
            <input type="submit" value="Submit"/>
        </Form>
    }
}
