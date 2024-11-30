use crate::{
    database::{init_database, AppState},
    model::text::get_all_texts,
};
use leptos::logging::log;
use leptos::*;

// #[server]
// pub async fn check_context() -> Result<bool, ServerFnError> {
//     if let Some(state) = use_context::<AppState>() {
//         logging::log!("state {}", state.db_connected.get());
//         Ok(true)
//     } else {
//         Err(ServerFnError::ServerError("no context".to_string()))
//     }
// }

// #[server]
// pub async fn change_state() -> Result<(), ServerFnError> {
//     use std::time::Duration;
//     use tokio::time::sleep;
//     if let Some(state) = use_context::<AppState>() {
//         logging::log!("change_state 1 {:?}", state.pool);
//         sleep(Duration::from_millis(2000)).await;
//         state.db_connected.set(false);
//         logging::log!("change_state 2 {}", state.db_connected.get());
//         Ok(())
//     } else {
//         logging::log!("No context??");
//         Err(ServerFnError::ServerError("no context".to_string()))
//     }
// }

#[component]
pub fn TextTable() -> impl IntoView {
    // let check = create_resource(|| (), |_| async move { check_context().await });
    log!("TextTable");
    // let state = expect_context::<AppState>();
    // view! { <div> "database connected:" {state.db_connected.get()}</div>}.into_view()

    // if let Some(state) = use_context::<AppState>() {
    //     // let check = create_resource(|| (), |_| async move { change_state().await });
    //     let texts = create_resource(|| (), |_| async move { get_all_texts().await });
    //     // check.get();
    //     log!("database {}", state.db_connected.get());
    //     view! { <div> "database connected:" {state.db_connected.get()}</div>}.into_view()
    // } else {
    //     // let state = expect_context::AppState();
    //     // view! {<div>"Context: " {state.db_error.get()}</div>}
    //     view! {<div>"No context"</div>}.into_view()
    // }
    // let texts = create_resource(|| (), |_| async move { get_all_texts().await });
    // view! {<div>"No context"</div>}.into_view()

    // if let Some(state) = use_context::<AppState>() {
    //     let (data, _) = create_signal(move |state: AppState| state.db_error);
    //     // view! {<div>"Context: " {state.db_connected.get()} " and: " {state.db_error.get()}</div>}
    //     data.with(move |value| {
    //         view! {<div>"Context: " {state.db_error.get()}</div>}
    //     })
    // } else {
    //     view! {<div>"No context"</div>}
    // }

    // logging::log!("TextTable component state.db_error {:?}", state.db_error);

    // let texts = create_resource(|| (), |_| async move { get_all_texts().await });

    // view! {
    //     <Suspense fallback={move || {
    //         view!{<div>"Loading"</div>}
    //     }}>
    //     {
    //         texts.get().map(|data| match data {
    //             Ok(value) => view!{<div>"Context" {value.len()}</div>},
    //             Err(error) => view!{<div>{error.to_string()}</div>}
    //         })
    //     }
    //     </Suspense>
    // }

    // view! {
    //     <div>"hallo"</div>
    // }

    // view! {
    //     <Suspense fallback={move || {
    //         view!{
    //             <div>"Loading"</div>
    //         }
    //     }}>
    //     <div>{texts.get().map(|data| match data {
    //         Ok(value) => format!("length is {}", value.len()),
    //         Err(error) => error.to_string()
    //     })}</div>
    //     </Suspense>
    // }

    let texts = create_resource(
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
    /*
            { move || {
                texts.get().map(|data| match data {
                    Ok(value) => {
                        let (texts, _) = create_signal(value);
                        view!{

                            <h2>"Data"</h2>
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
                                <For
                                    each=texts
                                    key=|text|text.id
                                    let: text
                                >
                                    <tr>
                                        <td>{text.title}</td>
                                        <td>{text.published}</td>
                                        <td align="right">
                                        <a class="btn btn-outline-primary"
                                            href="admin_text_edit">
                                            <i class="bi bi-pencil me-1"></i>
                                            "Edit"
                                        </a>
                                        </td>
                                    </tr>
                                </For>
                                </tbody>
                            </table>
                        }.into_view()
                    },
                    Err(error) => view!{<div>{error.to_string()}</div>}.into_view(),
                })
            }}
    */
            {move || {
                texts.get().map(|data| match data {
                    Ok(value) => {
                        view! {
                            <h2>"Data"</h2>
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
            </Suspense>
        }

    // view! {
    //   <h2>"Data"</h2>
    //   <div class="d-flex justify-content-between align-items-center">
    //     <h1>"Texts"</h1>
    //     <a class="btn btn-primary btn"  href="/somewhere">
    //       <i class="bi bi-plus"></i>
    //       "New text"
    //     </a>
    //   </div>
    //   <table class="table table-hover">
    //     <thead>
    //       <tr>
    //         <th scope="col" class="">
    //           <a>"Title"</a>
    //         </th>
    //         <th scope="col" class="">
    //           <a>"Published"</a>
    //         </th>
    //         <th/>
    //       </tr>
    //     </thead>
    //     <tbody>
    //   <Suspense fallback=move || view! { <p>"Loading texts..."</p> }>
    //   { move || {
    //       texts.get().map(|data| match data {
    //           Ok(value) => {
    //             let (data, _) = create_signal(value);
    //             view! {
    //                 <For
    //                   each= data
    //                   key=|text|text.id
    //                   children=move |text| {
    //                     view! {
    //                       <tr>
    //                         <td>{text.title}</td>
    //                         <td>{text.published}</td>
    //                         <td align="right">
    //                           <a class="btn btn-outline-primary"
    //                             href="admin_text_edit">
    //                             <i class="bi bi-pencil me-1"></i>
    //                             "Edit"
    //                           </a>
    //                         </td>
    //                       </tr>
    //                     }
    //                   }
    //                 />
    //             //     {move || {
    //             //       data.get().into_iter().map(|text|{
    //             //           view! {
    //             //             <tr>
    //             //               <td>{text.title}</td>
    //             //               <td>{text.published}</td>
    //             //               <td align="right">
    //             //               <a class="btn btn-outline-primary"
    //             //               href="admin_text_edit">
    //             //               <i class="bi bi-pencil me-1"></i>
    //             //               "Edit"
    //             //                 </a>
    //             //               </td>
    //             //               </tr>
    //             //             }
    //             //           }).collect_view()
    //             //       }
    //             //   }
    //             }.into_view()
    //           },
    //           Err(error) => {
    //             logging::log!("E R R O R {}", &error);
    //             view!{<div>{error.to_string()}</div>}.into_view()
    //           }
    //         });
    //       }}
    //       </Suspense>
    //     </tbody>
    //   </table>
    // }
    // .into_view()
}
