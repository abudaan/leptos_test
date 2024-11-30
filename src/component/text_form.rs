use ev::Event;
use ev::PointerEvent;
use leptos::ev::change;
use leptos::ev::InputEvent;
use leptos::*;
use web_sys::HtmlElement;
use web_sys::HtmlInputElement;

#[component]
pub fn TextForm() -> impl IntoView {
    let (name, set_name) = create_signal("daan".to_string());
    logging::log!("TextForm");

    view! {
    <form class="row g-9" autoComplete="off" noValidate="true">
      <div className="col-md-9">
        <label for="title" class="form-label">Title</label>
        <input type="text" id="title" class="form-control" required="true"
            prop:value={name}
            on:input=move |ev| {
                logging::log!("{:?}", ev);
                set_name(event_target_value(&ev));
            }
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
      <input type="text" class="form-control" prop:value={name} />


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

      <div className="col-12 pt-3 mb-5">
        <button type="button" className="btn btn-primary me-2" on:click:target= move |ev: PointerEvent|{
            // logging::log!("click {}", ev.target().unwrap().as_ref::<HtmlInputElement>());
            logging::log!("click {}", ev.target().unwrap().to_string());
        }>
          Save
        </button>
        <button type="button" className="btn btn-outline-danger"
            on:click:target= move |ev: PointerEvent|{
            // logging::log!("click {}", ev.target().unwrap().as_ref::<HtmlInputElement>());
            logging::log!("click {}", ev.target().unwrap().to_string());
        }
        >Cancel
        </button>
      </div>
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
}
