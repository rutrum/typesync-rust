//use chrono::Utc;
use seed::{prelude::*, *};
use std::time::Duration;
use typesync::{ScoreRecord, Song, TestMode};

use crate::song_summary;
use crate::Msg as SuperMsg;
use crate::Page;

#[derive(Clone, Debug)]
pub struct Model {
    song: Song,
    name: String,
    time: Duration,
    wpm: f32,
    mode: TestMode,
}

pub fn init(time: Duration, wpm: f32, song: Song, mode: TestMode) -> Model {
    Model {
        name: String::new(),
        time,
        wpm,
        song,
        mode,
    }
}

pub enum Msg {
    UpdateName(String),
    Submit,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<SuperMsg>) {
    use Msg::*;
    match msg {
        UpdateName(s) => model.name = s,
        Submit => {
            let score = ScoreRecord {
                name: model.name.clone(),
                genius_id: model.song.genius_id.clone(),
                milliseconds: model.time.as_millis(),
                absolute_time: 0, //Utc::now().timestamp(),
                mode: model.mode,
            };
            orders.send_msg(SuperMsg::ChangePage(Page::Summary(song_summary::init(
                Some(model.song.clone()),
            ))));
        }
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    let time = format!("{:.*}", 2, model.time.as_millis() as f32 / 1000.0);
    let wpm = format!("{:.*}", 1, model.wpm);

    div![
        h2!["Finished!"],
        p![format!("{} seconds", time)],
        p![format!("{} wmp", wpm)],
        input![attrs!(
            At::Type => "text",
            At::AutoComplete => "off",
            At::Placeholder => "Enter your name:",
        )],
        button!["Submit", ev(Ev::Click, |_| Msg::Submit)],
    ]
}
