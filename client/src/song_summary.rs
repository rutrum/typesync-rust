use seed::{prelude::*, *};
use typesync::{Leaderboards, Lyrics, Score, Song, TestMode};
use web_sys::Event;

use crate::Msg as SuperMsg;

#[derive(Clone, Debug, Default)]
pub struct Model {
    mode: Option<TestMode>,
    song: Option<Song>,
    leaderboards: Leaderboards,
}

pub fn init(song: Option<Song>) -> Model {
    Model {
        song,
        ..Default::default()
    }
}

pub enum Msg {
    UpdateMode(TestMode),
    UpdateLeaderboards(Leaderboards),
    StartTest,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<SuperMsg>) {
    use Msg::*;
    match msg {
        UpdateMode(mode) => model.mode = Some(mode),
        UpdateLeaderboards(leaderboards) => model.leaderboards = leaderboards,
        StartTest => {
            if let (Some(mode), Some(song)) = (model.mode, model.song.take()) {
                orders.send_msg(SuperMsg::StartTest(song, mode));
            }
        }
    };
}

fn start_test(ev: Event) -> Msg {
    ev.prevent_default();
    Msg::StartTest
}

fn get_difficulty_class(model: &Model) -> Option<String> {
    if let (Some(mode), Some(song)) = (&model.mode, &model.song) {
        let l = song.lyrics(*mode);
        Some(format!("{:?}", l.difficulty))
    } else {
        None
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    match &model.song {
        Some(song) => div![
            div![
                C!["song-summary"],
                img![
                    attrs!("alt" => "album art", "src" => song.img_url),
                ],
                div![
                    C!["details"],
                    div![
                        h1![C!["song-title"], &song.title],
                        h2![C!["song-artist"], &song.artist],
                    ],
                    form![
                        C!["modes"],
                        test_mode_view(&model.mode, TestMode::Standard, &song.tests.standard),
                        test_mode_view(&model.mode, TestMode::Simple, &song.tests.simple),
                    ],
                ],
                div![
                    C!["go"],
                    C![get_difficulty_class(&model)],
                    IF!(model.mode.is_some() => C!["appear"]),
                    IF!(model.mode.is_none() => C!["disappear"]),
                    ev(Ev::Click, start_test),
                    "Start!",
                ],
            ],
            leaderboard_view(&model.leaderboards.standard, TestMode::Standard),
            leaderboard_view(&model.leaderboards.simple, TestMode::Simple),
        ],
        None => no_song_view(),
    }
}

fn no_song_view() -> Node<Msg> {
    div![
        C!["song-summary"],
        img![attrs!("alt" => "not found!", "src" => "record.png"),],
        div![
            C!["details"],
            h2!["Song not found."],
            p!["Try a different song or check your spelling!"],
        ]
    ]
}

fn leaderboard_view(leaderboard: &[Score], mode: TestMode) -> Node<Msg> {
    let title = format!("{:?}", mode);
    table![
        C!["leaderboard"],
        tr![th![attrs!(At::ColSpan => 4), &title],],
        IF!(leaderboard.is_empty() =>
            tr![td![attrs!(At::ColSpan => 4), format!(
                "Be the first to complete {} mode!",
                title.to_lowercase()
            )]]
        ),
        IF!(!leaderboard.is_empty() => 
            tr![th![], th!["Name"], th!["Completed"], th!["Time"],]),
        leaderboard.iter().enumerate().map(|(i, score)| {
            tr![
                td![i + 1],
                td![&score.name],
                td![&score.date],
                td![format!("{:.*}", 2, score.milliseconds as f32 / 1000.0)],
            ]
        })
    ]
}

fn test_mode_view(selected: &Option<TestMode>, mode: TestMode, lyrics: &Lyrics) -> Node<Msg> {
    let is_selected = if let Some(smode) = selected {
        smode == &mode
    } else {
        false
    };

    label![
        ev(Ev::Click, move |_| Msg::UpdateMode(mode)),
        C!["mode"],
        C![format!("{:?}", lyrics.difficulty)],
        IF!(is_selected => C!["selected"]),

        input![attrs!(At::Type => "radio", At::Name => "mode", At::Value => format!("{:?}", mode)),],
        div![format!("{:?}", mode)],
        div![
            C!["difficulty"],
            C![format!("{:?}", lyrics.difficulty)],
            format!("{:?}", lyrics.difficulty),
        ],
        div![
            C!["stats"],
            div![format!("{} chars", lyrics.stats.total)],
            div![format!("Level {} song", lyrics.score)],
        ],
    ]
}
