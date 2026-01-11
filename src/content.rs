
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
    preview_dir: Option<PathBuf>,
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
        let mut preview_dir = PathBuf::from(styles::TEST_DIR);
        ContentArea {
            editor_state: EditorState::new(),
            preview_dir: Some(preview_dir),
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

    fn get_preview_files(&self) -> io::Result<Vec<PathBuf>> {
        if self.preview_dir.is_none() {
            return Ok(vec!());
        }
        let dir = self.preview_dir.as_ref().unwrap();
        let mut svgs = vec!();
        let dir_contents = fs::read_dir(dir)?;
        for entry in dir_contents {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                let filename: PathBuf = entry.file_name().into();
                if filename.extension().is_some_and(|ext| ext == "svg") {
                svgs.push(entry.path());
                }
            }
        }
        Ok(svgs)
    }

    // TODO error handling
    fn preview_view(&self) -> Element<'_, Message> {
        let mut svgs = column![].clip(false);
        match self.get_preview_files() {
            Err(_) => {}
            Ok(files) => {
                for file in files.iter() {
                    let image = svg(&file);
                    svgs = svgs.push(image);
                }
            }
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
