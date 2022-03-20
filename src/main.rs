use yew::prelude::*;
use yew_agent::use_bridge;

mod histo;
mod throwers;
mod store;

use histo::Historic;
use throwers::ThrowerList;
use store::{Store, StoreInput};

#[function_component(App)]
fn app() -> Html {
  // preparation
  let store = use_bridge::<Store, _>(|_| ());
  // callbacks
  let histo_clear_cb = {
    let store = store.clone();
    Callback::from(move |_| store.send(StoreInput::ClearHistory))
  };
  let thrower_add_cb = {
    let store = store.clone();
    Callback::from(move |_| store.send(StoreInput::ThrowAdd))
  };
  let thrower_clear_cb = {
    let store = store.clone();
    Callback::from(move |_| store.send(StoreInput::ClearThrowers))
  };
  // rendering
  html! {
    <div class="guaca-container">
      <div class="guaca-block guaca-throwers">
        <ThrowerList />
        <div class="guaca-navbar">
          <button onclick={thrower_clear_cb}>{"Tout supprimer"}</button>
          <button onclick={thrower_add_cb}>{"Ajouter"}</button>
        </div>
      </div>
      <div class="guaca-block guaca-histo">
        <div class="guaca-navbar">
          <button onclick={histo_clear_cb}>{"Tout effacer"}</button>
        </div>
        <div class="guaca-list"><Historic /></div>
      </div>
    </div>
  }
}

fn main() {
  console_log::init().unwrap();
  yew::start_app::<App>();
}
