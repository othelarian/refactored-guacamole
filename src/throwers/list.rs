use std::rc::Rc;
use yew::prelude::*;
use yewdux::prelude::*;

use super::config::ThrowerConfig;

#[derive(Clone, Default)]
pub struct ThrowerStore {
  configs: Vec<ThrowerConfig>
}

pub struct ThrowerList {
  dispatch: Dispatch<BasicStore<ThrowerStore>>,
  state: Rc<ThrowerStore>
}

impl Component for ThrowerList {
  type Message = ();
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self {
      dispatch: Dispatch::default(),
      state: Default::default()
    }
  }

  fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
    if first_render {
      //
      // TODO: ajout de la première config par défaut, ou récupération depuis App (url) ?
      //
    }
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    //
    // TODO
    //
    /*
    let selected_toggle_cb = ctx.link().callback(|_| Self::Message::ToggleSelected);
    let roll_selected_cb = ctx.link().callback(|_| Self::Message::RollSelected);
    let roll_all_cb = ctx.link().callback(|_| Self::Message::RollAll);
    */
    //
    html! {
      <div>
        {"list des lanceurs"}
        //
        //
        /*
          if self.throwers.len() > 1 {
            //
            // TODO: un peu de theming est nécessaire ici
            //
            <input type="checkbox" onchange={selected_toggle_cb} />
            //
            <button onclick={roll_selected_cb}>{"Lancer la sélection"}</button>
            <button onclick={roll_all_cb}>{"Tout lancer"}</button>
            //
          }
        */
      </div>
    }
    //
  }
}
