use super::stats::StatType;

#[derive(Debug, Clone)]
pub struct Skill {
    skill_type: SkillType,
    stat_type: StatType,
    proficiency: bool,
    expertise: bool,
    extra_bonus: i8,
}

impl Skill {
    pub fn new(skill_type: SkillType, proficiency: bool, expertise: bool) -> Skill {
        let stat_type = skill_type.get_stat();
        Skill {
            skill_type,
            stat_type,
            proficiency,
            expertise,
            extra_bonus: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SkillType {
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