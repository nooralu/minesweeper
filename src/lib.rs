use wasm_bindgen::prelude::wasm_bindgen;

mod settings;
mod board;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  pub fn debug(s: &str);
}
