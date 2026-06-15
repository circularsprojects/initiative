mod dnd;

use iced::widget::{button, text, column, Column, text_input};

use crate::dnd::dice::Dice;

pub fn main() -> iced::Result {
    iced::application(State::default, State::update, State::view).run()
}

struct State {
    sides_input: String,
    amount_input: String,
    dice_result: String,
}

#[derive(Debug, Clone)]
enum Message {
    SidesChanged(String),
    AmountChanged(String),
    DiceRolled,
}

impl State {
    fn default() -> Self {
        State {
            sides_input: "".parse().unwrap(),
            amount_input: "".parse().unwrap(),
            dice_result: "".parse().unwrap(),
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SidesChanged(sides) => { self.sides_input = sides; }
            Message::AmountChanged(amount) => { self.amount_input = amount; }
            Message::DiceRolled => {
                if let Ok(sides) = self.sides_input.parse::<u16>() && let Ok(amount) = self.amount_input.parse::<u16>() {
                    let dice = Dice::new(sides, amount);
                    self.dice_result = dice.roll().to_string();
                } else {
                    self.dice_result = "invalid".to_string();
                }
            }
        }
    }

    fn view(&self) -> Column<'_, Message> {
        let sides = text_input("sides", &self.sides_input).on_input(Message::SidesChanged);
        let amount = text_input("amount", &self.amount_input).on_input(Message::AmountChanged);

        let roll = button("roll").on_press(Message::DiceRolled);
        
        let result = text(&self.dice_result);
        
        let interface = column![sides, amount, roll, result];
        
        interface
    }
}