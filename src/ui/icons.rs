use iced::widget::svg::{Handle, Svg};
use std::sync::LazyLock;
use iced::Color;
use iced::widget::svg;

pub static PEN_LINE: LazyLock<Handle> =
    LazyLock::new(|| Handle::from_memory(include_bytes!("assets/pen-line.svg")));

pub static UNSKILLED: LazyLock<Handle> =
    LazyLock::new(|| Handle::from_memory(include_bytes!("assets/unskilled.svg")));

pub static HALF: LazyLock<Handle> =
    LazyLock::new(|| Handle::from_memory(include_bytes!("assets/half.svg")));

pub static PROFICIENT: LazyLock<Handle> =
    LazyLock::new(|| Handle::from_memory(include_bytes!("assets/proficient.svg")));

pub static EXPERTISE: LazyLock<Handle> =
    LazyLock::new(|| Handle::from_memory(include_bytes!("assets/expertise.svg")));

pub fn pen_line() -> Svg<'static> {
    Svg::new(PEN_LINE.clone()).width(16).style(|_theme, _status| svg::Style {
        color: Some(Color::WHITE),
    })
}

pub fn unskilled() -> Svg<'static> {
    Svg::new(UNSKILLED.clone()).width(16).style(|_theme, _status| svg::Style {
        color: Some(Color::WHITE),
    })
}

pub fn half() -> Svg<'static> {
    Svg::new(HALF.clone()).width(16).style(|_theme, _status| svg::Style {
        color: Some(Color::WHITE),
    })
}

pub fn proficient() -> Svg<'static> {
    Svg::new(PROFICIENT.clone()).width(16).style(|_theme, _status| svg::Style {
        color: Some(Color::WHITE),
    })
}

pub fn expertise() -> Svg<'static> {
    Svg::new(EXPERTISE.clone()).width(16).style(|_theme, _status| svg::Style {
        color: Some(Color::WHITE),
    })
}