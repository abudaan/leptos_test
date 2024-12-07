use ev::PointerEvent;
use leptos::prelude::*;
use leptos::*;
use leptos_router::hooks::use_params_map;
use leptos_router::params::Params;
use prelude::Read;
use prelude::Suspense;
use serde::Deserialize;
use serde::Serialize;
use server::Resource;
use uuid::Uuid;

use crate::model::text::get_one;
use crate::model::Text;

#[derive(Params, PartialEq, Serialize, Deserialize)]
struct TextParams {
    id: Option<Uuid>,
}

#[component]
pub fn TextForm() -> impl IntoView {
    logging::log!("TextForm");
    // let params = use_params::<TextParams>();
    // let (uuid, set_uuid) = create_signal(None);
    // create_effect(move |_| {
    //     params.with(move |value| match value {
    //         Ok(v) => {
    //             if v.id.is_some() {
    //                 // logging::log!("id: {}", v.id.unwrap().to_string());
    //                 set_uuid(v.id);
    //             }
    //         }
    //         Err(error) => {
    //             logging::log!("error: {}", error.to_string());
    //         }
    //     });
    // });

    let params = use_params_map();
    let id = move || params.try_read().expect("couldn't read params").get("id");

    let text = Resource::new(
        || (),
        move |_| async move {
            if let Some(id) = id() {
                let id = Uuid::parse_str(&id).unwrap();
                match get_one(id).await {
                    Ok(value) => Ok(Some(value)),
                    Err(error) => Err(error.to_string()),
                }
            } else {
                Ok(None)
            }
        },
    );

    let mut title = "".to_string();
    let mut published = true;
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

    view! {
      <Suspense fallback={
        move || view!{<div>"Loading..."</div>}.into_view()
      }>
      {
        move|| {
          text.get().map(|n| {
            if let Ok(Some(text)) = n {
              let title = text.title;
              let published = text.published;
              // logging::log!("{} {}", title, published);
              view!{
                <div>{title}</div>
                <div>{published.to_string()}</div>
              }.into_any()
            } else {
              view!{
                <div>"nothing"</div>
              }.into_any()
            }
        });
      }}
      </Suspense>
    }

    /*
        view! {
        <form class="row g-9" autoComplete="off" noValidate="true">
          <div className="col-md-9">
            <label for="title" class="form-label">Title</label>
            <input type="text" id="title" class="form-control" required="true"
                prop:value="title"
                // on:input=move |ev| {
                //     logging::log!("{:?}", ev);
                //     // set_name(event_target_value(&ev));
                // }
                // on:input:target=move |ev:InputEvent| {
                //     // .value() returns the current value of an HTML input element
                //     // let value = event_target_value(&ev);
                //     // set_name.set(&value);
                //     // let value = std::convert::Into::<HtmlInputElement>::into(ev.target().unwrap());
                //     logging::log!("value {:?}",  ev);
                //     // set_name.set(ev.value());
                // }
            />
            <div className="invalid-feedback">
              Please enter a title
            </div>
          </div>
          // <input type="text" class="form-control" prop:value={name} />


        //   <div className="col-md-9">
        //     <label htmlFor="content" className="form-label">Content</label>
        //     <textarea id="content" value="" required="true" className="form-control" rows="10"
        //         on:change:target=move |ev: change|{}
        //     />
        //     <div className="invalid-feedback">
        //       Please enter content
        //     </div>
        //   </div>

        //   <div className="col-md-9">
        //     <input type="checkbox" id="published" checked="false" className="ml-2" onChange={} />
        //     <label htmlFor="published" className="form-label mx-1">Published</label>
        //   </div>

        //   {state.route.name === 'admin_text_edit' &&
        //     <div className="col-12">
        //       <button className="btn btn-outline-danger me-2" type="button"
        //         data-bs-toggle="modal" data-bs-target="#deleteTextModal">
        //         Delete text
        //       </button>
        //     </div>}

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
    */
}
