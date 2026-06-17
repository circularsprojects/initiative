use std::fmt::{Display, Formatter};
use std::ops::Add;
use rand::distr::Uniform;
use rand::prelude::*;

#[derive(Debug, Clone)]
pub struct Dice {
    pub sides: u16,
    pub amount: u16,
}

impl Dice {
    pub fn new(amount: u16, sides: u16) -> Dice {
        Dice { sides, amount }
    }

    pub fn roll(&self) -> u16 {
        let mut rng = ThreadRng::default();
        let die = Uniform::new_inclusive(1, self.sides).expect("Failed to create dice roll");

        let mut total: u16 = 0;

        for _ in 0..self.amount {
            let throw = die.sample(&mut rng);
            total += throw;
        }

        total
    }
}

impl Display for Dice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}d{}", self.amount, self.sides)
    }
}

impl Add for Dice {
    type Output = CompoundDice;

    fn add(self, rhs: Self) -> Self::Output {
        let mut dice: Vec<Dice> = vec![self, rhs];
        dice.sort_by_key(|a| a.sides);
        dice.reverse();

        CompoundDice::from(dice)
    }
}

#[derive(Debug, Clone)]
pub struct CompoundDice {
    pub dice: Vec<Dice>
}

impl CompoundDice {
    pub fn roll(&self) -> u16 {
        let mut rng = ThreadRng::default();

        let mut total: u16 = 0;

        for dice in &self.dice {
            let die = Uniform::new_inclusive(1, dice.sides).expect("Failed to create dice roll");

            for _ in 0..dice.amount {
                let throw = die.sample(&mut rng);
                total += throw;
            }
        }

        total
    }
}

impl Display for CompoundDice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut rolls: Vec<String> = Vec::new();
        for dice in &self.dice {
            rolls.push(dice.to_string());
        }

        write!(f, "{}", rolls.join(" + "))
    }
}

impl Add for CompoundDice {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.dice.extend(rhs.dice);
        self.dice.sort_by_key(|a| a.sides);
        self.dice.reverse();
        self
    }
}

impl From<Vec<Dice>> for CompoundDice {
    fn from(value: Vec<Dice>) -> Self {
        CompoundDice { dice: value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dice_string_representation() {
        assert_eq!(Dice::new(10, 10).to_string(), "10d10");
        assert_eq!(Dice::new(4, 6).to_string(), "4d6");
        assert_eq!(Dice::new(2, 12).to_string(), "2d12");
        assert_eq!(Dice::new(7, 20).to_string(), "7d20");
        assert_eq!(Dice::new(9, 8).to_string(), "9d8");
        assert_eq!(Dice::new(3, 4).to_string(), "3d4");
    }

    #[test]
    fn test_compound_dice_string_representation() {
        let cm1 = Dice::new(4, 6) + Dice::new(2, 12);
        assert_eq!(cm1.to_string(), "2d12 + 4d6");

        let cm2 = Dice::new(4, 4) + Dice::new(3, 8);
        assert_eq!(cm2.to_string(), "3d8 + 4d4");

        let cm3 = Dice::new(2, 10) + Dice::new(2, 4);
        assert_eq!(cm3.to_string(), "2d10 + 2d4");

        let cm4 = cm1 + cm2.clone();
        assert_eq!(cm4.to_string(), "2d12 + 3d8 + 4d6 + 4d4");

        let cm5 = cm2 + cm3;
        assert_eq!(cm5.to_string(), "2d10 + 3d8 + 2d4 + 4d4");
    }
}