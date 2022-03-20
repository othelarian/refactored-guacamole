use log::info;
use std::rc::Rc;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::store::{Store, StoreInput, StoreOutput};
use super::ThrowerConfig;

pub enum ThrowerMsg {
  Delete,
  InitConfig(Rc<Vec<ThrowerConfig>>),
  NoOp,
  Roll
}

#[derive(Properties, PartialEq)]
pub struct ThrowerProperties {
  pub index: usize
}

pub struct Thrower {
  config: Rc<Vec<ThrowerConfig>>,
  ref_result: NodeRef,
  store: Box<dyn Bridge<Store>>
}

impl Component for Thrower {
  type Message = ThrowerMsg;
  type Properties = ThrowerProperties;

  fn create(ctx: &Context<Self>) -> Self {
    let mut store = Store::bridge(ctx.link().callback(|res| match res {
      StoreOutput::InitThrower(config) => ThrowerMsg::InitConfig(config),
      _ => ThrowerMsg::NoOp
    }));
    store.send(StoreInput::RegisterThrower);
    Self {
      config: Rc::new(vec!(ThrowerConfig::default())),
      ref_result: NodeRef::default(),
      store
    }
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      ThrowerMsg::Delete => {
        self.store.send(StoreInput::DeleteThrower(ctx.props().index));
        false
      }
      ThrowerMsg::InitConfig(config) => { self.config = config; true }
      ThrowerMsg::NoOp => false,
      ThrowerMsg::Roll => {
        //
        // TODO
        //
        false
      }
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    //
    info!("rendering thrower");
    //
    let index = ctx.props().index;
    let config = match self.config.get(index) {
      Some(config) => config.clone(),
      None => self.config.get(0).unwrap().clone()
    };
    //
    //
    // callbacks
    //
    let delete_thrower_cb = ctx.link().callback(|_| Self::Message::Delete);
    //
    let roll_cb = ctx.link().callback(|_| Self::Message::Roll);
    //
    //
    // rendering thrower
    html! {
      <div class="guaca-line">
        <div class="guaca-controls">
          //
          // TODO: le bouton pour la sélection partielle
          //
          <button onclick={delete_thrower_cb}>{"-"}</button>
        </div>
        <div class="guaca-thrower">
          //
          // TODO: refaire le setting du dnd diceroller
          <input class="guaca-title" placeholder="(title)" />
          //
          <br />
          //
          <input class="guaca-number" type="number" />
          //
          {"(dice type"}
          //
          <input class="guaca-number" type="number" />
          //
          <button>{"(tog tot)"}</button>
          //
        </div>
        <div class="guaca-result">
          <button ref={self.ref_result.clone()} onclick={roll_cb}>{"---"}</button>
        </div>
      </div>
    }
  }
}
