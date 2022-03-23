use yew::prelude::*;
use yew_agent::use_bridge;

mod histo;
mod throwers;
mod store;

use histo::Historic;
use throwers::ThrowerList;
use store::{Store, StoreInput, StoreOutput};

#[function_component(App)]
fn app() -> Html {
  // preparation
  let config = use_state(|| false);
  let store = {
    let config = config.clone();
    use_bridge::<Store, _>(move |out| match out {
      StoreOutput::ConfigState(next_state) => config.set(next_state),
      _ => ()
    })
  };
  store.send(StoreInput::GetConfig);
  // callbacks
  let thrower_add_cb = {
    let store = store.clone();
    Callback::from(move |_| store.send(StoreInput::ThrowAdd))
  };
  let thrower_clear_cb = {
    let store = store.clone();
    Callback::from(move |_| store.send(StoreInput::ClearThrowers))
  };
  let config_cb = {
    let store = store.clone();
    Callback::from(move |_| store.send(StoreInput::ToggleConfig))
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
        <div class="guaca-config">
          <span>{"Config. : url"}</span>
          <label class="switchy">
            <input type="checkbox" onclick={config_cb} checked={*config} />
            <span></span>
          </label>
          <span>{"local storage"}</span>
        </div>
        <hr />
        <h3>{"Historique"}</h3>
        <Historic />
      </div>
    </div>
  }
}

fn main() {
  console_log::init().unwrap();
  yew::start_app::<App>();
}
