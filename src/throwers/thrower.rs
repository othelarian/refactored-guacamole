use log::info;
use yew::prelude::*;

pub struct Thrower;

impl Component for Thrower {
  type Message = ();
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Thrower
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    //
    //
    //
    html! {
      //
      {"thrower!"}
      //
    }
  }
}
