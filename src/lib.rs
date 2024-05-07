use wasm_bindgen::prelude::wasm_bindgen;

mod settings;
mod board;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  pub fn debug(s: &str);

  #[wasm_bindgen(js_namespace = Math)]
  ///
  /// Returns a random number that's greater than or equal to 0 and less than 1.
  /// 
  pub fn random() -> f64;
}
