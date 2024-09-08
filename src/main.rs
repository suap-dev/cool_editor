#![warn(clippy::pedantic)]

use iced::widget::{column, container, horizontal_space, row, text, text_editor};
use iced::{executor, Command};
use iced::{Application, Element, Length, Settings, Theme};

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

impl Application for Editor {
    type Message = EditorMessage;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Editor, iced::Command<Self::Message>) {
        (
            Self {
                content: text_editor::Content::with(include_str!("main.rs")),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("A cool editor!")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            EditorMessage::Edit(action) => self.content.edit(action),
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let text_input = text_editor(&self.content).on_edit(EditorMessage::Edit);
        let cursor_position = {
            let (line, column) = self.content.cursor_position();
            text(format!("L:{}, C:{}", line + 1, column + 1))
        };

        let status_bar = row![horizontal_space(Length::Fill), cursor_position];

        container(column![text_input, status_bar].spacing(5))
            .padding(10)
            .into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}
