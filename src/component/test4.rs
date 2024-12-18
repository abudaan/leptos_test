use leptos::{ev, logging, prelude::*};
use leptos::{server::ServerAction, IntoView};

use crate::model::test::Test;

#[component]
pub fn Test4() -> impl IntoView {
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

    view! {
        <ActionForm action=test_action>
            <input type="text" name="value"/>
            <pre>{action_value}</pre>
            <input type="submit" value="Submit"/>
        </ActionForm>
    }
}
