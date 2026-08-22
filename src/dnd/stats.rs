use crate::dnd::skills::Skills;
use super::utils::determine_stat_modifier;

#[derive(Debug, Clone)]
pub struct Stat {
    score: i8,
    stat: StatType,
    saving_throw_proficiency: bool,
}

impl Stat {
    pub fn new(base: i8, stat: StatType, saving_throw_proficiency: bool) -> Self {
        Self {
            score: base,
            stat,
            saving_throw_proficiency,
        }
    }
    
    pub fn get_modifier(&self) -> i8 {
        determine_stat_modifier(self.score)
    }
}

#[derive(Debug, Clone)]
pub enum StatType {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
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

impl Default for Stats {
    fn default() -> Self {
        Self {
            strength: Stat::new(10, StatType::Strength, false),
            dexterity: Stat::new(10, StatType::Dexterity, false),
            constitution: Stat::new(10, StatType::Constitution, false),
            intelligence: Stat::new(10, StatType::Intelligence, false),
            wisdom: Stat::new(10, StatType::Wisdom, false),
            charisma: Stat::new(10, StatType::Charisma, false),
        }
    }
}

impl Stats {
    pub fn to_skills(&self) -> Skills {
        
    }
}