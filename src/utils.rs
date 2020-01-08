use wasm_bindgen::UnwrapThrowExt;

pub fn window() -> web_sys::Window {
  web_sys::window().unwrap_throw()
}

pub fn hash() -> Option<String> {
  window()
      .location()
      .hash()
      .ok()
      .and_then(|h| if h.is_empty() { None } else { Some(h) })
}

pub fn set_hash(hash: &str) {
  window().location().set_hash(hash).unwrap_throw();
}

pub fn document() -> web_sys::Document {
  window().document().unwrap_throw()
}

pub fn local_storage() -> web_sys::Storage {
  window().local_storage().unwrap_throw().unwrap_throw()
}