use seed::{prelude::*, *};
use std::time::Duration;
use typesync::{Leaderboards, Song, TestMode};

mod finished;
mod search_bar;
mod song_summary;
mod title;
mod typing_test;

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
    search_bar: search_bar::Model,
}

fn init(_url: Url, _orders: &mut impl Orders<Msg>) -> Model {
    Model {
        page: Page::Home,
        search_bar: search_bar::init(),
    }
}

pub enum Msg {
    ChangePage(Page),
    SearchBar(search_bar::Msg),
    FoundSong(Option<Song>),
    Summary(song_summary::Msg),
    StartTest(Song, TestMode),
    TypingTest(typing_test::Msg),
    TestDone(Song, TestMode, Duration, f32),
    Finished(finished::Msg),
    SubmitScores(Song),
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
                log!("Found ", genius_id);
                orders.perform_cmd({
                    async move {
                        let l = get_leaderboards(&genius_id).await.unwrap_or_default();
                        Msg::Summary(song_summary::Msg::UpdateLeaderboards(l))
                    }
                });
            }
            model.page = Page::Summary(song_summary::init(maybe_song));
        }
        Msg::Summary(msg) => {
            if let Page::Summary(summary_model) = &mut model.page {
                song_summary::update(msg, summary_model, orders);
            }
        }
        Msg::StartTest(song, mode) => {
            model.page = Page::Test(typing_test::init(song, mode));
        }
        Msg::TypingTest(msg) => {
            // event does nothing if page is not test
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
                    let l = get_leaderboards(&genius_id).await.unwrap_or_default();
                    Msg::Summary(song_summary::Msg::UpdateLeaderboards(l))
                }
            });
            model.page = Page::Summary(song_summary::init(Some(song)));
        }
        Msg::ChangePage(page) => model.page = page,
    }
}

async fn get_leaderboards(genius_id: &String) -> fetch::Result<Leaderboards> {
    fetch::Request::new(format!("http://localhost:8000/leaderboards/{}", genius_id))
        .fetch()
        .await?
        .json()
        .await
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
            Page::Home => div![search_bar::view(&model.search_bar).map_msg(Msg::SearchBar),],
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
