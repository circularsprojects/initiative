use iced::widget::svg::{Handle, Svg};
use std::sync::LazyLock;
use iced::Color;
use iced::widget::svg;

pub static PEN_LINE: LazyLock<Handle> =
    LazyLock::new(|| Handle::from_memory(include_bytes!("assets/pen-line.svg")));

pub fn pen_line() -> Svg<'static> {
    Svg::new(PEN_LINE.clone()).width(16).style(|_theme, _status| svg::Style {
        color: Some(Color::WHITE),
    })
}