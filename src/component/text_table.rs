use crate::error_template::ErrorTemplate;
use crate::model::text::get_all_texts;
use crate::model::Text;
use leptos::either::Either;
use leptos::prelude::*;
use leptos::*;
use server::Resource;

#[component]
pub fn TextTable() -> impl IntoView {
    logging::log!("TextTable");

    let texts_resource = Resource::new(
        move || (),
        move |_| {
            // match get_all_texts().await {
            //     Ok(value) => Ok(value),
            //     Err(error) => Err(error.to_string()),
            // }
            get_all_texts()
        },
    );

    let texts_view = move || {
        Suspend::new(async move {
            texts_resource.await.map(|texts| {
                if texts.is_empty() {
                    Either::Left(view! { <p>"No texts were found."</p> })
                } else {
                    Either::Right(
                        texts
                            .iter()
                            .map(move |text| {
                                let id = text.id;
                                view! {
                                    <li>
                                        {text.title.clone()}
                                    </li>
                                }
                            })
                            .collect::<Vec<_>>(),
                    )
                }
            })
        })
    };

    view! {
        <Suspense fallback={ move || {
            view! { <p>"Loading texts..."</p> }.into_any()
        }}>
        <ErrorBoundary fallback=|errors| view! {
            <ErrorTemplate errors={errors.into()} />
        }>

            {texts_view}
        // reactive
        // { move || {
        //         texts.get().map(|data| match data {
        //             Ok(value) => {
        //                 let (texts, _) = signal(value);
        //                 view!{
        //                     <div class="d-flex justify-content-between align-items-center">
        //                         <h1>"Texts"</h1>
        //                         <a class="btn btn-primary btn"  href="/text-form/">
        //                         <i class="bi bi-plus"></i>
        //                         "New text"
        //                         </a>
        //                     </div>
        //                     <table class="table table-hover">
        //                         <thead>
        //                         <tr>
        //                             <th scope="col" class="">
        //                             <a>"Title"</a>
        //                             </th>
        //                             <th scope="col" class="">
        //                             <a>"Published"</a>
        //                             </th>
        //                             <th scope="col" class="">
        //                             <a>"Uuid"</a>
        //                             </th>
        //                         </tr>
        //                         </thead>
        //                         <tbody>
        //                         <For
        //                             each=texts
        //                             key=|text: &Text|text.id
        //                             let: text
        //                         >
        //                         {
        //                             let href = format!("/text-form/{}", text.id);
        //                             view! {<tr>
        //                             <td>{text.title}</td>
        //                             <td>{text.published}</td>
        //                             <td>{text.id.to_string()}</td>
        //                             <td style="align:right">
        //                             <a class="btn btn-outline-primary"
        //                             href={href}>
        //                             <i class="bi bi-pencil me-1"></i>
        //                             "Edit"
        //                             </a>
        //                             </td>
        //                             </tr>}.into_view()
        //                         }
        //                         </For>
        //                         </tbody>
        //                     </table>
        //                 }.into_any()
        //             },
        //             Err(error) => {
        //                 view!{<div>"[ERROR]" {error}</div>}.into_any()
        //             },
        //         });
        //     }}
        </ErrorBoundary>
    </Suspense>
    }
}

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
