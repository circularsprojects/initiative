use crate::dnd::skills::Skills;
use crate::number;
use super::utils::{determine_stat_modifier, Number};

#[derive(Debug, Clone)]
pub enum Stat {
    Strength(Number),
    Dexterity(Number),
    Constitution(Number),
    Intelligence(Number),
    Wisdom(Number),
    Charisma(Number),
}

#[derive(Debug, Clone)]
pub struct Stats {
    strength: Stat,
    dexterity: Stat,
    constitution: Stat,
    intelligence: Stat,
    wisdom: Stat,
    charisma: Stat,
}

// impl StatInfo {
//     pub fn get_modifier(&self) -> i16 {
//         determine_stat_modifier(self.score.get_value())
//     }
// }

impl Default for Stats {
    fn default() -> Self {
        Stats {
            strength: Stat::Strength(number!(10)),
            dexterity: Stat::Dexterity(number!(10)),
            constitution: Stat::Constitution(number!(10)),
            intelligence: Stat::Intelligence(number!(10)),
            wisdom: Stat::Wisdom(number!(10)),
            charisma: Stat::Charisma(number!(10))
        }
    }
}