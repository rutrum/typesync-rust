use seed::{prelude::*, *};
use typesync::{Lyrics, Song, TestMode};

use crate::Msg as SuperMsg;

#[derive(Clone, Debug, Default)]
pub struct Model {
    mode: Option<TestMode>,
    song: Option<Song>,
}

pub fn init(song: Option<Song>) -> Model {
    Model {
        song,
        ..Default::default()
    }
}

pub enum Msg {
    UpdateMode(TestMode),
    StartTest,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<SuperMsg>) {
    use Msg::*;
    match msg {
        UpdateMode(mode) => model.mode = Some(mode),
        StartTest => {
            if let (Some(mode), Some(song)) = (model.mode, model.song.clone()) {
                orders.send_msg(SuperMsg::StartTest(song, mode));
            }
        }
    };
}

pub fn view(model: &Model) -> Node<Msg> {
    match &model.song {
        Some(song) => div![
            C!["song-summary"],
            img![
                attrs!("alt" => "album art", "src" => song.img_url),
                style!("width" => "100px", "height" => "100px"),
            ],
            div![
                C!["details"],
                div![
                    h1![id!["song-title"], &song.title],
                    h2![id!["song-artist"], &song.artist],
                ],
                form![
                    C!["modes"],
                    test_mode_view(TestMode::Standard, &song.tests.standard),
                    test_mode_view(TestMode::Simple, &song.tests.simple),
                    button![ev(Ev::Click, |_| Msg::StartTest), "Start!"]
                ],
            ]
        ],
        None => div![
            C!["song-summary"],
            img![attrs!("alt" => "not found!", "src" => "record.png"),],
            div![
                C!["details"],
                h2!["Song not found."],
                p!["Try a different song or check your spelling!"],
            ]
        ],
    }
}

fn test_mode_view(mode: TestMode, lyrics: &Lyrics) -> Node<Msg> {
    label![
        ev(Ev::Click, move |_| Msg::UpdateMode(mode)),
        C!["mode"],
        input![attrs!("type" => "radio", "name" => "mode", "value" => format!("{:?}", mode)),],
        div![format!("{:?}", mode)],
        div![format!("{:?}", lyrics.difficulty)],
        div![
            C!["stats"],
            div![format!("{} chars", lyrics.stats.total)],
            div![format!("Level {} song", lyrics.score)],
        ],
    ]
}
