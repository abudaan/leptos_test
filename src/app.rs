use crate::component::test::Test;
use crate::component::test1::Test1;
use crate::component::test2::Test2;
use crate::component::test3::Test3;
use crate::component::test4::Test4;
use crate::component::test5::Test5;
use crate::component::text_form::TextForm;
use leptos::prelude::*;
use leptos_meta::MetaTags;
use leptos_router::components::*;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                // id=leptos means cargo-leptos will hot-reload this stylesheet
                <link rel="stylesheet" id="leptos" href="/pkg/diabetes-game-admin.css"/>
                <link rel="stylesheet" id="bootstrap" href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css" />
                <link rel="stylesheet" id="bootstrap-icons" href="https://cdn.jsdelivr.net/npm/bootstrap-icons@1.11.3/font/bootstrap-icons.min.css" />
                <MetaTags/>
                <title>Leptos Admin</title>
            </head>
            <body>
                <App/>
                <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/js/bootstrap.bundle.min.js"/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    use crate::component::text_table::TextTable;
    use crate::error_template::{AppError, ErrorTemplate};
    use leptos_router::*;

    view! {
        <main>
        <Router>
            <Routes fallback=|| {
                let mut outside_errors = Errors::default();
                outside_errors.insert_with_default_key(AppError::NotFound);
                view! {
                    <ErrorTemplate outside_errors/>
                }
            }>
                <Route path=path!("") view=HomePage/>
                <Route path=path!("texts") view=TextTable/>
                <ParentRoute path=path!("text-form") view=TextForm>
                    <Route path=path!("/") view=TextForm/>
                    <Route path=path!("/:id") view=TextForm/>
                </ParentRoute>
                <Route path=path!("test") view=Test/>
                <Route path=path!("test1") view=Test1/>
                <Route path=path!("test2") view=Test2 ssr=SsrMode::Async />
                <Route path=path!("test3") view=Test3/>
                <Route path=path!("test4") view=Test4/>
                <Route path=path!("test5") view=Test5/>
            </Routes>
        </Router>
        </main>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <h1>"Admin"</h1>
        <A href="/texts">"Show all texts"</A>
    }
}
