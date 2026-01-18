//#![allow(dead_code)]
//#![allow(unused_imports)]

mod layout;
mod filetree;
mod components;
mod styles;
mod content;
mod header;
mod typst;
mod settings;
mod error;

use iced::{self, Element};

use crate::layout::Layout;

// Messages for controling the top-level laout of the app
#[derive(Debug, Clone)]
enum Message {
    LayoutMessage(layout::Message),
}

struct App {
    layout: Layout
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::LayoutMessage(message) => {
                self.layout.update(message)
            }
        }
    }
    fn view(&self) -> Element<'_, Message> {
        Layout::view(&self.layout).map(Message::LayoutMessage)
    }

    fn new() -> Self {
        App {
            layout: Layout::default()
        }
    }
}


pub fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .run()
}
