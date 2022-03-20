use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::store::{Store, StoreInput, StoreOutput};
use super::Thrower;

pub enum ThrowerListMsg {
  NoOp,
  RefreshList(usize),
  ThrowAll,
  ThrowSelected
}

pub struct ThrowerList {
  config_len: usize,
  store: Box<dyn Bridge<Store>>,
  ref_selbox: NodeRef,
  ref_selroll: NodeRef
}

impl Component for ThrowerList {
  type Message = ThrowerListMsg;
  type Properties = ();

  fn create(ctx: &Context<Self>) -> Self {
    let mut store = Store::bridge(ctx.link().callback(|res| match res {
      StoreOutput::UpdateThrowerList(len) => ThrowerListMsg::RefreshList(len),
      _ => ThrowerListMsg::NoOp
    }));
    store.send(StoreInput::GetLength);
    //
    // TODO: récupération probable de Location ici
    //
    Self {
      config_len: 0,
      store,
      ref_selbox: NodeRef::default(),
      ref_selroll: NodeRef::default()
    }
  }

  fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
    if first_render {
      //
      // TODO: ajout de la première config par défaut, ou récupération depuis App (url) ?
      //
    } else {
      //
      // TODO: pour l'état "indeterminate" de selector toggle, mieux vaut peut-être passer par ici
      //
    }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      ThrowerListMsg::NoOp => false,
      ThrowerListMsg::RefreshList(len) =>
        { self.config_len = len; true }
      ThrowerListMsg::ThrowAll =>
        { self.store.send(StoreInput::ThrowAll); false }
      ThrowerListMsg::ThrowSelected =>
        { self.store.send(StoreInput::ThrowSelected); false }
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    // preparation
    let throwers: Html = (0..self.config_len).map(|id| {
      html! { <Thrower index={id} /> }
    }).collect();
    // styles
    //
    // TODO
    let display_selroll = "visibility:visible";
    //
    // callbacks
    let roll_all_cb = ctx.link().callback(|_| ThrowerListMsg::ThrowAll);
    let roll_selected_cb = ctx.link().callback(|_| ThrowerListMsg::ThrowSelected);
    // rendering
    html! {
      <div>
        {throwers}
        if self.config_len > 1 {
          <div class="guaca-controls">
            //
            // TODO: checkbox 3 states
            //
            <label></label>
            //
          </div>
          <div class="guaca-selector">
            <button
              style={display_selroll}
              ref={self.ref_selroll.clone()}
              onclick={roll_selected_cb}
            >{"Lancer la sélection"}</button>
          </div>
          <div class="guaca-result">
            <button onclick={roll_all_cb}>{"Tout lancer"}</button>
          </div>
        } 
      </div>
    }
  }
}
