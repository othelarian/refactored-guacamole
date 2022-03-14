use log::info;
use yew::prelude::*;

mod histo;
use histo::HistoResult;

mod throwers;
use throwers::{Thrower, ThrowerConfig};

pub enum AppMessage {
  AddThrower,
  ClearHisto,
  DeleteThrowers,
  UpdateThrower(usize, ThrowerConfig)
}

struct App {
  history: Vec<HistoResult>,
  throwers: Vec<ThrowerConfig>
}

impl Component for App {
  type Message = AppMessage;
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self {
      history: Vec::default(),
      throwers: vec!(ThrowerConfig::default())
    }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Self::Message::AddThrower => {
        //
        // TODO
        //
        info!("add thrower (not impl)");
        //
        true
      }
      Self::Message::ClearHisto => { self.history = Vec::default(); true }
      Self::Message::DeleteThrowers => { self.throwers = Vec::default(); true }
      Self::Message::UpdateThrower(id, new_config) => {
        //
        // TODO
        //
        info!("update a thrower (not impl)");
        //
        //
        false
      }
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    // get throwers
    let throwers_list: Html = self.throwers.iter()
      .enumerate()
      .map(move |(id, thrower)| html! { <Thrower id={id} config={thrower.clone()} /> })
      .collect();
    // get result history
    //
    let historic: Html = self.history.iter()
      .enumerate()
      .map(move |(id, res)| self.view_history_result(ctx, res.clone(), id))
      .collect();
    //
    //
    //
    let histo_clear_cb = ctx.link().callback(|_| Self::Message::ClearHisto);
    //
    // Render everything
    html! {
      <div class="guaca-container">
        <div class="guaca-block guaca-throwers">
          {self.view_thrower_navbar(ctx)}
          //
          // TODO: add the throwers list here
          //
          {throwers_list}
          //
          // TODO: "all" and "selection" buttons line
          //
          if self.throwers.len() > 0 { {self.view_thrower_navbar(ctx)} }
        </div>
        <div class="guaca-block guaca-histo">
          <div class="guaca-navbar">
            <button onclick={histo_clear_cb}>{"tout effacer"}</button>
          </div>
          <div class="guaca-list">
            //
            // TODO: fill the history list
            //
          </div>
        </div>
      </div>
    }
  }
}

impl App {
  fn view_history_result(&self, ctx: &Context<Self>, res: HistoResult, id: usize) -> Html {
    //
    // TODO: create callback
    //
    //
    html! {
      <div class="">
        {res.view()}
        <button>{"X"}</button>
      </div>
    }
  }

  fn view_thrower_navbar(&self, ctx: &Context<Self>) -> Html {
    let thrower_add_cb = ctx.link().callback(|_| AppMessage::AddThrower);
    let thrower_delete_cb = ctx.link().callback(|_| AppMessage::DeleteThrowers);
    html! {
      <div class="guaca-navbar">
        <button onclick={thrower_delete_cb}>{"Tout supprimer"}</button>
        <button onclick={thrower_add_cb}>{"Ajouter"}</button>
      </div>
    }
  }
}

fn main() {
  console_log::init().unwrap();
  yew::start_app::<App>();
}
