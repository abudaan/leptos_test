#[component]
pub fn BusyButton() -> impl IntoView {
    view! {
        <div>
            <A href="/text">"Here is a button"</A>
            <button on:click=move |_| {
                spawn_local(async {
                    let r = get_all_texts().await;
                    logging::log!("all texts {:?}", r);

                });
            }>
            "Get questions"
            </button>
        </div>
    }
}
