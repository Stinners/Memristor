#![allow(dead_code, unused)]

use std::path::PathBuf;
use std::env::home_dir;

use iced::widget::{text, row, responsive, container, column};
use iced::widget::pane_grid::{self, PaneGrid, Axis};
use iced::{Element, Fill, Padding, Task};
use rfd::FileDialog;
use tempdir::TempDir;

use crate::filetree::{self, FileTree};
use crate::content::{self, ContentArea};
use crate::header::{self, MenuHeader, ContentHeader};
use crate::styles;
use crate::typst::{TypstContext, RenderResult};

use crate::settings::{Settings, ConfigStore};

#[derive(Debug, Clone)]
pub enum Message {
    PaneClicked(pane_grid::Pane),
    PaneResized(pane_grid::ResizeEvent),
    FiletreeMessage(filetree::Message),
    ContentAreaMessage(content::Message),
    HeaderMessage(header::Message),
}

pub struct Layout {
    // Metadata
    settings: Settings,

    // Pane handling 
    panes: pane_grid::State<Pane>,
    focus: Option<pane_grid::Pane>,
    menu_pane: Option<pane_grid::Pane>,
    content_pane: pane_grid::Pane,

    // Components
    filetree: FileTree,
    content: ContentArea,
    menu_header: MenuHeader,
    content_header: ContentHeader,

    // App level data
    typst: TypstContext,
}

#[derive(Clone, Copy)]
struct Pane {
    id: i64,
}

const MIN_RATIO: f32 = 0.2;
const MAX_RATIO: f32 = 0.8;

fn pick_dir() -> Option<PathBuf> {
    let mut dialog = FileDialog::new()
        .set_can_create_directories(true);
    if let Some(dir) = home_dir() {
        dialog = dialog.set_directory(dir);
    }
    dialog.pick_folder()
}

impl Layout {
    // TODO: think about how to handle errors when setting up the app
    fn new() -> Self {

        // Init app data
        let typst = TypstContext::new().expect("Couldn't create temporary directory");

        // Init Panes 
        let (mut panes, pane) = pane_grid::State::new(Pane{id: 0});
        let (content_pane, menu_content_split) = panes.split(Axis::Vertical, pane, Pane{id: 1}).unwrap();
        let menu_pane = Some(pane);
        panes.resize(menu_content_split, 0.25);

        Layout {
            // TODO error handling
            settings: ConfigStore::init().unwrap().read().unwrap(),
            panes,
            focus: None,
            menu_pane,
            content_pane: content_pane,
            filetree: FileTree::new(),
            content: ContentArea::new(),
            menu_header: MenuHeader::new(),
            content_header: ContentHeader::new(true),

            typst: typst,
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::PaneClicked(pane) => { 
                self.focus = Some(pane);
                Task::none()
            }

            Message::PaneResized(pane_grid::ResizeEvent { split, ratio }) => {
                if ratio > MIN_RATIO && ratio < MAX_RATIO {
                    self.panes.resize(split, ratio);
                }
                Task::none()
            }

            Message::FiletreeMessage(filetree::Message::OpenFile(filepath)) => {
                self.filetree.update(filetree::Message::OpenFile(filepath.clone()));
                self.content.update(content::Message::OpenFile(filepath), &mut self.typst)
                    .map(|future| Message::ContentAreaMessage(future))
            }

            Message::FiletreeMessage(message) => { 
                self.filetree.update(message);
                Task::none()
            }

            Message::HeaderMessage(header::Message::CloseMenu)  => {
                if self.menu_pane.is_some() {
                    self.panes.close(self.menu_pane.unwrap());
                    self.menu_pane = None;
                    self.content_header.update(header::Message::CloseMenu);
                }
                Task::none()
            }

            Message::HeaderMessage(header::Message::OpenMenu)  => {
                if self.menu_pane.is_none() {
                    let (menu_pane, menu_content_split) = self.panes.split(Axis::Vertical, self.content_pane, Pane{id: 0}).unwrap();
                    self.menu_pane = Some(menu_pane);
                    self.content_header.update(header::Message::OpenMenu);
                    self.panes.resize(menu_content_split, 0.25);

                    // TODO there must be a better way to do this
                    self.panes.swap(menu_pane, self.content_pane);
                }
                Task::none()
            }

            Message::HeaderMessage(header::Message::TogglePreview) => {
                self.content.preview_open = !self.content.preview_open;
                Task::none()
            }

            Message::HeaderMessage(header::Message::ToggleEditor) => {
                self.content.editor_open = !self.content.editor_open;
                Task::none()
            }

            Message::HeaderMessage(message) => { todo!() }

            Message::ContentAreaMessage(message) => {
                self.content.update(message, &mut self.typst)
                    .map(|future| Message::ContentAreaMessage(future))
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        //let focus = self.focus;

        let pane_grid = PaneGrid::new(&self.panes, |_id, pane, _is_maximized| {
            //let is_focused = focus == Some(id);

            pane_grid::Content::new(responsive(move |_| {
                if pane.id == 0 {
                    column! [
                        self.menu_header.view().map(Message::HeaderMessage),
                        self.filetree.view().map(Message::FiletreeMessage),
                    ]
                    .into()
                }
                else {
                    column![
                        self.content_header.view().map(Message::HeaderMessage),
                        container(
                            self.content.view(&self.typst).map(Message::ContentAreaMessage)
                        )
                    ]
                    .into()
                }
            }))
        })
        .width(Fill)
        .height(Fill)
        .spacing(0)
        .on_click(Message::PaneClicked)
        .on_resize(10, Message::PaneResized);

        container(pane_grid).into()
    }
}

fn content1() -> Element<'static, Message> {
    row!(text("Pane 1")).into()
}

fn content2() -> Element<'static, Message> {
    row!(text("Pane 2")).into()
}

impl Default for Layout {
    fn default() -> Self {
        Self::new()
    }
}

pub fn layout() -> Layout {
    Layout::default()
}
