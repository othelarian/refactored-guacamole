use wasm_bindgen::prelude::*;

#[wasm_bindgen(module="/saver.js")]
extern "C" {
  pub fn initiate_storage() -> bool;
}

pub fn js_init() {
  //
  //initiate_storage();
}
