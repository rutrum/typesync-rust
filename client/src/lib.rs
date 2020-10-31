use seed::{prelude::*, *};
use std::time::Duration;
use typesync::{Song, TestMode};
use typesync::{SongPlays};

mod api_call;
mod finished;
mod search_bar;
mod song_summary;
mod title;
mod typing_test;
mod popular;

#[derive(Clone, Debug)]
pub enum Page {
    Home,
    Summary(song_summary::Model),
    Test(typing_test::Model),
    Finish(finished::Model),
}

#[derive(Clone, Debug)]
pub struct Model {
    page: Page,
    popular: Vec<SongPlays>,
    search_bar: search_bar::Model,
}

fn init(mut url: Url, orders: &mut impl Orders<Msg>) -> Model {
    //orders.subscribe(|subs::UrlChanged(url)| log!(url));

    let page = match url.remaining_path_parts().as_slice() {
        [] => Page::Home,
        ["song", id, rest @ ..] => {
            let gid: String = id.to_owned().to_string();
            match rest {
                ["standard"] => {
                    orders.perform_cmd({
                        async move {
                            Msg::MaybeStartTest(
                                api_call::get_song_from_id(&gid).await.ok().flatten(),
                                TestMode::Standard,
                            )
                        }
                    });
                }
                ["simple"] => {
                    orders.perform_cmd({
                        async move {
                            Msg::MaybeStartTest(
                                api_call::get_song_from_id(&gid).await.ok().flatten(),
                                TestMode::Simple,
                            )
                        }
                    });
                }
                _ => {
                    orders.perform_cmd({
                        async move {
                            Msg::FoundSong(api_call::get_song_from_id(&gid).await.ok().flatten())
                        }
                    });
                }
            }
            Page::Home
        }
        _ => {
            Url::new().go_and_replace();
            Page::Home
        }
    };

    orders.perform_cmd({
        async {
            Msg::FetchedPopular(api_call::get_popular().await.unwrap_or(Vec::new()))
        }
    });

    Model {
        page,
        popular: vec![],
        search_bar: search_bar::init(),
    }
}

pub enum Msg {
    ChangePage(Page),
    SearchBar(search_bar::Msg),
    FoundSong(Option<Song>),
    Summary(song_summary::Msg),
    MaybeStartTest(Option<Song>, TestMode),
    StartTest(Song, TestMode),
    TypingTest(typing_test::Msg),
    TestDone(Song, TestMode, Duration, f32),
    Finished(finished::Msg),
    SubmitScores(Song),
    ToHomeScreen,
    FetchedPopular(Vec<SongPlays>),
}

/// Todo: rewrite all these stuff so there really aren't options in
/// global state.  If it's optional, just pass it between models through
/// the page changing Msg variants
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SearchBar(msg) => {
            search_bar::update(msg, &mut model.search_bar, orders);
        }
        Msg::FoundSong(maybe_song) => {
            model.search_bar.searching = false;
            if let Some(song) = maybe_song.as_ref() {
                let genius_id = song.genius_id.clone();

                Url::new().set_path(&["song", &genius_id]).go_and_push();

                orders.perform_cmd({
                    async move {
                        let l = api_call::get_leaderboards(&genius_id)
                            .await
                            .unwrap_or_default();
                        Msg::Summary(song_summary::Msg::UpdateLeaderboards(l))
                    }
                });
            } else {
                Url::new().go_and_replace();
            }
            model.page = Page::Summary(song_summary::init(maybe_song));
        }
        Msg::Summary(msg) => {
            if let Page::Summary(summary_model) = &mut model.page {
                song_summary::update(msg, summary_model, orders);
            }
        }
        Msg::MaybeStartTest(maybesong, mode) => {
            if let Some(song) = maybesong {
                model.page = Page::Test(typing_test::init(song, mode));
            } else {
                //Url::new().go_and_replace();
            }
        }
        Msg::StartTest(song, mode) => {
            Url::current()
                .add_path_part(mode.to_lowercase())
                .go_and_push();
            model.page = Page::Test(typing_test::init(song, mode));
        }
        Msg::TypingTest(msg) => {
            if let Page::Test(typing_test_model) = &mut model.page {
                typing_test::update(msg, typing_test_model, orders);
            }
        }
        Msg::TestDone(song, mode, time, wpm) => {
            model.page = Page::Finish(finished::init(time, wpm, song, mode));
        }
        Msg::Finished(msg) => {
            if let Page::Finish(finished_model) = &mut model.page {
                finished::update(msg, finished_model, orders);
            }
        }
        Msg::SubmitScores(song) => {
            let genius_id = song.genius_id.clone();
            orders.perform_cmd({
                async move {
                    let l = api_call::get_leaderboards(&genius_id)
                        .await
                        .unwrap_or_default();
                    Msg::Summary(song_summary::Msg::UpdateLeaderboards(l))
                }
            });
            Url::new()
                .set_path(&["song", &song.genius_id])
                .go_and_push();
            model.page = Page::Summary(song_summary::init(Some(song)));
        }
        Msg::ChangePage(page) => model.page = page,
        Msg::ToHomeScreen => {
            Url::new().go_and_push();
            model.page = Page::Home;
        }
        Msg::FetchedPopular(popular) => model.popular = popular,
    }
}

// Idea: When matching on the page to determine
// which elements to view, I might screw up and render a model
// that should have been "refreshed", or not have the previous data.
// Can I force a move after a function is called?  I should probably
// just reassign the model in the update function whenever necessary.

fn view(model: &Model) -> Node<Msg> {
    div![
        title::view(),
        match &model.page {
            Page::Home => div![
                search_bar::view(&model.search_bar).map_msg(Msg::SearchBar),
                popular::view(&model),
            ],
            Page::Summary(summary_model) => div![
                search_bar::view(&model.search_bar).map_msg(Msg::SearchBar),
                song_summary::view(summary_model).map_msg(Msg::Summary),
            ],
            Page::Test(typing_test_model) =>
                div![typing_test::view(&typing_test_model).map_msg(Msg::TypingTest),],
            Page::Finish(finished_model) =>
                div![finished::view(&finished_model).map_msg(Msg::Finished)],
        }
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::start("app", init, update, view);
}
