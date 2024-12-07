pub mod app;
pub mod component;
pub mod database;
pub mod error_template;
pub mod model;

// #[cfg(feature = "hydrate")]
// #[wasm_bindgen::prelude::wasm_bindgen]
// pub fn hydrate() {
//     use crate::app::*;
//     console_error_panic_hook::set_once();
//     mount_to_body(App);
// }

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
