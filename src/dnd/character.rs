use super::prelude::*;

pub struct Character {
    pub name: String,
    // pub class: Class,
    // pub race: Race,

    pub inspiration: bool,
    pub speed: i16,
    pub health: i16,
    pub max_health: i16,
    pub temp_health: i16,
    pub armor_class: i16,

    pub stats: Stats,
    pub skills: Skills,
    pub proficiency_bonus: i16,
}