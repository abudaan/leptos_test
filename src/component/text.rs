use leptos::*;

use crate::model::text::get_all_texts;

#[component]
pub fn TextTable() -> impl IntoView {
    let texts = create_resource(|| (), |_| async move { get_all_texts().await });

    view! {
      <Suspense
        fallback=move || view! { <p>"Loading texts..."</p> }
      >
        <div className="d-flex justify-content-between align-items-center">
          <h1>"Texts"</h1>
          <a className="btn btn-primary btn"  href="/somewhere">
            <i className="bi bi-plus"></i>
            "New text"
          </a>
        </div>
        <table className="table table-hover">
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
              each=move || texts.get().unwrap().unwrap()
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
          </tbody>
        </table>
      </Suspense>
    }
}
