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
    OpenDirectory,

    // These are handled in ContentArea
    ToggleEditor,
    TogglePreview,

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
                        .on_press(Message::CloseMenu),
                    button("Open")
                        .on_press(Message::OpenDirectory) 
                ]
                .spacing(10)
            )
            .center_y(styles::HEADER_HEIGHT);

        column! [
            row! [
                header_contents,
                components::left_border(Color { r: 0.8, g: 0.8, b: 0.8, a: 1.0 }),
            ]
            .height(styles::HEADER_HEIGHT),
            components::hrule(),
        ]
        .width(Length::Fill)
        .into()
    }
}

pub struct ContentHeader {
    pub menu_open: bool,
    pub editor_open: bool,
    pub preview_open: bool,
}

impl ContentHeader {
    pub fn new(menu_open: bool) -> Self {
        ContentHeader {
            menu_open,
            editor_open: false,
            preview_open: false,
        }
    }

    pub fn update(&mut self, message: Message) { 
        match message {
            Message::CloseMenu => { self.menu_open = false },
            Message::OpenMenu => { self.menu_open = true },
            Message::ToggleEditor => { self.editor_open != !self.editor_open; },
            Message::TogglePreview => { self.preview_open != self.preview_open; },
            Message::OpenDirectory => { unreachable!("Handled in layout.rs")  }
        }
    }


    pub fn view(&self) -> Element<'_, Message> {
        let mut left_buttons = row! [
                Space::new().width(10),
            ]
            .spacing(10);

        left_buttons = 
            if !self.menu_open { left_buttons.push(button("Expand").on_press(Message::OpenMenu)) }
            else { left_buttons };

        left_buttons = left_buttons.push(
            button("Editor")
                .on_press(Message::ToggleEditor)
        );

        left_buttons = left_buttons.push(
            button("Preview")
                .on_press(Message::TogglePreview)
        );

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
