use std::ops::Add;

pub fn determine_stat_modifier(stat_score: i8) -> i8 {
    if stat_score <= 0 {
        panic!("Stat score must be greater than 0");
    }
    (stat_score - 10) >> 1
}

pub struct Number {
    // base is optional here
    // number is meant to be like an all encompassing struct for any sort of "number influenced by other things"
    // like for DC scores, attack roll modifiers and the like
    // a DC check would have a base of 10 but any attack modifier would just be a plus-something, with no "base number"
    pub base: Option<i16>,
    pub factors: Vec<Factor>,
}

pub struct Factor {
    pub change: i16,
    pub description: String,
}

impl Number {
    pub fn new(base: Option<i16>, factors: Vec<Factor>) -> Number {
        Number { base, factors }
    }

    pub fn get_value(&self) -> i16 {
        self.base.unwrap_or(0) + self.factors.iter().fold(0, | acc, f| acc + f.change)
    }
}

impl Factor {
    pub fn new(change: i16, description: String) -> Factor {
        Factor { change, description }
    }
}

macro_rules! number {
    ($a:expr) => {
        Number::new(Some($a as i16), vec![])
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_stat_modifier() {
        assert_eq!(determine_stat_modifier(1), -5);
        assert_eq!(determine_stat_modifier(2), -4);
        assert_eq!(determine_stat_modifier(3), -4);
        assert_eq!(determine_stat_modifier(4), -3);
        assert_eq!(determine_stat_modifier(5), -3);
        assert_eq!(determine_stat_modifier(6), -2);
        assert_eq!(determine_stat_modifier(7), -2);
        assert_eq!(determine_stat_modifier(8), -1);
        assert_eq!(determine_stat_modifier(9), -1);
        assert_eq!(determine_stat_modifier(10), 0);
        assert_eq!(determine_stat_modifier(11), 0);
        assert_eq!(determine_stat_modifier(12), 1);
        assert_eq!(determine_stat_modifier(13), 1);
        assert_eq!(determine_stat_modifier(14), 2);
        assert_eq!(determine_stat_modifier(15), 2);
        assert_eq!(determine_stat_modifier(16), 3);
        assert_eq!(determine_stat_modifier(17), 3);
        assert_eq!(determine_stat_modifier(18), 4);
        assert_eq!(determine_stat_modifier(19), 4);
        assert_eq!(determine_stat_modifier(20), 5);
        assert_eq!(determine_stat_modifier(21), 5);
        assert_eq!(determine_stat_modifier(22), 6);
        assert_eq!(determine_stat_modifier(23), 6);
        assert_eq!(determine_stat_modifier(24), 7);
        assert_eq!(determine_stat_modifier(25), 7);
        assert_eq!(determine_stat_modifier(26), 8);
        assert_eq!(determine_stat_modifier(27), 8);
        assert_eq!(determine_stat_modifier(28), 9);
        assert_eq!(determine_stat_modifier(29), 9);
        assert_eq!(determine_stat_modifier(30), 10);
    }

    #[test]
    fn test_modifier() {
        let factors: Vec<Factor> = vec![
            Factor::new(3, "Proficiency".to_string()),
            Factor::new(1, "Artifact of Increasing This Shit by One".to_string())
        ];

        let modifier = Number::new(Some(10), factors);

        assert_eq!(modifier.get_value(), 14);
    }
}