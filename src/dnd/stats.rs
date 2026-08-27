use crate::dnd::skills::Skills;
use crate::number;
use super::utils::{determine_stat_modifier, Number};

#[derive(Debug, Clone, Copy)]
pub enum StatType { Strength, Dexterity, Constitution, Intelligence, Wisdom, Charisma }

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

impl Stat {
    pub fn score(&self) -> i16 {
        match self {
            Stat::Strength(n) | Stat::Dexterity(n) | Stat::Constitution(n)
            | Stat::Intelligence(n) | Stat::Wisdom(n) | Stat::Charisma(n) => n.get_value(),
        }
    }

    pub fn get_modifier(&self) -> i16 {
        determine_stat_modifier(self.score())
    }
}

impl Stats {
    pub fn get(&self, stat: StatType) -> &Stat {
        match stat {
            StatType::Strength => &self.strength,
            StatType::Dexterity => &self.dexterity,
            StatType::Constitution => &self.constitution,
            StatType::Intelligence => &self.intelligence,
            StatType::Wisdom => &self.wisdom,
            StatType::Charisma => &self.charisma,
        }
    }

    pub fn get_modifier(&self, stat: StatType) -> i16 {
        self.get(stat).get_modifier()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub(crate) fn get_test_stats() -> Stats {
        Stats {
            strength: Stat::Strength(number!(12)),
            dexterity: Stat::Dexterity(number!(9)),
            constitution: Stat::Constitution(number!(12)),
            intelligence: Stat::Intelligence(number!(17)),
            wisdom: Stat::Wisdom(number!(14)),
            charisma: Stat::Charisma(number!(10)),
        }
    }

    #[test]
    fn test_stat_modifier() {
        let stats = get_test_stats();
        assert_eq!(stats.get_modifier(StatType::Strength), 1);
        assert_eq!(stats.get_modifier(StatType::Dexterity), -1);
        assert_eq!(stats.get_modifier(StatType::Constitution), 1);
        assert_eq!(stats.get_modifier(StatType::Intelligence), 3);
        assert_eq!(stats.get_modifier(StatType::Wisdom), 2);
        assert_eq!(stats.get_modifier(StatType::Charisma), 0);
    }
}