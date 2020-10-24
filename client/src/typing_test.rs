use seed::{prelude::*, *};
use std::time::Duration;
use wasm_timer::Instant;

use crate::Model as SuperModel;
use crate::Msg as SuperMsg;

#[derive(Clone, Debug)]
pub struct Model {
    typing_buffer: String, // stores the users keystrokes
    accurate: bool,        // is typing_buffer align with current_lyric?
    finished_line: bool,   // did the user just complete a line? used for events
    lyrics: Vec<String>,   // every line of the lyrics the user is to type
    time: Option<Instant>, // time the test starts
    total_chars: usize,
}

pub fn init(model: &SuperModel) -> Model {
    let song = model.song.as_ref().unwrap();
    let mode = model.mode;
    let lyrics = song.lyrics(mode).lyrics.clone();

    Model {
        typing_buffer: String::new(),
        accurate: true,
        finished_line: false,
        lyrics,
        time: None,
        total_chars: song.lyrics(mode).stats.total,
    }
}

pub enum Msg {
    KeyPress(String),
    InputChange(String),
    Tick,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<SuperMsg>) {
    use Msg::*;
    match msg {
        KeyPress(key) => {
            // Start timer if necessary
            if model.time.is_none() {
                model.time = Some(Instant::now());
                orders.send_msg(SuperMsg::TypingTest(Msg::Tick));
            }

            model.accurate = model.typing_buffer == model.lyrics[0];

            let line_complete = model.accurate && (key == " " || key == "Enter");
            let final_line_complete = model.lyrics.len() == 1
                && (format!("{}{}", model.typing_buffer, key) == model.lyrics[0]);

            if line_complete || final_line_complete {
                model.finished_line = true;
                model.lyrics.remove(0);
                model.typing_buffer.clear();
            }

            if model.lyrics.is_empty() {
                let final_time = model.time.unwrap().elapsed();
                let words = model.total_chars as f32 / 6.0;
                let minutes = final_time.as_secs_f32() / 60.0;
                orders.send_msg(SuperMsg::TestDone(final_time, words / minutes));
            }
        }

        Tick => {
            // Call the next tick (this tick will cause a rerender)
            orders.perform_cmd(cmds::timeout(10, || SuperMsg::TypingTest(Msg::Tick)));
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

pub fn view(model: &Model) -> Node<Msg> {
    let mut iter = model.lyrics.iter();
    let top = iter.next();
    let bottom = iter.next();

    let timer = match model.time {
        Some(time) => format!("{:.*}", 2, time.elapsed().as_millis() as f32 / 1000.0),
        None => format!("0.00"),
    };

    div![
        div![timer],
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
        div![div![id!["top-line"], top], div![id!["bot-line"], bottom],],
    ]
}
