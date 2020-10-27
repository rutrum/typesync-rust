use seed::{prelude::*, *};
use typesync::{SongRequest};

use crate::api_call;
use crate::Msg as SuperMsg;

#[derive(Clone, Default, Debug)]
pub struct Model {
    title: String,
    artist: String,
    pub searching: bool,
}

pub fn init() -> Model {
    Model {
        searching: false,
        ..Default::default()
    }
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

            // don't skip to allow search to refresh
            orders.perform_cmd({
                async move { SuperMsg::FoundSong(
                    api_call::post_song_request(song_request).await.ok().flatten()
                )}
            });
        }
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    form![
        C!["search-box"],
        title_input(),
        artist_input(),
        submit_button(model),
        ev(Ev::Submit, |ev| {
            ev.prevent_default();
            Msg::SearchSong
        })
    ]
}

fn title_input() -> Node<Msg> {
    input![
        attrs![
            At::Type => "text",
            At::Name => "title",
            At::AutoFocus => AtValue::None, // causes warning
            At::Placeholder => "Song Title",
        ],
        ev(Ev::Change, |ev| {
            ev.prevent_default();
            let el = ev
                .target()
                .unwrap()
                .unchecked_into::<web_sys::HtmlInputElement>();
            Msg::UpdateTitle(el.value())
        }),
    ]
}

fn artist_input() -> Node<Msg> {
    input![
        attrs![
            At::Type => "text",
            At::Name => "title",
            At::Placeholder => "Artist Name",
        ],
        ev(Ev::Change, |ev| {
            ev.prevent_default();
            let el = ev
                .target()
                .unwrap()
                .unchecked_into::<web_sys::HtmlInputElement>();
            Msg::UpdateArtist(el.value())
        }),
    ]
}

fn submit_button(model: &Model) -> Node<Msg> {
    button![
        C!["search-button"],
        attrs![At::Type => "submit"],
        if model.searching {
            "Searching..."
        } else {
            "Search!"
        },
    ]
}
