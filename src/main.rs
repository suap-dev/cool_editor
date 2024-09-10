#![warn(clippy::pedantic)]
#![allow(dead_code)]

use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use iced::executor;
use iced::widget::{button, column, container, horizontal_space, row, text, text_editor};
use iced::{Application, Command, Element, Length, Settings, Theme};

fn main() -> iced::Result {
    Editor::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Error {
    DialogClosed,
    IO(io::ErrorKind),
}
struct Editor {
    path: Option<PathBuf>,
    content: text_editor::Content,
    error: Option<Error>,
}
#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
    FileOpened(Result<(PathBuf, Arc<String>), Error>),
    Open,
}
impl Application for Editor {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Editor, iced::Command<Message>) {
        (
            Self {
                path: None,
                content: text_editor::Content::new(),
                error: None,
            },
            // Command::none(),
            Command::perform(load_file(default_file()), Message::FileOpened),
        )
    }

    fn title(&self) -> String {
        String::from("A cool editor!")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Edit(action) => {
                self.content.edit(action);
                Command::none()
            }
            Message::Open => Command::perform(pick_file(), Message::FileOpened),
            Message::FileOpened(Ok((path, file_content))) => {
                self.path = Some(path);
                self.content = text_editor::Content::with(&file_content);
                Command::none()
            }
            Message::FileOpened(Err(error)) => {
                self.error = Some(error);
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let content = {
            let controls = {
                let open_file = button("Open file").on_press(Message::Open);
                row![open_file]
            };
            let text_editor = text_editor(&self.content).on_edit(Message::Edit);
            let status_bar = {
                let file_path = match self.path.as_deref().and_then(Path::to_str) {
                    Some(path) => text(path),
                    None => text(""),
                };
                let cursor_position = {
                    let (line, column) = self.content.cursor_position();
                    text(format!("L:{}, C:{}", line + 1, column + 1))
                };
                row![
                    file_path.size(14),
                    horizontal_space(Length::Fill),
                    cursor_position
                ]
            };
            column![controls, text_editor, status_bar].spacing(5)
        };
        container(content).padding(10).into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

fn default_file() -> PathBuf {
    PathBuf::from(format!("{}\\src\\main.rs", env!("CARGO_MANIFEST_DIR")))
}

async fn pick_file() -> Result<(PathBuf, Arc<String>), Error> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Choose a text file...")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    load_file(handle.path().to_owned()).await
}

async fn load_file(path: PathBuf) -> Result<(PathBuf, Arc<String>), Error> {
    let contents = tokio::fs::read_to_string(&path)
        .await
        .map(Arc::new)
        .map_err(|e| e.kind())
        .map_err(Error::IO)?;

    Ok((path, contents))
}
