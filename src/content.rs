
#![allow(dead_code, unused)]

use std::path::PathBuf;
use std::fs;
use std::io;

use iced::{Element, Padding, Length, Border, Color, Background, Theme};
use iced::border::Radius;
use iced::widget::{row, Row, text, mouse_area, column, container, rule, 
                   Space, button, TextEditor, text_editor, svg, Svg, Column,
                   scrollable};
use iced::widget::container::Style;
use thiserror::Error;

use crate::error::TypstError;
use crate::styles;
use crate::typst::{TypstContext, RenderResult};

pub struct ContentArea {
    open_file: Option<PathBuf>,
    content: text_editor::Content,
    preview_files: Vec<PathBuf>,
    pub editor_open: bool,
    pub preview_open: bool
}

#[derive(Debug, Clone)]
pub enum Message {
    Edit(text_editor::Action),
    OpenFile(PathBuf),
    OpenPreview,
}

impl ContentArea {
    pub fn new() -> Self {
        ContentArea {
            open_file: None,
            preview_files: vec!(),
            content: text_editor::Content::new(),
            editor_open: false,
            preview_open: true,
        }
    }

    pub fn update(&mut self, message: Message, typst: &mut TypstContext) {
        match message {
            Message::Edit(action) => {
                self.render(typst);
                self.content.perform(action);
            }
            Message::OpenFile(filepath) => {
                let text = fs::read_to_string(filepath).expect("Could not read file");
                self.content = text_editor::Content::with_text(&text);
                self.editor_open = true;
            }
            Message::OpenPreview => {
                self.preview_open = true;
            },
        }
    }

    fn editor_view(&self) -> Element<'_, Message> {
        container(
            text_editor(&self.content)
                .placeholder("")
                .on_action(Message::Edit)
                .style(|theme: &Theme, _status| {
                    let palette = theme.extended_palette();
                    text_editor::Style {
                        background: Background::Color(palette.background.base.color),
                        border: Border { 
                            radius: 1.0.into(),
                            width: 0.0,
                            color: palette.background.strong.color,
                        },
                        placeholder: palette.secondary.base.color,
                        value: palette.background.base.text,
                        selection: palette.primary.weak.color,
                    }
                })
        )
        .width(Length::FillPortion(1))
        .height(Length::Fill)
        .into()
    }

    fn render(&mut self, typst: &mut TypstContext) {
        // No file is open - so we can't render anything
        if self.open_file.is_none() {
            return
        }

        let open_file = self.open_file.as_ref().unwrap();
        match typst.render(&self.content.text(), open_file) {
            RenderResult::Debounce => (),
            RenderResult::Error(msg) => (), // TODO error handling
            RenderResult::Success(files) => self.preview_files = files,
        }
    }

    // TODO error handling
    fn preview_view(&self) -> Element<'_, Message> {

        let mut svgs = column![].clip(false);
        for file in self.preview_files.iter() {
            let image = svg(&file);
            svgs = svgs.push(image);
        }
        container( scrollable( svgs ))
        .width(Length::FillPortion(1))
        .height(Length::Fill)
        .style(|_| container::Style {
            background: Some(Background::Color(Color {r:0.9, g: 0.9, b: 0.9, a: 1.0})),
            ..container::Style::default()
        })
        .into()
    }

    pub fn view(&self, typst: &TypstContext) -> Element<'_, Message> {
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
