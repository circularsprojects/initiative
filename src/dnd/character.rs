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
    pub fn get_proficiency_bonus(&self) -> i16 {
        (self.level as f32 / 4.0).ceil() as i16 + 1
    }

    pub fn get_stat_modifier(&self, stat: StatType) -> i16 {
        self.stats.get_modifier(stat)
    }

    pub fn get_skill_modifier(&self, skill: SkillType) -> i16 {
        self.skills.get(skill).get_modifier(&self.stats, self.get_proficiency_bonus())
    }
}