use std::rc::Rc;
use yew::prelude::*;
use yew_agent::use_bridge;

use crate::store::StoreOutput;

#[derive(Clone)]
pub struct HistoResult {
  //
  //
  test1: usize
  //
}

impl HistoResult {
  pub fn create(test1: usize) -> Self {
    //
    //
    HistoResult {
      //
      test1
      //
    }
  }

  pub fn view(&self) -> Html {
    //
    //
    html! {
      <div>
        {"histo: "}{self.test1}
      </div>
    }
  }
}

pub enum HistoAction {
  Add(HistoResult),
  Clear,
  Remove(usize)
}

#[derive(Default)]
struct HistoState {
  history: Vec<HistoResult>
}

impl Reducible for HistoState {
  type Action = HistoAction;

  fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
    let mut history = self.history.clone();
    match action {
      HistoAction::Add(new_res) => history.push(new_res),
      HistoAction::Clear => history.clear(),
      HistoAction::Remove(id) => { history.remove(id); },
    };
    Self { history }.into()
  }
}

#[function_component(Historic)]
pub fn historic() -> Html {
  let reducer = use_reducer(HistoState::default);
  {
    let reducer = reducer.clone();
    let bridge = use_bridge::<crate::store::Store, _>(move |out| match out {
      StoreOutput::HistoryAction(action) => { reducer.dispatch(action); }
      _ => ()
    });
    bridge.send(crate::store::StoreInput::RegisterHistory);
  };
  let history: Html = reducer.history.iter().rev().enumerate().map(|(id, res)| {
    let delete_res_cb = {
      let reducer = reducer.clone();
      Callback::from(move |_| reducer.dispatch(HistoAction::Remove(id)))
    };
    html! {
      <div>
        {res.view()}
        <button onclick={delete_res_cb}>{"x"}</button>
      </div>
    }
  }).collect();
  html! {history}
}
