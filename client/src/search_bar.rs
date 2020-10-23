use typesync::models::SongRequest;
use seed::{prelude::*, *};

use crate::Msg as SuperMsg;
use crate::Model as SuperModel;

#[derive(Clone, Default, Debug)]
pub struct Model {
    pub title: String,
    pub artist: String,
}

pub enum Msg {
    UpdateTitle(String),
    UpdateArtist(String),
    SearchSong,
}

pub fn update(msg: Msg, model: &mut Model) {
    use Msg::*;
    match msg {
        UpdateTitle(s) => model.title = s,
        UpdateArtist(s) => model.artist = s,
        SearchSong => {
            let song_request = SongRequest {
                title: model.title.clone(),
                artist: model.artist.clone(),
            };

            // Post the data
        }
    }
}

pub fn view(model: &SuperModel) -> Node<SuperMsg> {
    form![
        C!["search-box"],
        title_input(),
        artist_input(),
        submit_button(model),
        ev(Ev::Submit, |ev| {
            ev.prevent_default();
            SuperMsg::SearchSong
        })
    ]
}

fn title_input() -> Node<SuperMsg> {
    input![
        id!["song-box"],
        attrs![
            At::Type => "text", 
            At::Name => "title", 
            At::AutoFocus => "",
            At::Placeholder => "Song Title",
        ],
        ev(Ev::Change, |ev| {
            let el = ev.target()
                .unwrap()
                .unchecked_into::<web_sys::HtmlInputElement>();
            SuperMsg::SearchBar(Msg::UpdateTitle(el.value()))
        }),
    ]
}

fn artist_input() -> Node<SuperMsg> {
    input![
        id!["song-box"],
        attrs![
            At::Type => "text", 
            At::Name => "title", 
            At::Placeholder => "Artist Name",
        ],
        ev(Ev::Change, |ev| {
            let el = ev.target()
                .unwrap()
                .unchecked_into::<web_sys::HtmlInputElement>();
            SuperMsg::SearchBar(Msg::UpdateArtist(el.value()))
        }),
    ]
}

fn submit_button(model: &SuperModel) -> Node<SuperMsg> {
    button![
        C!["search-button"],
        attrs![At::Type => "submit"],
        if model.searching { "Searching..." } else { "Search!" },
    ]
}
