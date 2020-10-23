use seed::{prelude::*, *};
use typesync::Song;

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
