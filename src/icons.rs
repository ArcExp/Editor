use iced::widget::text;
use iced::{Element, Font};

pub fn get_icon<'a, Message>(codepoint: char) -> Element<'a, Message> {
    const ICON_FONT: Font = Font::with_name("editor-icons");
    text(codepoint).font(ICON_FONT).into()
}

pub fn new_icon<'a, Message>() -> Element<'a, Message> {
    get_icon('\u{0e801}')
}

pub fn save_icon<'a, Message>() -> Element<'a, Message> {
    get_icon('\u{0e800}')
}

pub fn open_icon<'a, Message>() -> Element<'a, Message> {
    get_icon('\u{0f115}')
}

pub fn undo_icon<'a, Message>() -> Element<'a, Message> {
    get_icon('\u{0e805}')
}

pub fn redo_icon<'a, Message>() -> Element<'a, Message> {
    get_icon('\u{0e806}')
}

pub fn left_align_icon<'a, Message>() -> Element<'a, Message> {
    get_icon('\u{0e803}')
}

pub fn center_align_icon<'a, Message>() -> Element<'a, Message> {
    get_icon('\u{0e804}')
}

pub fn right_align_icon<'a, Message>() -> Element<'a, Message> {
    get_icon('\u{0e802}')
}

pub fn bold_icon<'a, Message>() -> Element<'a, Message> {
    get_icon('\u{0e807}')
}

pub fn underline_icon<'a, Message>() -> Element<'a, Message> {
    get_icon('\u{0f0cd}')
}

pub fn italic_icon<'a, Message>() -> Element<'a, Message> {
    get_icon('\u{0e808}')
}

pub fn bullet_icon<'a, Message>() -> Element<'a, Message> {
    get_icon('\u{0f0ca}')
}
