use crate::model::text::get_all_texts;
use leptos::*;

#[component]
pub fn TextTable() -> impl IntoView {
    logging::log!("TextTable");

    let texts: Resource<(), Result<Vec<crate::model::Text>, String>> = create_resource(
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
                view! { <p>"Loading texts..."</p> }}
            }>
            // reactive
            { move || {
                texts.get().map(|data| match data {
                    Ok(value) => {
                        let (texts, _) = create_signal(value);
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
                                    key=|text|text.id
                                    let: text
                                >
                                {
                                    let href = format!("/text-form/{}",text.id.to_string());
                                    view! {<tr>
                                    <td>{text.title}</td>
                                    <td>{text.published}</td>
                                    <td>{text.id.to_string()}</td>
                                    <td align="right">
                                    <a class="btn btn-outline-primary"
                                    href={href}>
                                    <i class="bi bi-pencil me-1"></i>
                                    "Edit"
                                    </a>
                                    </td>
                                    </tr>}
                                }
                                </For>
                                </tbody>
                            </table>
                        }.into_view()
                    },
                    Err(error) => view!{<div>{error.to_string()}</div>}.into_view(),
                })
            }}
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
                        }.into_view()
                    },
                    Err(error) => {
                        view!{<div>{error.to_string()}</div>}.into_view()
                    }
                })
            }}
       */
            </Suspense>
        }
}
