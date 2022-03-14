use yew::prelude::*;

use crate::{App, AppMessage};

#[derive(PartialEq, Clone)]
pub enum DiceType {
  D4, D6, D8, D10, D20, D100, Custom(usize)
}

#[derive(Properties, PartialEq, Clone)]
pub struct ThrowerConfig {
  nb_dice: usize,
  dice_type: DiceType,
  modifier: isize,
  result: Option<usize>
}

impl ThrowerConfig {
  pub fn default() -> Self { Self::create(1, DiceType::D6, 0) }

  pub fn create(nb_dice: usize, dice_type: DiceType, modifier: isize) -> Self {
    Self {nb_dice, dice_type, modifier, result: None}
  }
}

#[derive(Properties, PartialEq)]
pub struct ThrowerProps {
  pub config: ThrowerConfig,
  pub id: usize
}

pub enum ThrowerMessage {
  NoOp
}

pub struct Thrower {
  config: ThrowerConfig,
  curr_result: Option<usize>,
  id: usize
}

impl Component for Thrower {
  type Message = ThrowerMessage;
  type Properties = ThrowerProps;

  fn create(ctx: &Context<Self>) -> Self {
    Self {
      config: ctx.props().config.clone(),
      curr_result: None,
      id: ctx.props().id
    }
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      //
      //
      Self::Message::NoOp => {
        //
        // TODO: removeNoOp
        //
        ctx.link().get_parent().unwrap().clone().downcast::<App>().send_message(AppMessage::AddThrower);
        //
        false
      }
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    //
    let cbtest = ctx.link().callback(|_| Self::Message::NoOp);
    //
    html! {
      //
      <>
      <button onclick={cbtest}>{"test"}</button>
      //
      {"this is a thrower!"}
      //
      </>
    }
  }
}
