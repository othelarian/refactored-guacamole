use log::info;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use yew_agent::{Agent, AgentLink, Context, HandlerId};

use crate::histo::{HistoAction, HistoResult};
use crate::saver::*;
use crate::throwers::ThrowerConfig;

pub enum StoreMsg {
  UpdateConfig,
  UpdateList
}

pub enum StoreInput {
  AddHistory(HistoResult),
  ClearThrowers,
  DeleteThrower(usize),
  GetConfig,
  InitList,
  RegisterHistory,
  RegisterThrower(usize),
  SelectToggleAll(bool),
  ToggleConfig,
  ThrowAdd,
  ThrowAll,
  ThrowSelected,
  UpdateConfig,
  UpdateSelbox
}

pub enum StoreOutput {
  ConfigState(bool),
  ForceRefresh,
  HistoryAction(HistoAction),
  InitList(IdsOrder, ThrowerIds),
  InitThrower(ConfigHash),
  ToggleSelCheck(bool),
  UpdateSelbox(SelboxState),
  UpdateThrowerList
}

pub enum SelboxState {
  Unchecked,
  PartiallyChecked,
  Checked
}

pub type ConfigHash = Rc<RefCell<HashMap<usize, ThrowerConfig>>>;

pub fn create_config_hash() -> ConfigHash {
  Rc::new(RefCell::new(HashMap::default()))
}

pub type IdsOrder = Rc<RefCell<Vec<usize>>>;

pub fn create_ids_order() -> IdsOrder {
  Rc::new(RefCell::new(Vec::default()))
}

pub type ThrowerIds = Rc<RefCell<HashMap<usize, Option<HandlerId>>>>;

pub fn create_thrower_ids() -> ThrowerIds {
  Rc::new(RefCell::new(HashMap::default()))
}

pub struct Store {
  counter: usize,
  id_history: Option<HandlerId>,
  id_list: Option<HandlerId>,
  link: AgentLink<Self>,
  order: IdsOrder,
  storage_config: bool,
  throwers: ConfigHash,
  thrower_ids: ThrowerIds
}

impl Agent for Store {
  type Reach = Context<Self>;
  type Message = StoreMsg;
  type Input = StoreInput;
  type Output = StoreOutput;

  fn create(link: AgentLink<Self>) -> Self {
    //
    // TODO: interroger le js pour savoir s'il peut déterminer l'état de la config
    //
    let res = initiate_storage();
    //
    let storage_config = false;
    //
    //
    let order = create_ids_order();
    order.borrow_mut().push(0);
    let throwers = create_config_hash();
    throwers.borrow_mut().insert(0, ThrowerConfig::default());
    let thrower_ids = create_thrower_ids();
    thrower_ids.borrow_mut().insert(0, None);
    Self {
      counter: 1,
      id_history: None,
      id_list: None,
      link, order, storage_config, throwers, thrower_ids
    }
  }

  fn update(&mut self, msg: Self::Message) {
    match msg {
      StoreMsg::UpdateConfig => {
        //
        // TODO: maj via JS ici
        //
      }
      StoreMsg::UpdateList => {
        if let Some(id) = &self.id_list {
          self.link.respond(*id, StoreOutput::UpdateThrowerList);
        }
        let order_len = self.order.borrow().len();
        if order_len == 1 || order_len == 2 {
          let cfgs = self.thrower_ids.borrow();
          let some_id = cfgs.get(self.order.borrow().get(0).unwrap()).unwrap();
          if let Some(id) = &some_id {
            self.link.respond(*id, StoreOutput::ForceRefresh);
          }
        }
        self.link.send_message(StoreMsg::UpdateConfig);
      }
    }
  }

  fn handle_input(&mut self, input: Self::Input, id: HandlerId) {
    match input {
      StoreInput::AddHistory(result) => {
        if let Some(id) = &self.id_history {
          self.link.respond(*id, StoreOutput::HistoryAction(HistoAction::Add(result)));
        }
        if self.storage_config {
          self.link.send_message(StoreMsg::UpdateConfig)
        }
      }
      StoreInput::ClearThrowers => {
        self.throwers.borrow_mut().clear();
        self.thrower_ids.borrow_mut().clear();
        self.order.borrow_mut().clear();
        self.counter = 0;
        self.link.send_message(StoreMsg::UpdateList);
      }
      StoreInput::DeleteThrower(key) => {
        self.throwers.borrow_mut().remove(&key);
        self.thrower_ids.borrow_mut().remove(&key);
        let mut order = self.order.borrow_mut();
        let pos = order.iter().position(|x| *x == key).unwrap();
        order.remove(pos);
        self.link.send_message(StoreMsg::UpdateList);
      }
      StoreInput::GetConfig => {
        if let Some(id) = &self.id_history {
          self.link.respond(*id, StoreOutput::ConfigState(self.storage_config));
        }
      }
      StoreInput::InitList => {
        self.id_list = Some(id);
        self.link.respond(id, StoreOutput::InitList(self.order.clone(), self.thrower_ids.clone()));
      }
      StoreInput::RegisterHistory => self.id_history = Some(id),
      StoreInput::RegisterThrower(key) => {
        self.thrower_ids.borrow_mut().insert(key, Some(id));
        self.link.respond(id, StoreOutput::InitThrower(self.throwers.clone()));
      }
      StoreInput::SelectToggleAll(state) => {
        //
        // TODO: maj des configs
        //
        //
        for some_id in self.thrower_ids.borrow().values() {
          if let Some(id) = &some_id {
            self.link.respond(*id, StoreOutput::ToggleSelCheck(state));
          }
        }
        self.link.send_message(StoreMsg::UpdateConfig);
      }
      StoreInput::ToggleConfig => {
        self.storage_config = !self.storage_config;
        if let Some(id) = &self.id_history {
          self.link.respond(*id, StoreOutput::ConfigState(self.storage_config));
        }
        self.link.send_message(StoreMsg::UpdateConfig);
      }
      StoreInput::ThrowAdd => {
        self.throwers.borrow_mut().insert(
          self.counter, ThrowerConfig::default());
        self.thrower_ids.borrow_mut().insert(self.counter, None);
        self.order.borrow_mut().push(self.counter);
        self.counter += 1;
        self.link.send_message(StoreMsg::UpdateList);
      }
      StoreInput::ThrowAll => {
        //
        // TODO
        //
        info!("roll all");
        //
      }
      StoreInput::ThrowSelected => {
        //
        // TODO
        //
        info!("roll selected");
        //
      }
      StoreInput::UpdateConfig =>
        self.link.send_message(StoreMsg::UpdateConfig),
      StoreInput::UpdateSelbox => {
        let cfgs = self.throwers.borrow();
        let (acc, total) = cfgs.values().fold((0, 0), |(acc, total), config| {
          if config.selected { (acc+1, total+1) } else { (acc, total+1) }
        });
        let state =
          if acc == 0 { SelboxState::Unchecked }
          else if acc == total { SelboxState::Checked }
          else { SelboxState::PartiallyChecked };
        if let Some(id) = self.id_list {
          self.link.respond(id, StoreOutput::UpdateSelbox(state));
        }
        self.link.send_message(StoreMsg::UpdateConfig);
      }
    }
  }
}
