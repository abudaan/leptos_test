use leptos::{ev, logging, prelude::*};
use leptos::{server::ServerAction, IntoView};

use crate::model::test::Test;

#[component]
pub fn Test1() -> impl IntoView {
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
