use log::info;
use yew::prelude::*;
use yewdux::prelude::*;

mod histo;
use histo::{Historic, HistoStore};

mod throwers;
use throwers::{ThrowerList, ThrowerStore};


struct App {
  histo_dispatch: Dispatch<BasicStore<HistoStore>>,
  thrower_dispatch: Dispatch<BasicStore<ThrowerStore>>
}

impl Component for App {
  type Message = ();
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    //
    // TODO: parser d'url ici ?
    //
    Self {
      histo_dispatch: Dispatch::default(),
      thrower_dispatch: Dispatch::default()
    }
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    //
    // TODO: a supprimer après les tests
    info!("app rendering...");
    //
    //
    // throwers callbacks
    let thrower_add_cb = self.thrower_dispatch.reduce_callback(|s| {
      //
      // TODO
      //
      //
    });
    let thrower_clear_cb = self.thrower_dispatch.reduce_callback(|s| {
      //
      // TODO
      //
    });
    // history callback
    let histo_clear_cb = self.histo_dispatch.reduce_callback(|s| s.history.clear() );
    //
    //
    // TODO: TEST !!!!!
    let histo_add_cb = self.histo_dispatch.reduce_callback(|s| {
      let l = s.history.len();
      s.history.push(histo::HistoResult::create(l))
    });
    //
    // Render everything
    html! {
      <div class="guaca-container">
        <div class="guaca-block guaca-throwers">
          <div class="guaca-navbar">
            <button onclick={thrower_clear_cb}>{"Tout supprimer"}</button>
            <button onclick={thrower_add_cb}>{"Ajouter"}</button>
          </div>
          //
          <button onclick={histo_add_cb}>{"test purpose"}</button>
          //
          <ThrowerList />
        </div>
        <div class="guaca-block guaca-histo">
          <div class="guaca-navbar">
            <button onclick={histo_clear_cb}>{"tout effacer"}</button>
          </div>
          <div class="guaca-list"><Historic /></div>
        </div>
      </div>
    }
  }
}

fn main() {
  console_log::init().unwrap();
  yew::start_app::<App>();
}
