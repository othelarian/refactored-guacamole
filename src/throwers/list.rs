use yew::prelude::*;
use web_sys::HtmlInputElement;
use yew_agent::{Bridge, Bridged};

use crate::saver::*;
use crate::store::{
  Store, StoreInput, StoreOutput,
  IdsOrder, create_ids_order,
  ThrowerIds, create_thrower_ids,
  SelboxState
};
use super::Thrower;

pub enum ThrowerListMsg {
  InitThrowerIds(IdsOrder, ThrowerIds),
  NoOp,
  RefreshList,
  ThrowAll,
  ThrowSelected,
  ToggleSelbox,
  UpdateSelbox(SelboxState)
}

pub struct ThrowerList {
  order: IdsOrder,
  selbox_state: SelboxState,
  store: Box<dyn Bridge<Store>>,
  ref_selbox: NodeRef,
  ref_selroll: NodeRef,
  thrower_ids: ThrowerIds
}

impl Component for ThrowerList {
  type Message = ThrowerListMsg;
  type Properties = ();

  fn create(ctx: &Context<Self>) -> Self {
    let mut store = Store::bridge(ctx.link().callback(|res| match res {
      StoreOutput::InitList(order, thrower_ids) =>
        ThrowerListMsg::InitThrowerIds(order, thrower_ids),
      StoreOutput::UpdateSelbox(state) =>
        ThrowerListMsg::UpdateSelbox(state),
      StoreOutput::UpdateThrowerList => ThrowerListMsg::RefreshList,
      _ => ThrowerListMsg::NoOp
    }));
    store.send(StoreInput::InitList);
    //
    // TODO: récupération probable de Location ici
    //
    Self {
      order: create_ids_order(),
      store,
      selbox_state: SelboxState::Unchecked,
      ref_selbox: NodeRef::default(),
      ref_selroll: NodeRef::default(),
      thrower_ids: create_thrower_ids()
    }
  }

  fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
    if first_render {
      //
      // TODO: ajout de la première config par défaut, ou récupération depuis App (url) ?
      //
    }
    self.update_selbox();
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      ThrowerListMsg::InitThrowerIds(order, thrower_ids) => {
        self.order = order;
        self.thrower_ids = thrower_ids;
        true
      }
      ThrowerListMsg::NoOp => false,
      ThrowerListMsg::RefreshList => true,
      ThrowerListMsg::ThrowAll =>
        { self.store.send(StoreInput::ThrowAll); false }
      ThrowerListMsg::ThrowSelected =>
        { self.store.send(StoreInput::ThrowSelected); false }
      ThrowerListMsg::ToggleSelbox => {
        //
        // TODO
        //
        false
      }
      ThrowerListMsg::UpdateSelbox(state) => {
        self.selbox_state = state; self.update_selbox(); false
      }
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    // preparation
    let order = self.order.borrow();
    let throwers: Html = order.iter().map(|key| {
      html! { <div key={key.to_string()} class="guaca-line">
        <Thrower index={key.clone()} />
      </div> }
    }).collect();
    // styles
    //
    // TODO
    let selbox_display =
      if order.len() > 1 { "display:inline-block;" }
      else { "display:none;" };
    //
    // callbacks
    let roll_all_cb = ctx.link().callback(|_| ThrowerListMsg::ThrowAll);
    let roll_selected_cb = ctx.link().callback(|_| ThrowerListMsg::ThrowSelected);
    let selbox_cb = ctx.link().callback(|_| ThrowerListMsg::ToggleSelbox);
    // rendering
    html! {
      <div>
        {throwers}
        if order.len() > 1 {
          <div class="guaca-controls">
            <label class="checky" style={selbox_display}>
              <input type="checkbox" onchange={selbox_cb}
                ref={self.ref_selbox.clone()} />
              <span></span>
            </label>
          </div>
          <div class="guaca-selector">
            //
            // TODO: gérer l'affichage conditionnel de "lancer la sélection"
            //
            <button
              //style={display_selroll}
              ref={self.ref_selroll.clone()}
              onclick={roll_selected_cb}
            >{"Lancer la sélection"}</button>
          </div>
          <div class="guaca-result">
            <button onclick={roll_all_cb}>{"Tout lancer"}</button>
          </div>
        } 
      </div>
    }
  }
}

impl ThrowerList {
  fn update_selbox(&self) {
    if let Some(element) = self.ref_selbox.cast::<HtmlInputElement>() {
      let (checky, indery) = match self.selbox_state {
        SelboxState::Checked => (true, false),
        SelboxState::PartiallyChecked => (false, true),
        SelboxState::Unchecked => (false, false)
      };
      element.set_checked(checky);
      element.set_indeterminate(indery);
    }
  }
}
