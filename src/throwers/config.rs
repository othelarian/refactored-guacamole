#[derive(Clone)]
pub enum DiceMethod {
  Each,
  Total
}

#[derive(Clone)]
pub enum DiceType {
  D4, D6, D8, D10, D20, D100, Custom(usize)
}

#[derive(Clone)]
pub struct ThrowerConfig {
  nb_dice: usize,
  dice_type: DiceType,
  modifier: isize,
  method: DiceMethod,
  result: Option<usize>
}

impl ThrowerConfig {
  pub fn default() -> Self { Self::create(1, DiceType::D6, 0) }

  pub fn create(nb_dice: usize, dice_type: DiceType, modifier: isize) -> Self {
    Self {
      nb_dice, dice_type, modifier,
      method: DiceMethod::Total, result: None
    }
  }

  pub fn roll(&self) {
    //
    // TODO
    //
  }
}
