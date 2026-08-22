use super::prelude::*;

pub struct Character {
    pub name: String,
    // pub class: Class,
    // pub race: Race,
    pub level: u8,

    // pub inventory: Inventory,
    // pub spells: Spells,
    // pub spell_slots: SpellSlots,

    pub inspiration: bool,
    pub speed: i16,
    pub health: i16,
    pub max_health: i16,
    pub temp_health: i16,
    pub armor_class: i16,

    pub stats: Stats,
    pub skills: Skills,
}

impl Character {
    pub fn get_proficiency_bonus(&self) -> u8 {
        (self.level as f32 / 4.0).ceil() as u8 + 1
    }
}