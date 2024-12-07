use crate::database::AppState;
use crate::model::text::get_all_texts;
use crate::model::Text;
use leptos::prelude::*;
use leptos::*;
use server::Resource;

#[component]
pub fn TextTable() -> impl IntoView {
    logging::log!("TextTable");

    if let Some(state) = use_context::<AppState>() {
        if let Some(error) = state.db_error {
            // Ok(pool);
            view! {<div>{error}</div>}.into_any()
        } else {
            // tracing::error!("No database");
            // Err(ServerFnError::new(format!(
            //     "No database connection {}",
            //     state.db_error.unwrap_or_default()
            // )));
            view! {<div>{"OK!"}</div>}.into_any()
        }
    } else {
        // tracing::error!("No context");
        // Err(ServerFnError::new("No context available"));
        view! {<div>{"No context available"}</div>}.into_any()
    }
    /*
    let texts = Resource::new(
        || (),
        |_| async move {
            match get_all_texts().await {
                Ok(value) => Ok(value),
                Err(error) => Err(error.to_string()),
            }
        },
    );

    view! {
            <Suspense fallback={ move || {
                view! { <p>"Loading texts..."</p> }.into_any()
            }}>
            // reactive
            { move || {
                texts.get().map(|data| match data {
                    Ok(value) => {
                        let (texts, _) = signal(value);
                        view!{
                            <div class="d-flex justify-content-between align-items-center">
                                <h1>"Texts"</h1>
                                <a class="btn btn-primary btn"  href="/text-form/">
                                <i class="bi bi-plus"></i>
                                "New text"
                                </a>
                            </div>
                            <table class="table table-hover">
                                <thead>
                                <tr>
                                    <th scope="col" class="">
                                    <a>"Title"</a>
                                    </th>
                                    <th scope="col" class="">
                                    <a>"Published"</a>
                                    </th>
                                    <th scope="col" class="">
                                    <a>"Uuid"</a>
                                    </th>
                                </tr>
                                </thead>
                                <tbody>
                                <For
                                    each=texts
                                    key=|text: &Text|text.id
                                    let: text
                                >
                                {
                                    let href = format!("/text-form/{}",text.id.to_string());
                                    view! {<tr>
                                    <td>{text.title}</td>
                                    <td>{text.published}</td>
                                    <td>{text.id.to_string()}</td>
                                    <td style="align:right">
                                    <a class="btn btn-outline-primary"
                                    href={href}>
                                    <i class="bi bi-pencil me-1"></i>
                                    "Edit"
                                    </a>
                                    </td>
                                    </tr>}.into_view()
                                }
                                </For>
                                </tbody>
                            </table>
                        }.into_any()
                    },
                    Err(error) => {
                        logging::log!("{}", error);
                        // view!{<div>"something"</div>}.into_view()
                        view!{<div>{error.to_string()}</div>}.into_any()
                    },
                });
            }}
    */
    // static
    /*
         {move || {
             texts.get().map(|data| match data {
                 Ok(value) => {
                     view! {
                         <div class="d-flex justify-content-between align-items-center">
                             <h1>"Texts"</h1>
                             <a class="btn btn-primary btn"  href="/somewhere">
                             <i class="bi bi-plus"></i>
                             "New text"
                             </a>
                         </div>
                         <table class="table table-hover">
                         <thead>
                         <tr>
                             <th scope="col" class="">
                             <a>"Title"</a>
                             </th>
                             <th scope="col" class="">
                             <a>"Published"</a>
                             </th>
                             <th/>
                         </tr>
                         </thead>
                         <tbody>
                         {
                             value.into_iter().map(|text| {
                                 view!{<tr>
                                     <td>{text.title}</td>
                                     <td>{text.published}</td>
                                     <td align="right">
                                     <a class="btn btn-outline-primary" href="admin_text_edit">
                                     <i class="bi bi-pencil me-1"></i>
                                     "Edit"
                                     </a>
                                     </td>
                                 </tr>}
                             }).collect_view()
                         }
                         </tbody>
                     </table>
                     }.into_any()
                 },
                 Err(error) => {
                     view!{<div>{error.to_string()}</div>}.into_any()
                 }
             })
         }}
    */
    // </Suspense>
    // }
}
