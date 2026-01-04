
use iced::{Color};
use iced::border::Radius;
use iced::widget::{rule, Rule};

pub fn hrule() -> Rule<'static> {
    rule::horizontal(1)
        .style(|_| { rule::Style {
            color: Color::BLACK,
            radius: Radius::new(0),
            fill_mode: rule::FillMode::Full,
            snap: false,
    }})
}
