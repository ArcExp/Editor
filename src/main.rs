use iced::highlighter::{self, Highlighter};
use iced::theme;
use iced::{executor, keyboard};

use iced::widget::{
    button, column, container, horizontal_space, pick_list, row, text, text_editor, tooltip,
};
use iced::{Application, Command, Element, Font, Length, Settings, Subscription, Theme};

use std::path::{Path, PathBuf};
use std::sync::Arc;

use icons::*;
mod icons;

use file_handling::*;
mod file_handling;

fn main() -> iced::Result {
    // Initialize the Editor application
    Editor::run(Settings {
        default_font: Font::MONOSPACE,
        fonts: vec![include_bytes!("../fonts/icons.ttf").as_slice().into()],
        ..Settings::default()
    })
}

// Define the Editor struct to manage the editor state
struct Editor {
    path: Option<PathBuf>,         // Path to the opened file
    content: text_editor::Content, // Text editor content
    error: Option<Error>,          // Error state
    theme: highlighter::Theme,     // Theme for syntax highlighting
    is_dirty: bool,                // Flag to track if content is dirty (changed)
}

// Define the message enum for handling editor actions
#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),                         // Text editor actions
    New,                                               // New file action
    Open,                                              // Open file action
    FileOpened(Result<(PathBuf, Arc<String>), Error>), // File opened action
    Save,                                              // Save file action
    FileSaved(Result<PathBuf, Error>),                 // File saved action
    ThemeSelected(highlighter::Theme),                 // Theme selection action
}

// Implement the Application trait for Editor
impl Application for Editor {
    // Initialize the editor and load default file on startup
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let editor = Self {
            path: None,
            content: text_editor::Content::new(),
            error: None,
            theme: highlighter::Theme::SolarizedDark,
            is_dirty: true,
        };

        let command = Command::perform(load_file(default_file()), Message::FileOpened);
        (editor, command)
    }

    fn title(&self) -> String {
        String::from("A cool editor!")
    }

    // Update the editor state based on messages received
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Edit(action) => {
                self.is_dirty = self.is_dirty || action.is_edit();
                self.error = None;

                self.content.edit(action);

                Command::none()
            }
            Message::New => {
                self.path = None;
                self.content = text_editor::Content::new();
                self.is_dirty = true;
                Command::none()
            }
            Message::Open => Command::perform(pick_file(), Message::FileOpened),
            Message::FileOpened(Ok((path, content))) => {
                self.path = Some(path);
                self.content = text_editor::Content::with(&content);
                self.is_dirty = false;
                Command::none()
            }
            Message::Save => {
                let text = self.content.text();

                Command::perform(save_file(self.path.clone(), text), Message::FileSaved)
            }
            Message::FileSaved(Ok(path)) => {
                self.path = Some(path);
                self.is_dirty = false;
                Command::none()
            }
            Message::FileSaved(Err(error)) => {
                self.error = Some(error);
                Command::none()
            }
            Message::FileOpened(Err(error)) => {
                self.error = Some(error);
                Command::none()
            }
            Message::ThemeSelected(theme) => {
                self.theme = theme;
                Command::none()
            }
        }
    }

    // Define subscriptions (keyboard events)
    fn subscription(&self) -> Subscription<Message> {
        keyboard::on_key_press(|key_code, modifiers| match key_code {
            keyboard::KeyCode::S if modifiers.command() => Some(Message::Save),
            _ => None,
        })
    }

    // Define the UI elements and their interactions
    fn view(&self) -> Element<'_, Message> {
        let controls = row![
            action(new_icon(), "New file", Some(Message::New)),
            action(
                save_icon(),
                "Save file",
                self.is_dirty.then_some(Message::Save)
            ),
            action(open_icon(), "Open file", Some(Message::Open)),
            action(left_align_icon(), "Left", None),
            action(center_align_icon(), "Centre", None),
            action(right_align_icon(), "Right", None),
            action(bold_icon(), "Bold", None),
            action(italic_icon(), "Italics", None),
            action(underline_icon(), "Underline", None),
            action(bullet_icon(), "Bullet Point", None),
            horizontal_space(Length::Fill),
            pick_list(
                highlighter::Theme::ALL,
                Some(self.theme),
                Message::ThemeSelected
            )
        ]
        .spacing(10);

        let input = text_editor(&self.content)
            .on_edit(Message::Edit)
            .highlight::<Highlighter>(
                highlighter::Settings {
                    theme: self.theme,
                    extension: self
                        .path
                        .as_ref()
                        .and_then(|path| path.extension()?.to_str())
                        .unwrap_or("rs")
                        .to_string(),
                },
                |highlight, _theme| highlight.to_format(),
            );

        let word_count = self.content.text().split_whitespace().count();
        let status_bar = {
            let status = if let Some(Error::IOFailed(error)) = self.error.as_ref() {
                text(error.to_string())
            } else {
                match self.path.as_deref().and_then(Path::to_str) {
                    Some(path) => text(path).size(14),
                    None => text("New file"),
                }
            };

            let word_count_display = text(format!("Words: {}", word_count));

            row![status, horizontal_space(Length::Fill), word_count_display]
        };

        container(column![controls, input, status_bar].spacing(10))
            /* Container is a widget that must contain all other widgets used in the app.
            .into() converts the constructed widget hierarchy into the final Element that represents the app's UI.
             Note: In iced, Element types are the primary type to be rendered or displayed. */
            .padding(10)
            .into()
    }

    fn theme(&self) -> Theme {
        match self.theme.is_dark() {
            true => Theme::Dark,
            false => Theme::Light,
        }
    }
}

// Define a helper function for creating UI actions (buttons, etc.)
fn action<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    label: &'a str,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let action = button(container(content).width(25).center_x());

    if let Some(on_press) = on_press {
        tooltip(
            action.on_press(on_press),
            label,
            tooltip::Position::FollowCursor,
        )
        .style(theme::Container::Box)
        .into()
    } else {
        action.style(theme::Button::Secondary).into()
    }
}
