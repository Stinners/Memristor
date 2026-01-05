
#![allow(dead_code, unused)]

use iced::{Element, Padding, Length, Border, Color};
use iced::border::Radius;
use iced::widget::{row, Column, text, mouse_area, column, container, rule, Space, button};
use iced::widget::container::Style;
use thiserror::Error;

pub struct ContentArea {
    editor_open: bool,
    preview_open: bool
}

#[derive(Debug, Clone)]
pub enum Message {
}

impl ContentArea {
    pub fn new() -> Self {
        ContentArea {
            editor_open: false,
            preview_open: false,
        }
    }

    fn update(&mut self, message: Message) {
        todo!()
    }

    fn view(&self) -> Element<'_, Message> {
        column![
            text("Foo")
        ]
        .width(Length::Fill)
        .into()
    }
}
