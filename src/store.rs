use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use yew_agent::{Agent, AgentLink, Context, HandlerId};

use crate::histo::{HistoAction, HistoLine, HistoResult};
use crate::saver::*;
use crate::throwers::ThrowerConfig;

pub enum StoreMsg {
  UpdateConfig,
  UpdateList,
  UpdateNames
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
  ToggleConfig(bool),
  ThrowAdd,
  ThrowAll,
  ThrowSelected,
  UpdateConfig(usize),
  UpdateName(usize),
  UpdateSelbox(usize)
}

pub enum StoreOutput {
  ConfigState(ConfigStoring, bool),
  ConfigLSAfail,
  ForceRefresh,
  HistoryAction(HistoAction),
  InitList(IdsOrder, ThrowerIds, SelboxState),
  InitThrower(ConfigHash),
  ToggleSelCheck(bool),
  UpdateRollRes(isize),
  UpdateSelbox(SelboxState),
  UpdateThrowerList
}

pub enum ConfigStoring {
  Operational,
  Pending,
  Unavailable
}

#[derive(PartialEq)]
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
  cfgs: Vec<String>,
  counter: usize,
  id_history: Option<HandlerId>,
  id_list: Option<HandlerId>,
  link: AgentLink<Self>,
  names: Vec<String>,
  order: IdsOrder,
  storage_config: Rc<GuacaConfig>,
  throwers: ConfigHash,
  thrower_ids: ThrowerIds
}

impl Agent for Store {
  type Reach = Context<Self>;
  type Message = StoreMsg;
  type Input = StoreInput;
  type Output = StoreOutput;

  fn create(link: AgentLink<Self>) -> Self {
    let storage_config = Rc::new(GuacaConfig::new());
    let order = create_ids_order();
    let throwers = create_config_hash();
    let thrower_ids = create_thrower_ids();
    let mut counter = 0;
    let (cfgs, names) = match parse_init(storage_config.has_config()) {
      Some((isurl, mut cfgs, names)) => {
        let mut names = if isurl { Vec::default() } else { names.unwrap() };
        let mut throwers = throwers.borrow_mut();
        let mut thrower_ids = thrower_ids.borrow_mut();
        let mut order = order.borrow_mut();
        let mut to_remove = Vec::default();
        for (idx, cfg) in cfgs.iter().enumerate() {
          match ThrowerConfig::from_string(cfg) {
            Ok(mut thrower) => {
              if isurl { names.push(String::default()); }
              else { thrower.name = names[idx].clone(); }
              throwers.insert(idx, thrower);
              thrower_ids.insert(idx, None);
              order.push(idx);
              counter += 1;
            }
            Err(_) => to_remove.push(idx)
          }
        }
        if to_remove.len() > 0 {
          for idx in to_remove.iter().rev() {
            cfgs.remove(idx.clone());
            if !isurl { names.remove(idx.clone()); }
          }
          storage_config.update_config(JsValue::from_serde(&cfgs).unwrap());
          if !isurl {
            storage_config.update_names(JsValue::from_serde(&names).unwrap()); }
        }
        (cfgs, names)
      }
      None => {
        order.borrow_mut().push(0);
        let def_thrower = ThrowerConfig::default();
        let cfgs = vec!(def_thrower.to_string());
        throwers.borrow_mut().insert(0, def_thrower);
        thrower_ids.borrow_mut().insert(0, None);
        storage_config.update_config(
          JsValue::from_serde(&vec!(&cfgs)).unwrap());
        if !storage_config.isurl() {
          storage_config.update_names(
            JsValue::from_serde(&vec!(String::from(""))).unwrap());
        }
        counter = 1;
        (cfgs, vec!(String::default()))
      }
    };
    Self {
      id_history: None,
      id_list: None,
      counter, cfgs, link, names, order, storage_config, throwers, thrower_ids
    }
  }

  fn update(&mut self, msg: Self::Message) {
    match msg {
      StoreMsg::UpdateConfig => {
        self.storage_config.update_config(
          JsValue::from_serde(&self.cfgs).unwrap());
      }
      StoreMsg::UpdateList => {
        let cfgs_len = self.cfgs.len();
        if cfgs_len == 1 || cfgs_len == 2 {
          let thrower_ids = self.thrower_ids.borrow();
          let some_id =
            thrower_ids.get(self.order.borrow().get(0).unwrap()).unwrap();
          if let Some(id) = &some_id {
            self.link.respond(*id, StoreOutput::ForceRefresh); }
        }
        self.link.send_message(StoreMsg::UpdateConfig);
        if let Some(id) = &self.id_list {
          self.link.respond(*id, StoreOutput::UpdateThrowerList); }
      }
      StoreMsg::UpdateNames => if !self.storage_config.isurl() {
        self.storage_config
          .update_names(JsValue::from_serde(&self.names).unwrap());
      }
    }
  }

