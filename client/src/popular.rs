use seed::{prelude::*, *};
use crate::{Msg};
use typesync::SongPlays;

pub fn view(popular: &Vec<SongPlays>) -> Node<Msg> {
    div![
        popular.iter().enumerate().map(|(i,popular)| {
            let song = &popular.song;
            div![
                ev(Ev::Click, move |_| Msg::LoadPopularByIndex(i)),
                C!["popular-song"],
                img![attrs!(At::Alt => "album art", At::Src => song.img_url),],
                div![
                    C!["details"],
                    div![
                        h1![C!["song-title"], &song.title],
                        h2![C!["song-artist"], &song.artist],
                    ],
                    div![
                        C!["plays"], 
                        popular.plays,
                        div![
                            C!["text"],
                            format!(
                                "play{}",
                                if popular.plays == 1 { "" } else { "s" }
                            ),
                        ]
                    ],
                ]
            ]
        })
    ]
}
