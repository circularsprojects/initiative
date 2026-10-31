use crate::dnd::dice::Dice;
use crate::dnd::stats::StatType;

pub struct Class {
    pub hit_dice: Dice,

    pub armor_proficiencies: Vec<String>,
    pub saving_throw_proficiencies: Vec<StatType>,
}