
#![allow(dead_code, unused)]

use std::path::PathBuf;
use std::fs;

use iced::{Element, Padding, Length, Border, Color};
use iced::border::Radius;
use iced::widget::{row, Row, text, mouse_area, column, container, rule, Space, button, TextEditor, text_editor, svg, Svg};
use iced::widget::container::Style;
use thiserror::Error;

use crate::styles;

pub struct EditorState {
    pub filepath: Option<PathBuf>,
    pub content: text_editor::Content
}

impl EditorState {
    fn new() -> Self {
        EditorState {
            filepath: None,
            content: text_editor::Content::new()
        }
    }
}

pub struct ContentArea {
    editor_state: EditorState,
    preview_file: Option<PathBuf>,
    editor_open: bool,
    preview_open: bool
}

#[derive(Debug, Clone)]
pub enum Message {
    Edit(text_editor::Action),
    OpenFile(PathBuf),
    OpenPreview,
}

impl ContentArea {
    pub fn new() -> Self {
        let mut preview_file = PathBuf::from(styles::TEST_DIR);
        preview_file.push("preview.svg");
        ContentArea {
            editor_state: EditorState::new(),
            preview_file: Some(preview_file),
            editor_open: false,
            preview_open: true,
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Edit(action) => {
                self.editor_state.content.perform(action);
            }

            Message::OpenFile(filepath) => {
                let text = fs::read_to_string(filepath).expect("Could not read file");
                self.editor_state.content = text_editor::Content::with_text(&text);
                self.editor_open = true;
            }

            Message::OpenPreview => {
                self.preview_open = true;
            }
        }
    }

    fn editor_view(&self) -> Element<'_, Message> {
        container(
            text_editor(&self.editor_state.content)
                .placeholder("Editor is not open")
                .on_action(Message::Edit)
        )
        .width(Length::FillPortion(1))
        .height(Length::Fill)
        .into()
    }

    // TODO error handling
    fn preview_view(&self) -> Element<'_, Message> {
        container(
            svg(self.preview_file.as_ref().unwrap())
        )
        .width(Length::FillPortion(1))
        .height(Length::Fill)
        .into()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let mut container = Row::new()
                .width(Length::Fill);

        if self.editor_open {
            container = container.push(self.editor_view());
        }

        if self.preview_open {
            container = container.push(self.preview_view());
        }

        container
            .height(Length::Fill)
            .into()
    }
}
