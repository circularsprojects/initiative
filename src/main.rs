mod dnd;
pub mod ui;

use iced::{Center};
use iced::widget::{button, text, column, Column, text_input, row, Row, TextInput, container, Button, space};
use strum::IntoEnumIterator;
use crate::dnd::dice::Dice;
use crate::dnd::utils::PrefixDisplay;
use crate::ui::icons::pen_line;

pub fn main() -> iced::Result {
    iced::application(State::default, State::update, State::view).run()
}

struct State {
    sides_input: String,
    amount_input: String,
    dice_result: String,
    stats: dnd::stats::Stats,
    edited_stat: Option<dnd::stats::StatType>,
    edit_buffer: String,
}

#[derive(Debug, Clone)]
enum Message {
    SidesChanged(String),
    AmountChanged(String),
    DiceRolled,
    EditStarted(dnd::stats::StatType),
    EditChanged(String),
    EditSubmitted,
}

impl State {
    fn default() -> Self {
        State {
            sides_input: "".parse().unwrap(),
            amount_input: "".parse().unwrap(),
            dice_result: "".parse().unwrap(),
            stats: dnd::stats::Stats::default(),
            edited_stat: None,
            edit_buffer: "".parse().unwrap(),
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SidesChanged(sides) => { self.sides_input = sides; }
            Message::AmountChanged(amount) => { self.amount_input = amount; }
            Message::DiceRolled => {
                if let Ok(sides) = self.sides_input.parse::<u16>() && let Ok(amount) = self.amount_input.parse::<u16>() {
                    let dice = Dice::new(amount, sides);
                    self.dice_result = dice.roll().to_string();
                } else {
                    self.dice_result = "invalid".to_string();
                }
            }
            Message::EditStarted(stat) => {
                self.edited_stat = Some(stat);
                self.edit_buffer = self.stats.get(stat).score().to_string();
            }
            Message::EditChanged(value) => {
                self.edit_buffer = value;
            }
            Message::EditSubmitted => {
                if let Some(stat) = self.edited_stat && let Ok(value) = self.edit_buffer.parse::<i16>() && value > 0 {
                    self.stats.set_score(stat, value);
                }
                self.edited_stat = None;
                self.edit_buffer.clear();
            }
        }
    }

    fn view(&self) -> Row<'_, Message> {
        let dice_column = self.dice_column();
        let stats_column = self.stats_column();

        // row![dice_column, stats_column]
        row![stats_column]
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

            let mut row: Row<Message> = row![
                text(format!("{:?} {} ({})", stat, modifier.prefix_display(), score)),
                space().width(10)
                // text(format!("(modifier: {})", modifier))
            ].align_y(Center);
            if self.edited_stat == Some(stat) {
                let input: TextInput<Message> = text_input("10", &self.edit_buffer).on_input(Message::EditChanged).on_submit(Message::EditSubmitted);
                row = row.push(input);
            } else {
                let edit_button: Button<Message> = button(pen_line()).on_press(Message::EditStarted(stat)).style(button::text).padding(0);
                row = row.push(edit_button);
            }
            // column = column.push(text(format!("{:?}: {} (modifier: {})", stat, score, modifier)));
            let container = container(row)
                .padding(10)
                .style(container::bordered_box);
            column = column.push(container);
        }
        column
    }
}