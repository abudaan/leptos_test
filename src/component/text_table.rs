use leptos::*;

use crate::{database::init_database2, model::text::get_all_texts};

#[component]
pub fn TextTable() -> impl IntoView {
    let texts: Resource<(), Result<Vec<crate::model::Text>, ServerFnError>> = create_resource(
        || (),
        |_| async move {
            let r = init_database2().await;
            match r {
                Ok(_) => {
                    let r1 = get_all_texts().await;
                    match r1 {
                        Ok(texts) => Ok(texts),
                        Err(error) => Err(error),
                    }
                }
                Err(error) => Err(error),
            }
        },
    );
    view! {
      <Suspense fallback=move || view! { <p>"Loading texts..."</p> }>
      { move || {
          texts.get().map(|data| match data {
              Ok(value) => {
                let (data, _) = create_signal(value);
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
                    <For
                      each= data
                      key=|text|text.id
                      children=move |text| {
                        view! {
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
                        }
                      }
                    />
                    {move || {
                      data.get().into_iter().map(|text|{
                          view! {
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
                            }
                          }).collect_view()
                      }
                  }
                  </tbody>
                  </table>
                }.into_view()
              },
              Err(error) => {
                logging::log!("E R R O R {}", &error);
                view!{<div>{error.to_string()}</div>}.into_view()
              }
            });
          }}

        </Suspense>
    }
}
// <div className="d-flex justify-content-between align-items-center">
//   <h1>"Texts"</h1>
//   <a className="btn btn-primary btn"  href="/somewhere">
//     <i className="bi bi-plus"></i>
//     "New text"
//   </a>
// </div>
// <table className="table table-hover">
//   <thead>
//     <tr>
//       <th scope="col" class="">
//         <a>"Title"</a>
//       </th>
//       <th scope="col" class="">
//         <a>"Published"</a>
//       </th>
//       <th/>
//     </tr>
//   </thead>
//   <tbody>

//   </tbody>
//   </table>
