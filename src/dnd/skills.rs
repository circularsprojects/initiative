use super::stats::StatType;

pub struct Skill {
    skill_type: SkillType,
    stat_type: StatType,
    proficiency: bool,
    expertise: bool,
    extra_bonus: i8,
}

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