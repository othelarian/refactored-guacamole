//use log::info;
use yew::prelude::*;

mod histo;
use histo::HistoResult;

mod throwers;
use throwers::ThrowerConfig;

pub enum AppMessage {
  NoOp
}

struct App {
  history: Vec<HistoResult>,
  throwers: Vec<ThrowerConfig>
}

impl Component for App {
  type Message = AppMessage;
  type Properties = ();

  fn create(ctx: &Context<Self>) -> Self {
    Self {
      history: Vec::default(),
      throwers: Vec::default()
    }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      //
      //
      NoOp => false
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    // get throwers
    //
    // TODO
    //
    // get result history
    //
    let historic: Html = self.history.iter()
      .enumerate()
      .map(move |(id, res)| self.view_history_result(ctx, res.clone(), id))
      .collect();
    //
    // build callbacks
    //
    // TODO
    //
    // Render everything
    html! {
      <div class="guaca-container">
        <div class="guaca-throwers">
          //
          {"test"}
          //
          //
        </div>
        <div class="guaca-histo">
          //
          {"test"}
          //
          //
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
}

fn main() {
  console_log::init().unwrap();
  yew::start_app::<App>();
}