  fn handle_input(&mut self, input: Self::Input, id: HandlerId) {
    match input {
      StoreInput::AddHistory(result) => {
        if let Some(id) = &self.id_history {
          self.link.respond(*id,
            StoreOutput::HistoryAction(HistoAction::Add(result)));
        }
      }
      StoreInput::ClearThrowers => {
        self.throwers.borrow_mut().clear();
        self.thrower_ids.borrow_mut().clear();
        self.order.borrow_mut().clear();
        self.cfgs.clear();
        self.names.clear();
        self.counter = 0;
        self.storage_config.clear_config();
        if let Some(id) = &self.id_list {
          self.link.respond(*id, StoreOutput::UpdateThrowerList); }
      }
      StoreInput::DeleteThrower(key) => {
        self.throwers.borrow_mut().remove(&key);
        self.thrower_ids.borrow_mut().remove(&key);
        let mut order = self.order.borrow_mut();
        let pos = order.iter().position(|x| *x == key).unwrap();
        order.remove(pos);
        self.cfgs.remove(pos);
        self.names.remove(pos);
        self.link.send_message(StoreMsg::UpdateList);
        self.link.send_message(StoreMsg::UpdateNames);
      }
      StoreInput::GetConfig => {
        let (new_state, tog_val) = if self.storage_config.islsa() {
          (ConfigStoring::Operational, !self.storage_config.isurl())
        } else { (ConfigStoring::Unavailable, false) };
        self.link.respond(id, StoreOutput::ConfigState(new_state, tog_val));
      }
      StoreInput::InitList => {
        self.id_list = Some(id);
        self.link.respond(id, StoreOutput::InitList(
          self.order.clone(),
          self.thrower_ids.clone(),
          self.selbox_state()
        ));
      }
      StoreInput::RegisterHistory => {
        self.id_history = Some(id);
        self.link.respond(id, StoreOutput::HistoryAction(
          HistoAction::SetGuacaLink(self.storage_config.clone())));
      }
      StoreInput::RegisterThrower(key) => {
        self.thrower_ids.borrow_mut().insert(key, Some(id));
        self.link.respond(id, StoreOutput::InitThrower(self.throwers.clone()));
      }
      StoreInput::SelectToggleAll(state) => {
        let mut throwers = self.throwers.borrow_mut();
        let thrower_ids = self.thrower_ids.borrow();
        for (id, key) in self.order.borrow().iter().enumerate() {
          throwers.get_mut(&key).unwrap().selected = state;
          self.cfgs[id] = throwers.get(&key).unwrap().to_string();
          if let Some(id) = &thrower_ids.get(&key).unwrap() {
            self.link.respond(*id, StoreOutput::ToggleSelCheck(state));
          }
        }
        self.link.send_message(StoreMsg::UpdateConfig);
      }
      StoreInput::ToggleConfig(choice) => {
        if self.storage_config.toggle_ls(choice) {
          if choice {
            self.storage_config.update_names(
              JsValue::from_serde(&self.names).unwrap());
            if let Some(id) = &self.id_history {
              self.link.respond(*id,
                StoreOutput::HistoryAction(HistoAction::Copy));
            }
          }
        } else { self.link.respond(id, StoreOutput::ConfigLSAfail); }
      }
      StoreInput::ThrowAdd => {
        let new_thrower = ThrowerConfig::default();
        self.cfgs.push(new_thrower.to_string());
        self.names.push(String::default());
        self.throwers.borrow_mut().insert(self.counter, new_thrower);
        self.thrower_ids.borrow_mut().insert(self.counter, None);
        self.order.borrow_mut().push(self.counter);
        self.counter += 1;
        self.link.send_message(StoreMsg::UpdateList);
        self.link.send_message(StoreMsg::UpdateNames);
      }
      StoreInput::ThrowAll => {
        let throwers = self.throwers.borrow();
        let thrower_ids = self.thrower_ids.borrow();
        let results: Vec<HistoLine> = self.order.borrow().iter()
          .map(|key| {
            let thrower = throwers.get(&key).unwrap();
            let res = thrower.roll();
            if let Some(id) = &thrower_ids.get(key).unwrap() {
              self.link.respond(*id, StoreOutput::UpdateRollRes(res.total));
            }
            res
          }).collect();
        if let Some(id) = &self.id_history {
          self.link.respond(*id, StoreOutput::HistoryAction(
            HistoAction::Add(HistoResult::create(results))));
        }
      }
      StoreInput::ThrowSelected => {
        let throwers = self.throwers.borrow();
        let thrower_ids = self.thrower_ids.borrow();
        let results: Vec<HistoLine> = self.order.borrow().iter()
          .fold(Vec::default(), |mut acc, key| {
            let thrower = throwers.get(&key).unwrap();
            if thrower.selected {
              let res = thrower.roll();
              if let Some(id) = &thrower_ids.get(key).unwrap() {
                self.link.respond(*id, StoreOutput::UpdateRollRes(res.total));
              }
              acc.push(res);
              acc
            } else { acc }
          });
        if let Some(id) = &self.id_history {
          self.link.respond(*id, StoreOutput::HistoryAction(
            HistoAction::Add(HistoResult::create(results))));
        }
      }
      StoreInput::UpdateConfig(key) => {
        let pos = self.order.borrow().iter().position(|x| *x == key).unwrap();
        self.cfgs[pos] = self.throwers.borrow().get(&key).unwrap().to_string();
        self.link.send_message(StoreMsg::UpdateConfig);
      }
      StoreInput::UpdateName(key) => {
        let pos = self.order.borrow().iter().position(|x| *x == key).unwrap();
        self.names[pos] =
          self.throwers.borrow().get(&key).unwrap().name.clone();
        self.link.send_message(StoreMsg::UpdateNames);
      }
      StoreInput::UpdateSelbox(key) => {
        if let Some(id) = self.id_list {
          self.link.respond(id, StoreOutput::UpdateSelbox(self.selbox_state()));
        }
        let pos = self.order.borrow().iter().position(|x| *x == key).unwrap();
        self.cfgs[pos] = self.throwers.borrow().get(&key).unwrap().to_string();
        self.link.send_message(StoreMsg::UpdateConfig);
      }
    }
  }
}

impl Store {
  fn selbox_state(&self) -> SelboxState {
    let cfgs = self.throwers.borrow();
    let (acc, total) = cfgs.values().fold((0, 0), |(acc, total), config| {
      if config.selected { (acc+1, total+1) } else { (acc, total+1) }
    });
    if acc == 0 { SelboxState::Unchecked }
    else if acc == total { SelboxState::Checked }
    else { SelboxState::PartiallyChecked }
  }
}
