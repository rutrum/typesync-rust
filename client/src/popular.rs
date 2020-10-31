use seed::{prelude::*, *};
use crate::{Model, Msg};

pub fn view(model: &Model) -> Node<Msg> {
    div![
        model.popular.iter().map(|popular| {
            let song = &popular.song;
            div![
                C!["popular-song"],
                img![attrs!(At::Alt => "album art", At::Src => song.img_url),],
                div![
                    C!["details"],
                    div![
                        h1![C!["song-title"], &song.title],
                        h2![C!["song-artist"], &song.artist],
                    ],
                    div![C!["plays"], format!("{} play{}",popular.plays, if popular.plays == 1 { "" } else { "s" })],
                ]
            ]
        })
    ]
}
