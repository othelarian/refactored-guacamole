use log::info;
use rand::prelude::*;

use crate::histo::HistoResult;

#[derive(Clone)]
pub enum DiceMethod {
  Each,
  Total
}

#[derive(Clone, PartialEq)]
pub enum DiceType {
  D4, D6, D8, D10, D12, D20, D100, Custom(usize)
}

#[derive(Clone)]
pub struct ThrowerConfig {
  pub name: String,
  nb_dice: usize,
  dice_type: DiceType,
  modifier: isize,
  pub method: DiceMethod,
  pub result: Option<usize>
}

impl ThrowerConfig {
  pub fn default() -> Self {
    Self::create(String::default(), 1, DiceType::D6, 0)
  }

  pub fn create(name: String, nb_dice: usize, dice_type: DiceType, modifier: isize) -> Self {
    Self {
      name, nb_dice, dice_type, modifier,
      method: DiceMethod::Total, result: None
    }
  }

  pub fn max(dice_type: &DiceType) -> usize {
    match dice_type {
      DiceType::D4 => 4,
      DiceType::D6 => 6,
      DiceType::D8 => 8,
      DiceType::D10 => 10,
      DiceType::D12 => 12,
      DiceType::D20 => 20,
      DiceType::D100 => 100,
      DiceType::Custom(v) => v.clone()
    }
  }

  pub fn method(method: &DiceMethod) -> String {
    match method {
      DiceMethod::Each => String::from("Séparé"),
      DiceMethod::Total => String::from("Total")
    }
  }

  pub fn placeholder(&self) -> String {
    let dice_nb = ThrowerConfig::max(&self.dice_type);
    let modifier =
      if self.modifier == 0 { String::default() }
      else if self.modifier > 0 { format!("+{}", self.modifier) }
      else { format!("{}", self.modifier) };
    let method = ThrowerConfig::method(&self.method);
    format!("{}d{}{}, {}", self.nb_dice, dice_nb, modifier, method)
  }

  pub fn roll(&self) -> HistoResult {
    //
    // TODO
    //
    let mut rng = thread_rng();
    //
    HistoResult::create(rng.gen_range(1..=6))
    //
  }
}
