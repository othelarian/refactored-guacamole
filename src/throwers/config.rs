use rand::prelude::*;
use std::str::FromStr;
use strum_macros::{Display, EnumIter, EnumString};

use crate::histo::HistoLine;

#[derive(Clone, Debug, Display, EnumString)]
pub enum DiceMethod {
  #[strum(serialize = "e")]
  Each,
  #[strum(serialize = "t")]
  Total
}

#[derive(Clone, Debug, Display, EnumIter, EnumString, PartialEq)]
pub enum DiceType {
  #[strum(serialize = "q")]
  D4,
  #[strum(serialize = "s")]
  D6,
  #[strum(serialize = "h")]
  D8,
  #[strum(serialize = "x")]
  D10,
  #[strum(serialize = "z")]
  D12,
  #[strum(serialize = "v")]
  D20,
  #[strum(serialize = "p")]
  D100,
  #[strum(serialize = "c")]
  D(usize)
}

impl DiceType {
  pub fn to_name(&self) -> String { String::from(match self {
    Self::D4 => "d4", Self::D6 => "d6", Self::D8 => "d8",
    Self::D10 => "d10", Self::D12 => "d12", Self::D20 => "d20",
    Self::D100 => "d100", Self::D(_) => "d"
  })}
}

#[derive(Clone)]
pub struct ThrowerConfig {
  pub dice_type: DiceType,
  pub method: DiceMethod,
  pub modifier: isize,
  pub name: String,
  pub nb_dice: usize,
  pub selected: bool
}

impl Default for ThrowerConfig {
  fn default() -> Self {
    Self::create(
      String::default(), 1, DiceType::D6, 0, DiceMethod::Total, false)
  }
}

#[derive(Display)]
enum CfgParseError {
  DiceCustom,
  DiceType,
  Method,
  Modifier,
  NbDice,
  Selected,
  Split
}

impl ThrowerConfig {
  pub fn create(
    name: String, nb_dice: usize, dice_type: DiceType,
    modifier: isize, method: DiceMethod, selected: bool
  ) -> Self {
    Self { name, nb_dice, dice_type, modifier, method, selected }
  }

  pub fn from_string(config: &String) -> Result<Self, ()> {
    let mut cfg = Self::default();
    let tmp_vec = &config.split("k").collect::<Vec<&str>>();
    if tmp_vec.len() < 3 { Err(()) } else {
      match DiceType::from_str(&config[0..1])
        .map_err(|_| CfgParseError::DiceType
        ).and_then(|dice_type| {
          if let DiceType::D(_) = dice_type {
            if tmp_vec.len() < 4 { Err(CfgParseError::Split) } else {
              tmp_vec[3].parse::<usize>()
                .map_err(|_| CfgParseError::DiceCustom)
                .map(|cv| DiceType::D(cv))
            }
          } else { Ok(dice_type) }
        }).and_then(|dice_type| {
          cfg.dice_type = dice_type;
          DiceMethod::from_str(&config[1..2]).map_err(|_| CfgParseError::Method)
        }).and_then(|method| {
          cfg.method = method;
          tmp_vec[1].parse::<usize>().map_err(|_| CfgParseError::NbDice)
          .and_then(|nb_dice| {
            cfg.nb_dice = nb_dice;
            tmp_vec[2].parse::<isize>().map_err(|_| CfgParseError::Modifier)
          })
        }).and_then(|modifier| {
          cfg.modifier = modifier;
          if &config[2..3] == "t" { cfg.selected = true; Ok(cfg) }
          else if &config[2..3] == "f" { cfg.selected = false; Ok(cfg) }
          else { Err(CfgParseError::Selected) }
        }) {
          Ok(cfg) => Ok(cfg),
          Err(_err) => {
            //log::info!("cfg parse error: {}", err);
            Err(())
          }
        }
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
      DiceType::D(v) => v.clone()
    }
  }

  pub fn method(method: &DiceMethod) -> String {
    match method {
      DiceMethod::Each => String::from("Séparé"),
      DiceMethod::Total => String::from("Total")
    }
  }

  pub fn placeholder(&self) -> String {
    let dice_nb = Self::max(&self.dice_type);
    let modifier =
      if self.modifier == 0 { String::default() }
      else if self.modifier > 0 { format!("+{}", self.modifier) }
      else { format!("{}", self.modifier) };
    let method = Self::method(&self.method);
    format!("{}d{}{}, {}", self.nb_dice, dice_nb, modifier, method)
  }

  pub fn roll(&self) -> HistoLine {
    //
    // TODO
    //
    let mut rng = thread_rng();
    //
    let name =
      if self.name.is_empty() { self.placeholder() }
      else { format!("{} ({})", self.name, self.placeholder()) };
    //
    let total = 5; // TODO: ce n'est clairement pas ça Xb
    //
    //HistoResult::create(rng.gen_range(1..=6))
    //
    HistoLine::create(name, total)
    //
  }

  pub fn to_string(&self) -> String {
    let dice_type = self.dice_type.to_string();
    let method = self.method.to_string();
    let selected = match self.selected { true => "t", false => "f" };
    let custom_dice = if let DiceType::D(custom) = &self.dice_type {
      format!("k{}", custom) } else { String::from("") };
    format!("{}{}{}k{}k{}{}",
      dice_type, method, selected, self.nb_dice, self.modifier, custom_dice)
  }
}
