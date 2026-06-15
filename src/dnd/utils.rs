pub fn determine_modifier(ability_score: i8) -> i8 {
    if ability_score <= 0 {
        panic!("Ability score must be greater than 0");
    }
    (ability_score - 10) >> 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_modifier() {
        assert_eq!(determine_modifier(1), -5);
        assert_eq!(determine_modifier(2), -4);
        assert_eq!(determine_modifier(3), -4);
        assert_eq!(determine_modifier(4), -3);
        assert_eq!(determine_modifier(5), -3);
        assert_eq!(determine_modifier(6), -2);
        assert_eq!(determine_modifier(7), -2);
        assert_eq!(determine_modifier(8), -1);
        assert_eq!(determine_modifier(9), -1);
        assert_eq!(determine_modifier(10), 0);
        assert_eq!(determine_modifier(11), 0);
        assert_eq!(determine_modifier(12), 1);
        assert_eq!(determine_modifier(13), 1);
        assert_eq!(determine_modifier(14), 2);
        assert_eq!(determine_modifier(15), 2);
        assert_eq!(determine_modifier(16), 3);
        assert_eq!(determine_modifier(17), 3);
        assert_eq!(determine_modifier(18), 4);
        assert_eq!(determine_modifier(19), 4);
        assert_eq!(determine_modifier(20), 5);
        assert_eq!(determine_modifier(21), 5);
        assert_eq!(determine_modifier(22), 6);
        assert_eq!(determine_modifier(23), 6);
        assert_eq!(determine_modifier(24), 7);
        assert_eq!(determine_modifier(25), 7);
        assert_eq!(determine_modifier(26), 8);
        assert_eq!(determine_modifier(27), 8);
        assert_eq!(determine_modifier(28), 9);
        assert_eq!(determine_modifier(29), 9);
        assert_eq!(determine_modifier(30), 10);
    }
}