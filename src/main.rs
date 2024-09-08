#![warn(clippy::pedantic)]

use std::io;
use std::path::Path;
use std::sync::Arc;

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
    FileOpened(Result<Arc<String>, io::ErrorKind>),
}

impl Application for Editor {
    type Message = EditorMessage;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Editor, iced::Command<Self::Message>) {
        (
            Self {
                content: text_editor::Content::new(),
            },
            Command::perform(
                load_file(format!("{}/src/main.rs", env!("CARGO_MANIFEST_DIR"))),
                Self::Message::FileOpened,
            ),
        )
    }

    fn title(&self) -> String {
        String::from("A cool editor!")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Self::Message::Edit(action) => self.content.edit(action),
            Self::Message::FileOpened(result) => {
                if let Ok(file_content) = result {
                    self.content = text_editor::Content::with(&file_content);
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let text_input = text_editor(&self.content).on_edit(Self::Message::Edit);
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

async fn load_file(path: impl AsRef<Path>) -> Result<Arc<String>, io::ErrorKind> {
    tokio::fs::read_to_string(path)
        .await
        .map(Arc::new)
        .map_err(|e| e.kind())
}
