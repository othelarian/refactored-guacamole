use log::info;
use std::rc::Rc;
use yew_agent::{Agent, AgentLink, Context, HandlerId};

use crate::histo::{HistoAction, HistoResult};
use crate::throwers::ThrowerConfig;

pub enum StoreMsg {
  UpdateList(usize)
}

pub enum StoreInput {
  AddHistory,
  ClearHistory,
  ClearThrowers,
  DeleteThrower(usize),
  GetLength,
  RegisterHistory,
  RegisterThrower,
  ThrowAdd,
  ThrowAll,
  ThrowSelected
}

pub enum StoreOutput {
  HistoryAction(HistoAction),
  InitThrower(Rc<Vec<ThrowerConfig>>),
  UpdateThrowerList(usize)
}

pub struct Store {
  id_history: Option<HandlerId>,
  id_list: Option<HandlerId>,
  ids_throwers: Vec<HandlerId>,
  link: AgentLink<Self>,
  throwers: Rc<Vec<ThrowerConfig>>
}

impl Agent for Store {
  type Reach = Context<Self>;
  type Message = StoreMsg;
  type Input = StoreInput;
  type Output = StoreOutput;

  fn create(link: AgentLink<Self>) -> Self {
    Self {
      id_history: None,
      id_list: None,
      ids_throwers: Vec::default(),
      link,
      throwers: Rc::new(vec!(ThrowerConfig::default()))
    }
  }

  fn update(&mut self, msg: Self::Message) {
    match msg {
      StoreMsg::UpdateList(len) => {
        self.ids_throwers.clear();
        if let Some(id) = &self.id_list {
          self.link.respond(*id, StoreOutput::UpdateThrowerList(len));
        }
      }
    }
  }

  fn handle_input(&mut self, input: Self::Input, id: HandlerId) {
    match input {
      StoreInput::AddHistory => {
        //
        // TODO: le resultat, d'où vient-il ?
        //
        let res = HistoResult::create(34);
        //
        if let Some(id) = &self.id_history {
          self.link.respond(*id, StoreOutput::HistoryAction(HistoAction::Add(res)));
        }
      }
      StoreInput::ClearHistory => {
        if let Some(id) = &self.id_history {
          self.link.respond(*id, StoreOutput::HistoryAction(HistoAction::Clear));
        }
      }
      StoreInput::ClearThrowers => {
        self.throwers = Rc::new(Vec::default());
        self.link.send_message(StoreMsg::UpdateList(0));
      }
      StoreInput::DeleteThrower(id) => {
        let mut throwers = (*self.throwers).clone();
        throwers.remove(id);
        let len = throwers.len();
        self.throwers = Rc::new(throwers);
        self.link.send_message(StoreMsg::UpdateList(len));
      }
      StoreInput::GetLength => {
        self.id_list = Some(id);
        self.link.send_message(StoreMsg::UpdateList(self.throwers.len()));
      }
      StoreInput::RegisterHistory => self.id_history = Some(id),
      StoreInput::RegisterThrower => {
        self.ids_throwers.push(id);
        self.link.respond(id, StoreOutput::InitThrower(self.throwers.clone()));
      }
      StoreInput::ThrowAdd => {
        let mut throwers = (*self.throwers).clone();
        throwers.push(ThrowerConfig::default());
        let len = throwers.len();
        self.throwers = Rc::new(throwers);
        self.link.send_message(StoreMsg::UpdateList(len));
      }
      StoreInput::ThrowAll => {
        //
        // TODO
        //
      }
      StoreInput::ThrowSelected => {
        //
        // TODO
        //
      }
    }
  }
}
