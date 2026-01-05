#![allow(dead_code)]
#![allow(unused)]

use iced::{Element, Padding, Length, Border, Color};
use iced::border::Radius;
use iced::widget::{row, Column, text, mouse_area, column, container, rule, Space, button};
use iced::widget::container::Style;
use thiserror::Error;

use crate::styles;
use crate::components;

#[derive(Debug, Clone)]
pub enum Message {

    // These two messages are handled at the Layout level 
    CloseMenu,
    OpenMenu,
}


pub struct MenuHeader {}

impl MenuHeader {
    pub fn new() -> Self {
        MenuHeader {}
    }

    pub fn update(&mut self, message: Message) { }

    pub fn view(&self) -> Element<'_, Message> {
        let header_contents = container(
                row![
                    Space::new().width(10),
                    button("Collapse")
                        .on_press(Message::CloseMenu)
                ]
                .spacing(10)
            )
            .center_y(styles::HEADER_HEIGHT);

        column! [
            header_contents,
            components::hrule(),
        ]
        .width(Length::Fill)
        .into()
    }
}

pub struct ContentHeader {
    pub menu_open: bool
}

impl ContentHeader {
    pub fn new(menu_open: bool) -> Self {
        ContentHeader {
            menu_open
        }
    }

    pub fn update(&mut self, message: Message) { 
        match message {
            Message::CloseMenu => { self.menu_open = false },
            Message::OpenMenu => { self.menu_open = true },
        }
    }


    pub fn view(&self) -> Element<'_, Message> {
        let left_buttons = row! [
                Space::new().width(10),
            ]
            .spacing(10);

        let left_buttons = 
            if !self.menu_open { left_buttons.push(button("Expand").on_press(Message::OpenMenu)) }
            else { left_buttons };

        let header_contents = container(
                left_buttons
            )
            .center_y(styles::HEADER_HEIGHT);

        column! [
            header_contents,
            components::hrule(),
        ]
        .width(Length::Fill)
        .into()
    }
}
