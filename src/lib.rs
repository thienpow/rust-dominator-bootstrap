use wasm_bindgen::prelude::*;
use crate::app::App;

pub mod utils;
mod app;
mod routing;
mod navbar;
mod footer;
mod page;

#[macro_use]
mod macros;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let app = App::new();

    dominator::append_dom(&dominator::body(), App::render(app));

    Ok(())
}