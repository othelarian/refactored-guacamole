use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::histo::HistoResult;
use crate::store::{
  Store, StoreInput, StoreOutput,
  ConfigHash, create_config_hash
};
use super::config::{DiceMethod, DiceType};
use super::ThrowerConfig;

pub enum InputUpdated {
  Modifier,
  NbCustom,
  NbDice
}

pub enum ThrowerMsg {
  ChangeDiceType(DiceType),
  Delete,
  InitConfig(ConfigHash),
  NoOp,
  Refresh,
  Roll,
  ToggleMethod,
  ToggleSelectRoll(bool),
  UpdateInput(InputUpdated, isize),
  UpdateName(String),
  UpdateRes(isize),
  UpdateSelbox(bool)
}

#[derive(Properties, PartialEq)]
pub struct ThrowerProperties {
  pub index: usize
}

pub struct Thrower {
  config: ConfigHash,
  init: bool,
  ref_result: NodeRef,
  ref_select_roll: NodeRef,
  result: Option<isize>,
  store: Box<dyn Bridge<Store>>
}

impl Component for Thrower {
  type Message = ThrowerMsg;
  type Properties = ThrowerProperties;

  fn create(ctx: &Context<Self>) -> Self {
    let mut store = Store::bridge(ctx.link().callback(|res| match res {
      StoreOutput::InitThrower(config) => ThrowerMsg::InitConfig(config),
      StoreOutput::ForceRefresh => ThrowerMsg::Refresh,
      StoreOutput::ToggleSelCheck(state) =>
        ThrowerMsg::UpdateSelbox(state),
      StoreOutput::UpdateRollRes(res) => ThrowerMsg::UpdateRes(res),
      _ => ThrowerMsg::NoOp
    }));
    store.send(StoreInput::RegisterThrower(ctx.props().index));
    let config = create_config_hash();
    config.borrow_mut().insert(0, ThrowerConfig::default());
    Self {
      config, init: false,
      ref_result: NodeRef::default(), ref_select_roll: NodeRef::default(),
      result: None, store
    }
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      ThrowerMsg::ChangeDiceType(mut new_type) => {
        if let DiceType::D(_) = new_type { new_type = DiceType::D(42); }
        let index = ctx.props().index;
        let mut cfgs = self.config.borrow_mut();
        cfgs.get_mut(&index).unwrap().dice_type = new_type;
        self.store.send(StoreInput::UpdateConfig(index));
        true
      }
      ThrowerMsg::Delete => {
        self.store.send(StoreInput::DeleteThrower(ctx.props().index));
        false
      }
      ThrowerMsg::InitConfig(config) => {
        self.config = config;
        self.init = true;
        true
      }
      ThrowerMsg::NoOp => false,
      ThrowerMsg::Refresh => true,
      ThrowerMsg::Roll => {
        let res = self.config.try_borrow().unwrap()
          .get(&ctx.props().index).unwrap().roll();
        self.result = Some(res.total);
        self.ref_result.cast::<Element>().unwrap()
          .set_inner_html(&res.total.to_string());
        self.store.send(StoreInput::AddHistory(HistoResult::one_result(res)));
        false
      }
      ThrowerMsg::ToggleMethod => {
        let index = ctx.props().index;
        let mut cfgs = self.config.borrow_mut();
        let mut config = cfgs.get_mut(&index).unwrap();
        config.method = match config.method {
          DiceMethod::Each => DiceMethod::Total,
          DiceMethod::Total => DiceMethod::Each
        };
        self.store.send(StoreInput::UpdateConfig(index));
        true
      }
      ThrowerMsg::ToggleSelectRoll(state) => {
        let mut cfgs = self.config.borrow_mut();
        let mut config = cfgs.get_mut(&ctx.props().index).unwrap();
        config.selected = state;
        self.store.send(StoreInput::UpdateSelbox(ctx.props().index));
        false
      }
      ThrowerMsg::UpdateInput(inp, new_val) => {
        let index = ctx.props().index;
        let mut cfgs = self.config.borrow_mut();
        let mut config = cfgs.get_mut(&index).unwrap();
        match inp {
          InputUpdated::Modifier => config.modifier = new_val,
          InputUpdated::NbCustom =>
            config.dice_type = DiceType::D(new_val),
          InputUpdated::NbDice => config.nb_dice = new_val as usize
        }
        self.store.send(StoreInput::UpdateConfig(index));
        true
      }
      ThrowerMsg::UpdateName(new_name) => {
        let index = ctx.props().index;
        self.config.borrow_mut()
          .entry(index).or_default().name = new_name;
        self.store.send(StoreInput::UpdateName(index));
        false
      }
      ThrowerMsg::UpdateRes(new_res) => {
        self.ref_result.cast::<Element>().unwrap()
          .set_inner_html(&new_res.to_string());
        self.result = Some(new_res);
        false
      }
      ThrowerMsg::UpdateSelbox(new_val) => {
        self.ref_select_roll.cast::<HtmlInputElement>().unwrap()
          .set_checked(new_val);
        false
      }
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let cfgs = self.config.borrow();
    let config =
      if self.init { cfgs.get(&ctx.props().index).unwrap() }
      else { cfgs.get(&0).unwrap() };
    // details
    let (dcustom_data, dcustom_style) =
      if let DiceType::D(v) = &config.dice_type {
        (v.to_string(), "display:inline-block;")
      }
      else { (String::from("42"), "display:none;") };
    let method_data = ThrowerConfig::method(&config.method);
    let modifier_data = config.modifier.to_string();
    let nb_dice_data = config.nb_dice.to_string();
    let placeholder_data = config.placeholder();
    let result_data = match self.result {
      Some(result) => result.to_string(),
      None => String::from("Roll")
    };
    let select_roll_data =
      if cfgs.len() == 1 { "display:none;" }
      else { "display:inline-block;" };
    // callbacks
    let dcustom_cb = ctx.link().callback(|ev: Event| {
      match ev.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()) {
        Some(input) => Self::Message::UpdateInput(
          InputUpdated::NbCustom,
          input.value().parse::<usize>().unwrap_or_else(|_| 42) as isize
        ),
        None => Self::Message::NoOp
      }
    });
    let delete_thrower_cb = ctx.link().callback(|_| Self::Message::Delete);
    let modifier_cb = ctx.link().callback(|ev: Event| {
      match ev.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()) {
        Some(input) => Self::Message::UpdateInput(
          InputUpdated::Modifier,
          input.value().parse::<isize>().unwrap_or_else(|_| 0)
        ),
        None => Self::Message::NoOp
      }
    });
    let name_cb = ctx.link().callback(|ev: Event| {
      match ev.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()) {
        Some(input) => Self::Message::UpdateName(input.value()),
        None => Self::Message::NoOp
      }
    });
    let nb_dice_cb = ctx.link().callback(|ev: Event| {
      match ev.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()) {
        Some(input) => Self::Message::UpdateInput(
          InputUpdated::NbDice,
          input.value().parse::<usize>().unwrap_or_else(|_| 1) as isize
        ),
        None => Self::Message::NoOp
      }
    });
    let roll_cb = ctx.link().callback(|_| Self::Message::Roll);
    let select_roll_cb = ctx.link().callback(|ev: Event| {
      match ev.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()) {
        Some(input) =>
          Self::Message::ToggleSelectRoll(input.checked()),
        None => Self::Message::NoOp
      }
    });
    let toggle_method_cb = ctx.link().callback(|_| Self::Message::ToggleMethod);
    // rendering thrower
    html! {
      <>
        <div class="guaca-controls">
          <label class="checky" style={select_roll_data}>
            <input type="checkbox" onchange={select_roll_cb}
              ref={self.ref_select_roll.clone()} checked={config.selected} />
            <span></span>
          </label>
          <button onclick={delete_thrower_cb}>{"x"}</button>
        </div>
        <div class="guaca-thrower">
          <input
            class="guaca-title" placeholder={placeholder_data}
            maxlength="30" onchange={name_cb}
            value={config.name.clone()} />
          <br />
          <input class="guaca-number" type="number" min="1"
            onchange={nb_dice_cb} value={nb_dice_data} />
          {self.type_selector(ctx, config.dice_type.clone())}
          <input
            class="guaca-number guaca-custom" type="number" min="2"
            onchange={dcustom_cb} style={dcustom_style}
            value={dcustom_data} />
          {" "}
          <span style="display:inline-block;">
            <input class="guaca-number" type="number"
              onchange={modifier_cb} value={modifier_data} />
            <button onclick={toggle_method_cb}>{method_data}</button>
          </span>
        </div>
        <div class="guaca-result">
          <button ref={self.ref_result.clone()} onclick={roll_cb}>
            {result_data}
          </button>
        </div>
      </>
    }
  }
}

impl Thrower {
  fn type_selector(&self, ctx: &Context<Self>, curr_type: DiceType) -> Html {
    use std::str::FromStr;
    use strum::{IntoEnumIterator};
    let select_type_cb = ctx.link().callback(|ev: Event| {
      match ev.target().and_then(|t| t.dyn_into::<HtmlSelectElement>().ok()) {
        Some(input) => ThrowerMsg::ChangeDiceType(
          DiceType::from_str(&input.value()).unwrap()),
        None => ThrowerMsg::NoOp
      }
    });
    let selects: Html = DiceType::iter().map(|tp| html! {
      <option value={tp.to_string()} selected={curr_type == tp}>
        {tp.to_name()}</option>
    }).collect();
    html! {
      <select onchange={select_type_cb}>{selects}</select>
    }
  }
}
