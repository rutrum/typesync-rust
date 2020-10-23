use seed::{prelude::*, *};
use typesync::{Lyrics, Song, TestMode};

use crate::Msg;

pub fn view(maybe_song: &Option<Song>) -> Node<Msg> {
    match maybe_song {
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
                    //ev(Ev::Change, |_| Msg::???)
                    test_mode_view(TestMode::Standard, &song.tests.standard),
                    test_mode_view(TestMode::Simple, &song.tests.simple),
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
        C!["mode"],
        div![format!("{:?}", mode)],
        div![format!("{:?}", lyrics.difficulty)],
        div![
            C!["stats"],
            div![format!("{} chars", lyrics.stats.total)],
            div![format!("Level {} song", lyrics.score)],
        ],
    ]
}
