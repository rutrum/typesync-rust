use seed::{prelude::*, *};
use typesync::{Lyrics, Song, TestMode};
//use web_sys::KeyboardEvent;

use crate::Msg as SuperMsg;
use crate::Model as SuperModel;

#[derive(Clone, Debug)]
pub struct Model {
    typing_buffer: String,      // stores the users keystrokes
    accurate: bool,             // is typing_buffer align with current_lyric?
    started: bool,              // has the user started typing?
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
        lyrics,
    }
}

pub enum Msg {
    KeyPress(String)
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<SuperMsg>) {
    use Msg::*;
    match msg {
        KeyPress(_) => {
            model.lyrics.remove(0);

            if model.lyrics.is_empty() {
                orders.send_msg(SuperMsg::TestDone);
            }
        }
    }
}

pub fn view(model: &Model, supermodel: &SuperModel) -> Node<Msg> {
    
    let mut iter = model.lyrics.iter();
    let top = iter.next();
    let bottom = iter.next();

    div![
        input![
            attrs!{
                At::AutoComplete => "off",
                At::Type => "text",
                At::AutoFocus => AtValue::None, 
                At::Placeholder => "Type to begin...",
                At::Value => model.typing_buffer
            },
            ev(Ev::KeyPress, |_| Msg::KeyPress(String::new()))
        ],
        div![
            div![ id!["top-line" ], top ],
            div![ id!["bot-line" ], bottom ],
        ],
    ]
}
