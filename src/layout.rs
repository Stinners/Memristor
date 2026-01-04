#![allow(dead_code, unused)]

use iced::widget::{text, row, responsive, container};
use iced::widget::pane_grid::{self, PaneGrid, Axis};
use iced::{Element, Fill};

use crate::filetree::{self, FileTree};

#[derive(Debug, Clone)]
pub enum Message {
    PaneClicked(pane_grid::Pane),
    PaneResized(pane_grid::ResizeEvent),
    FiletreeMessage(filetree::Message),
}

pub struct Layout {
    panes: pane_grid::State<Pane>,
    focus: Option<pane_grid::Pane>,
    menu_pane: Option<pane_grid::Pane>,
    filetree: FileTree,
}

#[derive(Clone, Copy)]
struct Pane {
    id: i64,
}

const MIN_RATIO: f32 = 0.2;
const MAX_RATIO: f32 = 0.8;

impl Layout {
    fn new() -> Self {
        let (mut panes, pane) = pane_grid::State::new(Pane{id: 0});
        panes.split(Axis::Vertical, pane, Pane{id: 1});

        let menu_pane = Some(pane);

        Layout {
            panes,
            focus: None,
            menu_pane,
            filetree: FileTree::new(),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::PaneClicked(pane) => { 
                self.focus = Some(pane);
            }
            Message::PaneResized(pane_grid::ResizeEvent { split, ratio }) => {
                if ratio > MIN_RATIO && ratio < MAX_RATIO {
                    self.panes.resize(split, ratio);
                }
            }
            Message::FiletreeMessage(filetree::Message::CollapseMenu) => {
                self.menu_pane.map(|pane| {
                    self.panes.close(pane);
                });
                self.menu_pane = None;
            }
            Message::FiletreeMessage(message) => { self.filetree.update(message) },
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        //let focus = self.focus;

        let pane_grid = PaneGrid::new(&self.panes, |_id, pane, _is_maximized| {
            //let is_focused = focus == Some(id);

            pane_grid::Content::new(responsive(move |_| {
                if pane.id == 0 {
                    self.filetree.view().map(Message::FiletreeMessage)
                }
                else {
                    content2()
                }
            }))
        })
        .width(Fill)
        .height(Fill)
        .spacing(10)
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
