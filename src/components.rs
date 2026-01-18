
use iced::{Color, Length};
use iced::border::Radius;
use iced::widget::{rule, Rule, Container, container};
use iced::alignment::Horizontal;

pub fn hrule() -> Rule<'static> {
    rule::horizontal(2)
        .style(|_| { rule::Style {
            color: Color::BLACK,
            radius: Radius::new(0),
            fill_mode: rule::FillMode::Full,
            snap: false,
    }})
}

pub fn left_border<M: Send + 'static>(color: Color) -> Container<'static, M> {
    container (
        rule::vertical(2)
            .style(move |_| { rule::Style {
                color,
                radius: Radius::new(0),
                fill_mode: rule::FillMode::Full,
                snap: false,
        }})
    )
    .width(Length::Fill)
    .align_x(Horizontal::Right)
}
