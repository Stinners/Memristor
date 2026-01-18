#![allow(dead_code, unused)]

use std::path::PathBuf;
use std::fs;
use std::time::{Instant, Duration};

use iced::{Element, Length, Border, Color, Background, Theme, Task};
use iced::widget::{Row,column, container, text_editor, svg, scrollable};

use crate::error::TypstError;
use crate::typst::TypstContext;

const SECONDS_BETWEEN_RENDER: u64 = 5;

pub struct ContentArea {
    open_file: Option<PathBuf>,
    content: text_editor::Content,
    preview_files: Vec<PathBuf>,
    next_render: Instant,
    pub editor_open: bool,
    pub preview_open: bool
}

#[derive(Debug, Clone)]
pub enum Message {
    Edit(text_editor::Action),
    OpenFile(PathBuf),
    OpenPreview,
    RenderDone(Result<(), TypstError>),
}

impl ContentArea {
    pub fn new() -> Self {
        ContentArea {
            open_file: None,
            preview_files: vec!(),
            content: text_editor::Content::new(),
            next_render: Instant::now(),
            editor_open: false,
            preview_open: true,
        }
    }

    fn set_render_task(&mut self, typst: &TypstContext) -> Task<Message> {
        // No file is open so we can't render anything
        if self.open_file.is_none() {
            return Task::none();
        }

        // Debounce
        let now = Instant::now();
        if now < self.next_render {
            return Task::none();
        }
        else {
            let next_render = now.checked_add(Duration::from_secs(SECONDS_BETWEEN_RENDER)).unwrap();
            self.next_render = next_render;
        }

        // Render
        let preview_path = typst.preview_path.clone();
        let content = self.content.text();
        let open_file = self.open_file.as_ref().unwrap().clone();
        Task::perform(
            TypstContext::compile(preview_path, content, open_file),
            |result| Message::RenderDone(result)
        )
    }

    pub fn update(&mut self, message: Message, typst: &TypstContext) -> Task<Message> {
        match message {
            Message::Edit(action) => {
                self.content.perform(action);
                self.set_render_task(typst)
            }
            Message::OpenFile(filepath) => {
                let text = fs::read_to_string(filepath).expect("Could not read file");
                self.content = text_editor::Content::with_text(&text);
                self.editor_open = true;
                self.set_render_task(typst)
            }
            Message::OpenPreview => {
                self.preview_open = true;
                self.set_render_task(typst)
            }
            Message::RenderDone(result) => {
                match result {
                    Err(_) => {} // TODO handle error
                    Ok(()) => {
                        match typst.get_preview_files() {
                            Err(_) => {},
                            Ok(files) => self.preview_files = files,
                        }
                    }
                }
                Task::none()
            }
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
