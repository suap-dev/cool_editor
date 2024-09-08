#![warn(clippy::pedantic)]

use iced::{widget::text, Element, Sandbox, Settings};

fn main() -> iced::Result {
    Editor::run(Settings::default())
}

#[derive(Debug)]
enum EditorMessage {}
struct Editor;
impl Sandbox for Editor {
    type Message = EditorMessage;

    fn new() -> Self {
        Self {}
    }

    fn title(&self) -> String {
        String::from("A cool editor!")
    }

    fn update(&mut self, _message: Self::Message) {
        todo!()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        text("Hello, iced!").into()
    }
}
