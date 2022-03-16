use log::info;
use std::rc::Rc;
use yew::prelude::*;
use yewdux::prelude::*;

use super::config::ThrowerConfig;

#[derive(Clone, Default)]
pub struct ThrowerStore {
  pub configs: Vec<ThrowerConfig>
}

pub enum ThrowerListMsg {
  RollAll,
  RollSelected,
  SelectToggle,
  State(Rc<ThrowerStore>)
}

pub struct ThrowerList {
  dispatch: Dispatch<BasicStore<ThrowerStore>>,
  state: Rc<ThrowerStore>,
  
  ref_selbox: NodeRef,
  ref_selroll: NodeRef
}

impl Component for ThrowerList {
  type Message = ThrowerListMsg;
  type Properties = ();

  fn create(ctx: &Context<Self>) -> Self {
    //
    // TODO: récupération probable de Location ici
    //
    Self {
      dispatch: Dispatch::bridge_state(ctx.link().callback(ThrowerListMsg::State)),
      state: Default::default(),
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
      ThrowerListMsg::RollAll => {
        //
        // TODO
        //
        false
      }
      ThrowerListMsg::RollSelected => {
        //
        // TODO
        //
        false
      }
      ThrowerListMsg::SelectToggle => {
        //
        // TODO
        //
        false
      }
      ThrowerListMsg::State(state) => {
        //
        info!("set state");
        //
        let output = self.state.configs.len() != state.configs.len();
        self.state = state;
        output
      }
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    // throwers
    let throwers: Html = self.state.configs.iter()
      .enumerate()
      .map(|(id, config)| {
        //
        //
        html! {
          //
          <div>{"a thrower!"}</div>
          //
        }
      })
      .collect();
    // states
    //
    // TODO: etat pour la checkbox
    //
    // TODO: display_selroll doit être conditionné
    let display_selroll = "visibility:visible";
    //
    //
    // callbacks
    let selector_cb = ctx.link().callback(|_| ThrowerListMsg::SelectToggle);
    let roll_all_cb = ctx.link().callback(|_| ThrowerListMsg::RollAll);
    let roll_selected_cb = ctx.link().callback(|_| ThrowerListMsg::RollSelected);
    // rendering
    html! {
      <div>
        {throwers}
        if (self.state.configs.len() > 1) {
          <div class="guaca-thrower-c1">
            //
            // TODO: checkbox avec 3 états
            //
            <label onclick={selector_cb}>{"(C)"}</label>
            //
          </div>
          <div class="guaca-thrower-c2">
            <button
              style={display_selroll}
              ref={self.ref_selroll.clone()}
              onclick={roll_selected_cb}
            >{"Lancer la sélection"}</button>
          </div>
          <div class="guaca-thrower-c3">
            <button onclick={roll_all_cb}>{"Tout lancer"}</button>
          </div>
        }
      </div>
    }
  }
}
