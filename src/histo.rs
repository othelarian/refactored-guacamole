use yew::prelude::*;
use yewdux::prelude::*;
use yewdux_functional::*;

#[derive(Clone)]
pub struct HistoResult {
  //
  //
  test1: usize
  //
}

impl HistoResult {
  pub fn create(test1: usize) -> Self {
    //
    //
    HistoResult {
      //
      test1
      //
    }
  }

  pub fn view(&self) -> Html {
    //
    //
    html! {
      <div>
        {"histo: "}{self.test1}
      </div>
    }
  }
}

#[derive(Clone, Default)]
pub struct HistoStore {
  pub history: Vec<HistoResult>
}

#[function_component(Historic)]
pub fn historic() -> Html {
  let store = use_store::<BasicStore<HistoStore>>();
  match store.state() {
    None => html! {<></>},
    Some(state) => {
      let history: Html = state.history.iter()
        .enumerate()
        .map(|(id, res)| {
          let delete_res_cb = store.dispatch().reduce_callback(move |s| {
            s.history.remove(id)
          });
          html! {
            //
            // TODO: un petit travail de theming reste à fare ici
            //
            <div class="">
              {res.view()}
              <button onclick={delete_res_cb}>{"x"}</button>
            </div>
          }
        })
        .collect();
      html! { <div>{history}</div> }
    }
  }
}
