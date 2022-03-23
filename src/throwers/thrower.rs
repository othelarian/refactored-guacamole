use log::info;
use std::rc::Rc;
use web_sys::Element;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::store::{Store, StoreInput, StoreOutput};
use super::config::{DiceMethod, DiceType};
use super::ThrowerConfig;

pub enum ThrowerMsg {
  Delete,
  InitConfig(Rc<Vec<ThrowerConfig>>),
  NoOp,
  Roll,
  ToggleMethod
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
        let mut new_config = (*self.config).get(ctx.props().index).unwrap();
        //
        let res = new_config.roll();
        //
        //
        //
        self.ref_result.cast::<Element>().unwrap()
          .set_inner_html(&res.total.to_string());
        //
        false
      }
      ThrowerMsg::ToggleMethod => {
        let mut new_config = (*self.config).get(ctx.props().index).unwrap().clone();
        new_config.method = match new_config.method {
          DiceMethod::Each => DiceMethod::Total,
          DiceMethod::Total => DiceMethod::Each
        };
        self.store.send(StoreInput::UpdateConfig(ctx.props().index, new_config));
        false
      }
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let index = ctx.props().index;
    //
    info!("rendering thrower {}", index);
    //
    let config = match self.config.get(index) {
      Some(config) => config.clone(),
      None => self.config.get(0).unwrap().clone()
    };
    let len = self.config.len();
    //
    // details
    //
    let method_data = ThrowerConfig::method(&config.method);
    //
    let placeholder_data = config.placeholder();
    //
    let result_data = match config.result {
      Some(result) => result.to_string(),
      None => String::from("Roll")
    };
    //
    // callbacks
    //
    let delete_thrower_cb = ctx.link().callback(|_| Self::Message::Delete);
    //
    let roll_cb = ctx.link().callback(|_| Self::Message::Roll);
    //
    let toggle_method_cb = ctx.link().callback(|_| Self::Message::ToggleMethod);
    //
    // rendering thrower
    html! {
      <div class="guaca-line">
        <div class="guaca-controls">
          //
          // TODO: le bouton pour la sélection partielle
          //
          <label class="checky">
            <input type="checkbox" />
            <span></span>
          </label>
          //
          <button onclick={delete_thrower_cb}>{"x"}</button>
        </div>
        <div class="guaca-thrower">
          //
          // TODO: connecter l'input à un onchange
          //
          <input
            class="guaca-title"
            placeholder={placeholder_data}
            text={config.name} />
          <br />
          //
          // TODO: finir le theming du lanceur
          //
          // TODO: finir les inputs du lanceur
          //
          <input class="guaca-number" type="number" min="1" />
          //
          {"(dice type"}
          //
          <input class="guaca-number guaca-custom" type="number" min="2" />
          //
          {" "}
          //
          <input class="guaca-number" type="number" />
          //
          <button onclick={toggle_method_cb}>{method_data}</button>
          //
        </div>
        <div class="guaca-result">
          <button ref={self.ref_result.clone()} onclick={roll_cb}>{result_data}</button>
        </div>
      </div>
    }
  }
}

impl Thrower {
  fn type_selector(&self, ctx: &Context<Self>) -> Html {
    //
    let select_type_cb = ctx.link().callback(|_| {
      //
      // TODO
      info!("dice selection");
      //
      ThrowerMsg::NoOp
      //
    });
    //
    //
    html! {
      <select onchange={select_type_cb}>
        //
        //
        //
      </select>
    }
  }
}
