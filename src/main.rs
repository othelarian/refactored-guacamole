use yew::prelude::*;
use yew_agent::use_bridge;

mod histo;
mod throwers;
pub mod saver;
mod store;
mod veil;

use histo::Historic;
use throwers::ThrowerList;
use store::{Store, StoreInput};
use veil::{Veil, VeilShow};

#[function_component(App)]
fn app() -> Html {
  // preparation
  let store = { use_bridge::<Store, _>(|_| ()) };
  // callbacks
  let thrower_add_cb = {
    let store = store.clone();
    Callback::from(move |_| store.send(StoreInput::ThrowAdd))
  };
  let thrower_clear_cb = {
    let store = store.clone();
    Callback::from(move |_| store.send(StoreInput::ClearThrowers))
  };
  let call_veil_cb = {
    let store = store.clone();
    Callback::from(move |_| store.send(StoreInput::ToggleVeil(true, VeilShow::Options)))
  };
  let copy_cb = {
    let store = store.clone();
    Callback::from(move |_| store.send(StoreInput::CopyUrl))
  };
  // rendering
  html! {
    <>
      <h1>{"Refactored Guacamole"}</h1>
      <h2>
        {"Le lanceur de dés, copie personnelle de "}
        <a href="https://www.dnddiceroller.com/">{"dnddiceroller.com"}</a>
      </h2>
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
            <button onclick={call_veil_cb}>{"options"}</button>
            <button onclick={copy_cb}>{"copier la config"}</button>
          </div>
          <hr />
          <h3>{"Historique"}</h3>
          <Historic />
        </div>
      </div>
      <Veil />
    </>
  }
}

fn main() {
  console_log::init().unwrap();
  yew::start_app::<App>();
}
