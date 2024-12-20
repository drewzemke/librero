pub mod app;
pub mod model;

#[cfg(feature = "ssr")]
pub mod router;

#[cfg(feature = "ssr")]
pub mod testing;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
