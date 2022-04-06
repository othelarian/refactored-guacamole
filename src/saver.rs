use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module="/saver.js")]
extern "C" {
  pub type GuacaConfig;

  // config's config interface

  #[wasm_bindgen(constructor)]
  pub fn new() -> GuacaConfig;

  #[wasm_bindgen(method)]
  pub fn islsa(this: &GuacaConfig) -> bool;

  #[wasm_bindgen(method)]
  pub fn isurl(this: &GuacaConfig) -> bool;

  #[wasm_bindgen(method)]
  pub fn toggle_ls(this: &GuacaConfig, toggle: bool) -> bool;

  // configs interface

  #[wasm_bindgen(method)]
  pub fn has_config(this: &GuacaConfig) -> JsValue;

  #[wasm_bindgen(method)]
  pub fn update_config(this: &GuacaConfig, cfgs: JsValue);

  #[wasm_bindgen(method)]
  pub fn update_names(this: &GuacaConfig, names: JsValue);  

  #[wasm_bindgen(method)]
  pub fn clear_config(this: &GuacaConfig);

  // history interface

  #[wasm_bindgen(method)]
  pub fn add_history(this: &GuacaConfig, new_res: JsValue);

  #[wasm_bindgen(method)]
  pub fn clear_history(this: &GuacaConfig);

  #[wasm_bindgen(method)]
  pub fn copy_history(this: &GuacaConfig, history: JsValue);

  #[wasm_bindgen(method)]
  pub fn remove_history(this: &GuacaConfig, idx: usize);
}

use crate::histo::HistoResult;

#[derive(Deserialize, Serialize)]
struct HasConfigRes {
  pub has: bool,
  pub url: bool,
  pub cfgs: Vec<String>,
  pub names: Option<Vec<String>>,
  pub history: Option<Vec<HistoResult>>
}

pub fn parse_init(config: JsValue)
-> Option<(bool, Vec<String>, Option<Vec<String>>, Option<Vec<HistoResult>>)> {
  let config: HasConfigRes = config.into_serde().unwrap();
  if config.has {
    Some((config.url, config.cfgs, config.names, config.history)) }
  else { None }
}
