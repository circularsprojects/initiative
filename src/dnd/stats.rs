use super::utils::determine_modifier;

#[derive(Debug, Clone)]
pub struct Stat {
    base: i8,
    modifier: i8,
    stat: StatType,
    saving_throw_proficiency: bool,
}

impl Stat {
    pub fn new(base: i8, stat: StatType, saving_throw_proficiency: bool) -> Self {
        Self {
            base,
            modifier: determine_modifier(base),
            stat,
            saving_throw_proficiency,
        }
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