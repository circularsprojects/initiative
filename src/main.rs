mod dnd;

use iced::widget::{button, text, column, Column, text_input, row, Row, TextInput};
use strum::IntoEnumIterator;
use crate::dnd::dice::Dice;

pub fn main() -> iced::Result {
    iced::application(State::default, State::update, State::view).run()
}

struct State {
    sides_input: String,
    amount_input: String,
    dice_result: String,
    stats: dnd::stats::Stats,
}

#[derive(Debug, Clone)]
enum Message {
    SidesChanged(String),
    AmountChanged(String),
    StatChanged(i16, dnd::stats::StatType),
    DiceRolled,
}

impl State {
    fn default() -> Self {
        State {
            sides_input: "".parse().unwrap(),
            amount_input: "".parse().unwrap(),
            dice_result: "".parse().unwrap(),
            stats: dnd::stats::Stats::default(),
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SidesChanged(sides) => { self.sides_input = sides; }
            Message::AmountChanged(amount) => { self.amount_input = amount; }
            Message::StatChanged(value, stat) => {
                self.stats.get(stat).set_score(value);
            }
            Message::DiceRolled => {
                if let Ok(sides) = self.sides_input.parse::<u16>() && let Ok(amount) = self.amount_input.parse::<u16>() {
                    let dice = Dice::new(amount, sides);
                    self.dice_result = dice.roll().to_string();
                } else {
                    self.dice_result = "invalid".to_string();
                }
            }
        }
    }

    fn view(&self) -> Row<'_, Message> {
        let dice_column = self.dice_column();
        let stats_column = self.stats_column();

        row![dice_column, stats_column]
    }

    fn dice_column(&self) -> Column<'_, Message> {
        let amount = text_input("amount", &self.amount_input).on_input(Message::AmountChanged);
        let sides = text_input("sides", &self.sides_input).on_input(Message::SidesChanged);

        let roll = button("roll").on_press(Message::DiceRolled);
        
        let result = text(&self.dice_result);
        
        let column = column![amount, sides, roll, result];
        
        column
    }

    fn stats_column(&self) -> Column<'_, Message> {
        let mut column = column![];
        for stat in dnd::stats::StatType::iter() {
            let score = self.stats.get(stat).score();
            let modifier = self.stats.get(stat).get_modifier();
            let input: TextInput<Message> = text_input("10", &score.to_string()).on_input(move |value: String| {
                if let Ok(value) = value.parse::<i16>() {
                    Message::StatChanged(value, stat)
                } else {
                    Message::StatChanged(10, stat)
                }
            });
            let row: Row<Message> = row![
                text(format!("{:?}", stat)),
                input,
                text(format!("(modifer: {})", modifier))
            ];
            // column = column.push(text(format!("{:?}: {} (modifier: {})", stat, score, modifier)));
            column = column.push(row);
        }
        column
    }
}