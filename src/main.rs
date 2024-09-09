#![warn(clippy::pedantic)]

use std::io;
use std::path::Path;
use std::sync::Arc;

use iced::executor;
use iced::widget::{column, container, horizontal_space, row, text, text_editor};
use iced::{Application, Command, Element, Length, Settings, Theme};

fn main() -> iced::Result {
    Editor::run(Settings::default())
}

struct Editor {
    content: text_editor::Content,
}
#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
    FileOpened(Result<Arc<String>, io::ErrorKind>),
}

impl Application for Editor {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Editor, iced::Command<Message>) {
        (
            Self {
                content: text_editor::Content::new(),
            },
            Command::perform(
                load_file(format!("{}/src/main.rs", env!("CARGO_MANIFEST_DIR"))),
                Message::FileOpened,
            ),
        )
    }

    fn title(&self) -> String {
        String::from("A cool editor!")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Edit(action) => self.content.edit(action),
            Message::FileOpened(result) => {
                if let Ok(file_content) = result {
                    self.content = text_editor::Content::with(&file_content);
                }
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let content = {
            let text_editor = text_editor(&self.content).on_edit(Message::Edit);
            let status_bar = {
                let cursor_status = {
                    let (line, column) = self.content.cursor_position();
                    text(format!("L:{}, C:{}", line + 1, column + 1))
                };
                row![horizontal_space(Length::Fill), cursor_status]
            };
            column![text_editor, status_bar].spacing(5)
        };
        container(content).padding(10).into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

async fn load_file(path: impl AsRef<Path>) -> Result<Arc<String>, io::ErrorKind> {
    tokio::fs::read_to_string(path)
        .await
        .map(Arc::new)
        .map_err(|e| e.kind())
}
