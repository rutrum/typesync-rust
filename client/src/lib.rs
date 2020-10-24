use seed::{prelude::*, *};
use typesync::{Song, TestMode};

mod search_bar;
mod song_summary;
mod title;
mod typing_test;

#[derive(Clone, Debug)]
enum Page {
    Home,
    Discovery,
    Test(typing_test::Model),
    TestDone,
}

#[derive(Clone, Debug)]
pub struct Model {
    page: Page,
    color: String,
    song: Option<Song>,
    mode: TestMode,
    search_bar: search_bar::Model,
    song_summary: song_summary::Model,
}

fn init(_url: Url, _orders: &mut impl Orders<Msg>) -> Model {
    Model {
        page: Page::Home,
        color: "red".to_string(),
        song: None,
        mode: TestMode::Standard,
        search_bar: search_bar::init(),
        song_summary: song_summary::init(),
    }
}

pub enum Msg {
    GoHome,
    SearchBar(search_bar::Msg),
    FoundSong(Option<Song>),
    SongSummary(song_summary::Msg),
    StartTest(TestMode),
    TypingTest(typing_test::Msg),
    TestDone,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::GoHome => {
            model.page = Page::Home;
        }
        Msg::SearchBar(msg) => {
            search_bar::update(msg, &mut model.search_bar, orders);
        }
        Msg::FoundSong(maybe) => {
            model.search_bar.searching = false;
            model.song = maybe;
            model.page = Page::Discovery;
        }
        Msg::SongSummary(msg) => {
            song_summary::update(msg, &mut model.song_summary, orders);
        }
        Msg::StartTest(mode) => {
            model.page = Page::Test(typing_test::init(model));
            model.mode = mode;
        }
        Msg::TypingTest(msg) => {
            // event does nothing if page is not test
            if let Page::Test(typing_test_model) = &mut model.page {
                typing_test::update(msg, typing_test_model, orders);
            }
        }
        Msg::TestDone => {
            model.page = Page::TestDone;
        }
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
            Page::Home => div![search_bar::view(&model.search_bar).map_msg(Msg::SearchBar),],
            Page::Discovery => div![
                search_bar::view(&model.search_bar).map_msg(Msg::SearchBar),
                song_summary::view(&model.song).map_msg(Msg::SongSummary),
            ],
            Page::Test(typing_test_model) =>
                div![typing_test::view(&typing_test_model, &model).map_msg(Msg::TypingTest),],
            Page::TestDone => div!["You did it!"],
        }
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::start("app", init, update, view);
}
