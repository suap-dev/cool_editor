#![warn(clippy::pedantic)]

use iced::widget::text_editor;
use iced::{Element, Sandbox, Settings};

fn main() -> iced::Result {
    Editor::run(Settings::default())
}

struct Editor {
    content: text_editor::Content,
}

#[derive(Debug, Clone)]
enum EditorMessage {
    Edit(text_editor::Action),
}

impl Sandbox for Editor {
    type Message = EditorMessage;

    fn new() -> Self {
        Self {
            content: text_editor::Content::with("Edit me!"),
        }
    }

    fn title(&self) -> String {
        String::from("A cool editor!")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            EditorMessage::Edit(action) => {
                self.content.edit(action);
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        text_editor(&self.content)
            .on_edit(EditorMessage::Edit)
            .into()
    }
}
