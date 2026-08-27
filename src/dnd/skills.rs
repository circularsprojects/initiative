use super::stats::{Stat, StatType, Stats};

#[derive(Debug, Clone)]
pub struct Skill {
    // skill_type: SkillType,
    stat_type: StatType,
    proficiency: bool,
    expertise: bool,
    extra_bonus: i8,
}

impl Skill {
    pub fn new(skill_type: SkillType, proficiency: bool, expertise: bool) -> Skill {
        let stat_type = skill_type.get_stat();
        Skill {
            // skill_type,
            stat_type,
            proficiency,
            expertise,
            extra_bonus: 0,
        }
    }

    pub fn get_modifier(&self, stats: &Stats, proficiency_bonus: i16) -> i16 {
        let stat_mod = stats.get_modifier(self.stat_type);
        let prof_mult = if self.expertise { 2 } else if self.proficiency { 1 } else { 0 };
        stat_mod + (proficiency_bonus * prof_mult) + self.extra_bonus as i16
    }
}

#[derive(Debug, Clone)]
pub enum SkillType {
    // could have these as an enum that has the value of a Skill
    // like enum Acrobatics is a Skill with stat type dex
    // pub enum SkillType {
    //   Acrobatics(StatType::Dexterity)
    Acrobatics,
    AnimalHandling,
    Arcana,
    Athletics,
    Deception,
    History,
    Insight,
    Intimidation,
    Investigation,
    Medicine,
    Nature,
    Perception,
    Performance,
    Persuasion,
    Religion,
    SleightOfHand,
    Stealth,
    Survival,
}

impl SkillType {
    pub fn get_stat(&self) -> StatType {
        match self {
            SkillType::Acrobatics => StatType::Dexterity,
            SkillType::AnimalHandling => StatType::Wisdom,
            SkillType::Arcana => StatType::Intelligence,
            SkillType::Athletics => StatType::Strength,
            SkillType::Deception => StatType::Charisma,
            SkillType::History => StatType::Intelligence,
            SkillType::Insight => StatType::Wisdom,
            SkillType::Intimidation => StatType::Charisma,
            SkillType::Investigation => StatType::Intelligence,
            SkillType::Medicine => StatType::Wisdom,
            SkillType::Nature => StatType::Intelligence,
            SkillType::Perception => StatType::Wisdom,
            SkillType::Performance => StatType::Charisma,
            SkillType::Persuasion => StatType::Charisma,
            SkillType::Religion => StatType::Intelligence,
            SkillType::SleightOfHand => StatType::Dexterity,
            SkillType::Stealth => StatType::Dexterity,
            SkillType::Survival => StatType::Wisdom,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Skills {
    acrobatics: Skill,
    animal_handling: Skill,
    arcana: Skill,
    athletics: Skill,
    deception: Skill,
    history: Skill,
    insight: Skill,
    intimidation: Skill,
    investigation: Skill,
    medicine: Skill,
    nature: Skill,
    perception: Skill,
    performance: Skill,
    persuasion: Skill,
    religion: Skill,
    sleight_of_hand: Skill,
    stealth: Skill,
    survival: Skill,
}

macro_rules! skill {
    ($a:expr) => {
        Skill::new($a, false, false)
    };
}

impl Default for Skills {
    fn default() -> Self {
        Skills {
            acrobatics: skill!(SkillType::Acrobatics),
            animal_handling: skill!(SkillType::AnimalHandling),
            arcana: skill!(SkillType::Arcana),
            athletics: skill!(SkillType::Athletics),
            deception: skill!(SkillType::Deception),
            history: skill!(SkillType::History),
            insight: skill!(SkillType::Insight),
            intimidation: skill!(SkillType::Intimidation),
            investigation: skill!(SkillType::Investigation),
            medicine: skill!(SkillType::Medicine),
            nature: skill!(SkillType::Nature),
            perception: skill!(SkillType::Perception),
            performance: skill!(SkillType::Performance),
            persuasion: skill!(SkillType::Persuasion),
            religion: skill!(SkillType::Religion),
            sleight_of_hand: skill!(SkillType::SleightOfHand),
            stealth: skill!(SkillType::Stealth),
            survival: skill!(SkillType::Survival),
        }
    }
}

impl Skills {
    pub fn get(&self, skill: SkillType) -> &Skill {
        match skill {
            SkillType::Acrobatics => &self.acrobatics,
            SkillType::AnimalHandling => &self.animal_handling,
            SkillType::Arcana => &self.arcana,
            SkillType::Athletics => &self.athletics,
            SkillType::Deception => &self.deception,
            SkillType::History => &self.history,
            SkillType::Insight => &self.insight,
            SkillType::Intimidation => &self.intimidation,
            SkillType::Investigation => &self.investigation,
            SkillType::Medicine => &self.medicine,
            SkillType::Nature => &self.nature,
            SkillType::Perception => &self.perception,
            SkillType::Performance => &self.performance,
            SkillType::Persuasion => &self.persuasion,
            SkillType::Religion => &self.religion,
            SkillType::SleightOfHand => &self.sleight_of_hand,
            SkillType::Stealth => &self.stealth,
            SkillType::Survival => &self.survival,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dnd::stats::tests::get_test_stats;

    #[test]
    fn test_skills() {
        let stats = get_test_stats();
        let mut skills = Skills::default();
        skills.arcana.expertise = true;
        skills.history.proficiency = true;
        skills.insight.proficiency = true;
        skills.investigation.proficiency = true;
        skills.nature.proficiency = true;
        skills.perception.proficiency = true;
        skills.persuasion.proficiency = true;
        skills.stealth.proficiency = true;
        skills.survival.proficiency = true;
        let proficiency_bonus = 2;

        assert_eq!(skills.acrobatics.get_modifier(&stats, proficiency_bonus), -1);
        assert_eq!(skills.animal_handling.get_modifier(&stats, proficiency_bonus), 2);
        assert_eq!(skills.arcana.get_modifier(&stats, proficiency_bonus), 7);
        assert_eq!(skills.athletics.get_modifier(&stats, proficiency_bonus), 1);
        assert_eq!(skills.deception.get_modifier(&stats, proficiency_bonus), 0);
        assert_eq!(skills.history.get_modifier(&stats, proficiency_bonus), 5);
        assert_eq!(skills.insight.get_modifier(&stats, proficiency_bonus), 4);
        assert_eq!(skills.intimidation.get_modifier(&stats, proficiency_bonus), 0);
        assert_eq!(skills.investigation.get_modifier(&stats, proficiency_bonus), 5);
        assert_eq!(skills.medicine.get_modifier(&stats, proficiency_bonus), 2);
        assert_eq!(skills.nature.get_modifier(&stats, proficiency_bonus), 5);
        assert_eq!(skills.perception.get_modifier(&stats, proficiency_bonus), 4);
        assert_eq!(skills.performance.get_modifier(&stats, proficiency_bonus), 0);
        assert_eq!(skills.persuasion.get_modifier(&stats, proficiency_bonus), 2);
        assert_eq!(skills.religion.get_modifier(&stats, proficiency_bonus), 3);
        assert_eq!(skills.sleight_of_hand.get_modifier(&stats, proficiency_bonus), -1);
        assert_eq!(skills.stealth.get_modifier(&stats, proficiency_bonus), 1);
        assert_eq!(skills.survival.get_modifier(&stats, proficiency_bonus), 4);
    }
}