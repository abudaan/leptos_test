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
use crate::model::text::{AddOrUpdate, Delete};
use crate::model::Text;

#[derive(Params, PartialEq, Serialize, Deserialize)]
struct TextParams {
    id: Option<Uuid>,
}

fn create_modal(title: String, id: Uuid) -> impl IntoView {
    let navigate = leptos_router::hooks::use_navigate();
    let server_action = ServerAction::<Delete>::new();
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
            on:click= move |_ev| {
              server_action.dispatch(Delete {id});
              navigate("/texts", Default::default());
            }>
              Delete text
            </button>
          </div>
        </div>
      </div>
    </div>}
}

fn create_view(text: &Option<Text>) -> impl IntoView {
    let mut id: Option<Uuid> = None;
    let mut content = String::new();
    let (title, set_title) = signal("".to_string());
    let (published, set_published) = signal("false");

    if let Some(text) = text {
        let text = text.clone();
        id = Some(text.id);
        // title = text.title.clone();
        content = text.content.clone();
        set_title.set(text.title.clone());
        set_published.set(if text.published { "true" } else { "false" });
    }

    let (has_error_title, set_error_title) = signal(false);
    let (has_error_content, set_error_content) = signal(false);

    let server_action = ServerAction::<AddOrUpdate>::new();
    let pending = server_action.pending();
    let value = server_action.value();

    view! {
        <ActionForm
          action=server_action
          on:submit=move|ev|ev.prevent_default()
        >
        <div class="col-md-9">
          <label for="title" class="form-label">Title</label>
          <input type="text" id="title" name="text[title]" class="form-control"
              on:focus=move |_ev| set_error_title.set(false)
              on:blur:target=move |ev| {
                if ev.target().value().is_empty() {
                  set_error_title.set(true);
              }}
              on:change:target=move |ev| set_title.set(ev.target().value())

              prop:value=title
          />
          <Show
            when=has_error_title
            fallback=|| ().into_any()
          >
            <div class="error">
              "Please enter a title"
            </div>
          </Show>
        </div>

        <div class="col-md-9">
          <label for="content" class="form-label">Content</label>
          <textarea id="content" name="text[content]" class="form-control" rows="10"
            on:blur:target=move|ev|{
              if ev.target().value().is_empty() {
                set_error_content.set(true);
            }}
            on:focus=move|_ev|set_error_content.set(false)
          >
            {content}
          </textarea>
          <Show
            when=has_error_content
            fallback=|| ().into_any()
          >
            <div class="error">
              "Please enter content"
            </div>
          </Show>
        </div>

        <div class="col-md-9">
          <input type="checkbox" id="published" checked={move || published() == "true"} class="ml-2"
            on:change:target=move|ev|set_published(if ev.target().checked() {"true"} else {"false"})
          />
          <label for="published" class="form-label mx-1">Published</label>
          <input
              type="text"
              name="text[published]"
              hidden
              value=published
          />
        </div>

        <Show
          when=move || id.is_some()
          fallback=|| ().into_any()
        >
          <div class="col-12">
            <button class="btn btn-outline-danger me-2" type="button"
              data-bs-toggle="modal" data-bs-target="#deleteTextModal">
              Delete text
            </button>
          </div>
        </Show>


        <div class="col-12 pt-3 mb-5">
          <input type="submit" class="btn btn-primary me-2" value="Save" />
          <button type="button" class="btn btn-outline-danger"
              on:click:target= move |ev|{
              logging::log!("click {}", ev.target().to_string());
          }
          >Cancel
          </button>
        </div>
      </ActionForm>

      {move ||
        match id {
          Some(id) => create_modal(title(), id).into_any(),
          None => ().into_any()
        }
      }

      <div>
        {move || pending().then(|| view!{<div class="alert alert-success mb-5">
            "Loading..."
          </div>}.into_any())}
        {move || value().map(|result| match result {
            Ok(msg) => {
              if msg.is_empty() {
                  ().into_any()
              } else if msg == "ok" {
                view!{<div class="alert alert-success mb-5">
                  "Success!"
                </div>}.into_any()
              } else  {
                view!{<div class="alert alert-danger mb-5">
                  {msg}
                </div>}.into_any()
              }
            },
            Err(e) => view!{<div class="alert alert-danger mb-5">
                {e.to_string()}
              </div>}.into_any()
            })}
      </div>
    }
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
