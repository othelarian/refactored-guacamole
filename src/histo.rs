use serde::{Serialize, Deserialize};
use std::rc::Rc;
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew_agent::use_bridge;

use crate::saver::GuacaConfig;
use crate::store::{StoreInput, StoreOutput};

#[derive(Clone, Deserialize, Serialize)]
pub struct HistoLine {
  //
  // TODO
  //
  name: String,
  //
  //
  pub total: usize
}

impl HistoLine {
  pub fn create(name: String, total: usize) -> Self {
    //
    // TODO
    //
    //
    Self {
      //
      name,
      total
    }
  }

  fn view(&self) -> Html {
    //
    // TODO
    //
    html! {
      //
      <div>{"histo line, total "}{self.total}</div>
      //
    }
  }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct HistoResult { lines: Vec<HistoLine> }

impl HistoResult {
  pub fn create(lines: Vec<HistoLine>) -> Self { Self { lines } }

  pub fn one_result(result: HistoLine) -> Self { Self::create(vec!(result)) }

  fn view(&self) -> Html {
    html! {<div>{for self.lines.iter().map(|line| line.view())}</div>}
  }
}

pub enum HistoAction {
  Add(HistoResult),
  Clear,
  Copy,
  Remove(usize),
  SetGuacaLink(Rc<GuacaConfig>)
}

#[derive(Default)]
struct HistoState {
  history: Vec<HistoResult>,
  guaca_config: Option<Rc<GuacaConfig>>
}

impl Reducible for HistoState {
  type Action = HistoAction;

  fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
    let mut history = self.history.clone();
    let mut ngc = None;
    match action {
      HistoAction::Add(new_res) => {
        if let Some(gc) = self.guaca_config.clone() {
          gc.add_history(JsValue::from_serde(&new_res).unwrap());
          ngc = Some(gc);
        }
        history.push(new_res);
      }
      HistoAction::Clear => {
        if let Some(gc) = self.guaca_config.clone() {
          gc.clear_history(); ngc = Some(gc); }
        history.clear();
      }
      HistoAction::Copy => {
        if let Some(gc) = self.guaca_config.clone() {
          gc.copy_history(JsValue::from_serde(&history).unwrap());
          ngc = Some(gc);
        }
      }
      HistoAction::Remove(id) => {
        if let Some(gc) = self.guaca_config.clone() {
          gc.remove_history(id); ngc = Some(gc); }
        history.remove(id);
      }
      HistoAction::SetGuacaLink(guaca_config) => ngc = Some(guaca_config)
    };
    Self { history, guaca_config: ngc }.into()
  }
}

#[function_component(Historic)]
pub fn historic() -> Html {
  let reducer = use_reducer(HistoState::default);
  let init = use_state(|| false);
  {
    let reducer = reducer.clone();
    let init = init.clone();
    let bridge = use_bridge::<crate::store::Store, _>(move |out| {
      match out {
        StoreOutput::HistoryAction(action) => reducer.dispatch(action),
        _ => ()
      }
    });
    if !*init { bridge.send(StoreInput::RegisterHistory); init.set(true); }
  }
  let history: Html = reducer.history.iter().rev().enumerate().map(|(id, res)| {
    let delete_res_cb = {
      let reducer = reducer.clone();
      Callback::from(move |_| reducer.dispatch(HistoAction::Remove(id)))
    };
    html! {
      <div>{res.view()}<button onclick={delete_res_cb}>{"x"}</button></div>
    }
  }).collect();
  let histo_clear_cb = {
    let reducer = reducer.clone();
    Callback::from(move |_| reducer.dispatch(HistoAction::Clear))
  };
  html! {
    <>
      if reducer.history.len() > 0 {
        <div class="guaca-navbar">
          <button onclick={histo_clear_cb}>{"Tout effacer"}</button>
        </div>
        <div class="guaca-list">{history}</div>
      } else {
        <div class="guaca-list" style="text-align:center">
          {"Aucun résultat dans l'historique"}
        </div>
      }
    </>
  }
}
