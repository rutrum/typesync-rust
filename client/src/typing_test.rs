use seed::{prelude::*, *};

use crate::Model as SuperModel;
use crate::Msg as SuperMsg;

#[derive(Clone, Debug)]
pub struct Model {
    typing_buffer: String, // stores the users keystrokes
    accurate: bool,        // is typing_buffer align with current_lyric?
    started: bool,         // has the user started typing?
    finished_line: bool,   // did the user just complete a line? used for events
    lyrics: Vec<String>,
}

pub fn init(model: &SuperModel) -> Model {
    let song = model.song.as_ref().unwrap();
    let mode = model.mode;
    let lyrics = song.lyrics(mode).lyrics.clone();

    Model {
        typing_buffer: String::new(),
        accurate: true,
        started: false,
        finished_line: false,
        lyrics,
    }
}

pub enum Msg {
    KeyPress(String),
    InputChange(String),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<SuperMsg>) {
    use Msg::*;
    match msg {
        KeyPress(key) => {
            log!("Keypress!");
            if key == " " || key == "Enter" {
                if model.typing_buffer == model.lyrics[0] {
                    model.finished_line = true;
                    model.lyrics.remove(0);
                    model.typing_buffer.clear();
                }
            }

            if model.lyrics.is_empty() {
                orders.send_msg(SuperMsg::TestDone);
            }
        }
        InputChange(s) => {
            if model.finished_line {
                model.finished_line = false;
            } else {
                model.typing_buffer = s;
            }
        }
    }
}

pub fn view(model: &Model, _supermodel: &SuperModel) -> Node<Msg> {
    let mut iter = model.lyrics.iter();
    let top = iter.next();
    let bottom = iter.next();

    div![
        input![
            attrs! {
                At::AutoComplete => "off",
                At::Type => "text",
                At::AutoFocus => AtValue::None,
                At::Placeholder => "Type to begin...",
                At::Value => model.typing_buffer,
            },
            C![if model.accurate { "good" } else { "bad" }],
            keyboard_ev(Ev::KeyPress, |ev| Msg::KeyPress(ev.key())), // fires first
            input_ev(Ev::Input, |s| Msg::InputChange(s)),            // fires second
        ],
        div![
            div![format!("\"{}\"", model.typing_buffer)],
            div![id!["top-line"], top],
            div![id!["bot-line"], bottom],
        ],
    ]
}
