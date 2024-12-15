use either::Either;
use leptos::prelude::*;
use leptos::*;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;
use prelude::Read;
use serde::Deserialize;
use serde::Serialize;
use server::Resource;
use uuid::Uuid;

use crate::error_template::ErrorTemplate;
use crate::model::text::get_one;
use crate::model::Text;

#[derive(Params, PartialEq, Serialize, Deserialize)]
struct TextParams {
    id: Option<Uuid>,
}

fn create_modal(title: String) -> impl IntoView {
    view! {
    <div class="modal" id="deleteTextModal">
      <div class="modal-dialog">
        <div class="modal-content">
          <div class="modal-header">
            <h5 class="modal-title">Warning!</h5>
            <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
          </div>
          <div class="modal-body">
            <p>Are you sure that you want to delete the text <b>{title}</b>?</p>
          </div>
          <div class="modal-footer">
            <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Close</button>
            <button type="button" class="btn btn-danger" data-bs-dismiss="modal"
            on:click= move |ev| {
              // deleteText(text.id).then();
              // setLoading(true);
              // navigate('admin_text_index');
            }>
              Delete text
            </button>
          </div>
        </div>
      </div>
    </div>}
}

fn create_view(text: &Option<Text>) -> impl IntoView {
    let (title, set_title) = signal(String::new());
    let (content, set_content) = signal(String::new());
    let (published, set_published) = signal(false);
    let mut new_entry = true;
    if let Some(text) = text {
        let text = text.clone();
        set_title(text.title.clone());
        set_content(text.content.clone());
        set_published(text.published);
        new_entry = false;
    }
    // logging::log!("{} {} {}", title, published, content);

    view! {
    <form class="row g-9" autocomplete="off" novalidate="true">
      <div class="col-md-9">
        <label for="title" class="form-label">Title</label>
        <input type="text" id="title" class="form-control" required="true"
            prop:value={title}
            // on:input=move |ev| {
            //   set_title(event_target_value(&ev));
            //   if title().is_empty() {
            //     logging::log!("{:?} {} {}", ev, title(), title().is_empty());
            //   }
            // }
            // on:blur=move |ev| {
            //   set_title(event_target_value(&ev));
            //   logging::log!("{:?} {}", ev, title());
            // }
            on:input:target=move |ev| {
              // .value() returns the current value of an HTML input element
              set_title(ev.target().value());
              // logging::log!("{}", title());
            }
        />
        <Show
          when=move || title().is_empty()
          fallback=|| ().into_any()
        >
          // <div class:invalid-feedback=move || title().is_empty()>
          <div class="error">
            "Please enter a title"
          </div>
        </Show>
      </div>

      <div class="col-md-9">
        <label for="content" class="form-label">Content</label>
        <textarea id="content" required="true" class="form-control" rows="10"
          prop:value=content
          on:input:target=move|ev|set_content(ev.target().value())
        >
          {content.get_untracked()}
        </textarea>
        <Show
          when=move || content().is_empty()
          fallback=|| ().into_any()
        >
          <div class="error">
            "Please enter content"
          </div>
      </Show>
      </div>

      <div class="col-md-9">
        <input type="checkbox" id="published" checked={published} class="ml-2"
          on:change:target=move|ev|set_published(ev.target().checked()) />
        <label htmlFor="published" className="form-label mx-1">Published</label>
      </div>

      <Show
        when=move || !new_entry
        fallback=|| ().into_any()
      >
        <div class="col-12">
          <button class="btn btn-outline-danger me-2" type="button"
            data-bs-toggle="modal" data-bs-target="#deleteTextModal">
            Delete text
          </button>
        </div>
      </Show>


      // <div className="col-12 pt-3 mb-5">
      //   <button type="button" className="btn btn-primary me-2" on:click:target= move |ev: PointerEvent|{
      //       // logging::log!("click {}", ev.target().unwrap().as_ref::<HtmlInputElement>());
      //       logging::log!("click {}", ev.target().unwrap().to_string());
      //   }>
      //     Save
      //   </button>
      //   <button type="button" className="btn btn-outline-danger"
      //       on:click:target= move |ev: PointerEvent|{
      //       // logging::log!("click {}", ev.target().unwrap().as_ref::<HtmlInputElement>());
      //       logging::log!("click {}", ev.target().unwrap().to_string());
      //   }
      //   >Cancel
      //   </button>
      // </div>
    </form>

    {create_modal(title())}

    // {status === 'success' &&
    //   <div className="alert alert-success mb-5">
    //     Saved successfully!
    //   </div>}

    // {status === 'failure' &&
    //   <div className="alert alert-danger mb-5">
    //     Something went wrong!
    //   </div>}

    // <DeleteModal text={text} setLoading={setLoading} />

        }
    .into_view()
}

#[component]
pub fn TextForm() -> impl IntoView {
    logging::log!("TextForm");

    let params = use_params::<TextParams>();

    let id = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.id)
            .unwrap_or_default()
    };

    let text_resource = Resource::new(
        move || (),
        move |_| async move {
            let id = id();
            if id != Uuid::default() {
                match get_one(id).await {
                    Ok(value) => Ok(Some(value)),
                    Err(error) => Err(error.to_string()),
                }
            } else {
                Ok(None)
            }
        },
    );

    let text_view = move || {
        Suspend::new(async move {
            text_resource.map(|text| match text {
                Ok(text) => create_view(text).into_any(),
                Err(err) => view! { <p>{err.to_string()}</p> }.into_any(),
            })
        })
    };

    view! {
        <Suspense fallback={ move || {
            view! { <p>"Loading text..."</p> }.into_any()
        }}>
          <ErrorBoundary fallback=|errors| view! {
              <ErrorTemplate errors={errors.into()} />
          }>
              {text_view}
          </ErrorBoundary>
      </Suspense>
    }
    .into_any()
}

// let mut title = "".to_string();
// let mut published = true;
// text.get().map(|r| {
//     if let Ok(o) = r {
//         if let Some(text) = o {
//             title = text.title;
//             published = text.published;
//         }
//     }
// });

// if let Some(r) = text.get() {
//     if let Ok(o) = r {
//         if let Some(text) = o {
//             title = text.title;
//             published = text.published;
//         };
//     }
// }

// fn func(opt: Option<Result<u64, String>>) {
//     let n = match opt {
//         Some(Ok(n)) => n,
//         _ => return,
//     };
// }

// fn func2(res: Result<Option<Text>, String>) {
//     let n = match res {
//         Ok(Some(n)) => n,
//         _ => return,
//     };
// }
