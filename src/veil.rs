use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::use_bridge;


use crate::store::{ConfigStoring, Store, StoreInput, StoreOutput};

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

pub enum VeilShow {
  DoubleCfg,
  Options
}

#[function_component(Veil)]
pub fn veil() -> Html {
  // datas
  let init = use_state(|| false);
  let lock = use_state(|| false);
  let show = use_state(|| false);
  let showing = use_state(|| VeilShow::Options);
  let store = {
    let lock = lock.clone();
    let show = show.clone();
    let showing = showing.clone();
    use_bridge::<Store, _>(move |out| match out {
      StoreOutput::ToggleVeil(choice, to_show) => {
        let nlock = match to_show {
          VeilShow::DoubleCfg => true,
          VeilShow::Options => false
        };
        lock.set(nlock);
        show.set(choice);
        showing.set(to_show);
      }
      _ => ()
    })
  };
  // preps
  if !*init { store.send(StoreInput::RegisterVeil); init.set(true); }
  let visibility = if *show { "display:block" } else { "display:none" };
  let body = match *showing {
    VeilShow::DoubleCfg => {
      let only_ls_cb = {
        //
        // TODO
        //
        Callback::from(move |_| log::info!("not ready!"))
        //
      };

      //
      let only_url_cb = {
        //
        // TODO
        //
        Callback::from(move |_| log::info!("not ready!"))
        //
      };

      //
      let both_cb = {
        //
        // TODO
        //
        Callback::from(move |_| log::info!("not ready!"))
        //
      };
      html! {
        <>
          <h2>{"Double Configuration détectée !"}</h2>
          <div class="guaca-center2">{"Deux configurations ont été détectée, une via le local storage, et une via l'url"}</div>
          <div class="guaca-center2">{"Que souhaitez-vous faire ?"}</div>
          <div class="guaca-colbtns">
            <button onclick={only_ls_cb}>{"Garder le local storage uniquement"}</button>
            <button onclick={only_url_cb}>{"Remplacer le local storage par l'url"}</button>
            <button onclick={both_cb}>{"Ajouter la configuration url à la configuration local storage"}</button>
          </div>
        </>
      }
    }
    VeilShow::Options => html! {
      <>
        <h2>{"Options"}</h2>
        <StoreConfig />
        <hr />
        <div class="guaca-center">
          {"Si vous voulez voir le code, "}
          <a href="https://github.com/othelarian/refactored-guacamole" target="blank">
            {"c'est ici"}</a>
          <br />
          {format!("version de l'app : {}", env!("CARGO_PKG_VERSION"))}
        </div>
      </>
    }
  };
  // callbacks
  let togback_cb = {
    let lock = lock.clone();
    let show = show.clone();
    Callback::from(move |_| { if !*lock { show.set(false); } })
  };
  html! {
    <div class="guaca-veil" style={visibility}>
      <div class="guaca-back" onclick={togback_cb} />
      <div class="guaca-body">{body}</div>
    </div>
  }
}

