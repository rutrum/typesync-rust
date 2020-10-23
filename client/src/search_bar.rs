use typesync::models::SongRequest;
use seed::{prelude::*, *};

use crate::Msg as SuperMsg;
use crate::Model as SuperModel;

#[derive(Clone, Default, Debug)]
pub struct Model {
    pub title: String,
    pub artist: String,
    pub searching: bool,
}

pub enum Msg {
    UpdateTitle(String),
    UpdateArtist(String),
    SearchSong,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<SuperMsg>) {
    use Msg::*;
    match msg {
        UpdateTitle(s) => model.title = s,
        UpdateArtist(s) => model.artist = s,
        SearchSong => {
            model.searching = true;
            let song_request = SongRequest {
                title: model.title.clone(),
                artist: model.artist.clone(),
            };

            orders.skip().perform_cmd( {
                let sr = song_request.clone();
                async { SuperMsg::FoundSong(post_song_request(sr).await) }
            });
        }
    }
}

/// Returns fetch that requests a song from the API.  Currently
/// tries to parse result as a SongRequest, it will fail.
async fn post_song_request(song_request: SongRequest) -> fetch::Result<SongRequest> {
    fetch::Request::new("http://localhost:8080")
        .method(Method::Post)
        .json(&song_request)?
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

pub fn view(model: &SuperModel) -> Node<SuperMsg> {
    form![
        C!["search-box"],
        title_input(),
        artist_input(),
        submit_button(&model.search_bar),
        ev(Ev::Submit, |ev| {
            ev.prevent_default();
            SuperMsg::SearchBar(Msg::SearchSong)
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

fn submit_button(model: &Model) -> Node<SuperMsg> {
    button![
        C!["search-button"],
        attrs![At::Type => "submit"],
        if model.searching { "Searching..." } else { "Search!" },
    ]
}
