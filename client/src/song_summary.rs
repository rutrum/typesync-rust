use seed::{prelude::*, *};
use typesync::{Leaderboards, Lyrics, ScoreRecord, Song, TestMode};
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
                    IF!(model.mode.is_some() => button![ev(Ev::Click, start_test), "Start!"])
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

fn leaderboard_view(leaderboard: &[ScoreRecord], mode: TestMode) -> Node<Msg> {
    let title = format!("{:?}", mode);
    table![
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
                td![&score.absolute_time],
                td![format!("{:.*}", 2, score.milliseconds as f32 / 1000.0)],
            ]
        })
    ]
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
