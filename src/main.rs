use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::use_bridge;

mod histo;
mod throwers;
pub mod saver;
mod store;

use histo::Historic;
use throwers::ThrowerList;
use store::{ConfigStoring, Store, StoreInput, StoreOutput};

fn url_note() -> Html { html!{
  <span class="note">
    {"*la config url ne permet pas de stocker le nom des lanceurs"}
  </span>
}}

#[function_component(StoreConfig)]
fn store_config() -> Html {
  // preparation
  let storing = use_state(|| ConfigStoring::Pending);
  let togconfig = use_state(|| false);
  let store = {
    let storing = storing.clone();
    let togconfig = togconfig.clone();
    use_bridge::<Store, _>(move |out| match out {
      StoreOutput::ConfigState(new_state, tog_val) => {
        storing.set(new_state);
        togconfig.set(tog_val);
      }
      StoreOutput::ConfigLSAfail => storing.set(ConfigStoring::Unavailable),
      _ => ()
    })
  };
  // callback
  let config_cb = {
    let store = store.clone();
    Callback::from(move |evt: MouseEvent| {
      match evt.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()) {
        Some(input) => store.send(StoreInput::ToggleConfig(input.checked())),
        None => ()
      }
    })
  };
  // rendering
  match *storing {
    ConfigStoring::Operational => html! {
      <div class="guaca-config">
        <span>{"Config. : url*"}</span>
        <label class="switchy">
          <input type="checkbox" onclick={config_cb} checked={*togconfig} />
          <span></span>
        </label>
        <span>{"local storage"}</span>
        <br />
        {url_note()}
      </div>
    },
    ConfigStoring::Pending => {
      store.send(StoreInput::GetConfig);
      html! {
        <div class="guaca-config">{"Configuration en cours..."}</div>
      }
    }
    ConfigStoring::Unavailable => html! {
      <div class="guaca-config">
        {"Config. uniquement via url*"}<br/>{url_note()}
      </div>
    }
  }
}

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
          <StoreConfig />
          <hr />
          <h3>{"Historique"}</h3>
          <Historic />
        </div>
      </div>
    </>
  }
}

fn main() {
  //console_log::init().unwrap();
  yew::start_app::<App>();
}
