use seed::{prelude::*, *};

mod title;
mod search_bar;

#[derive(Clone, Debug)]
enum Page {
    Home,
}

#[derive(Clone, Debug)]
pub struct Model {
    page: Page,
    color: String,
    searching: bool,
    search_bar: search_bar::Model,
}

fn init(_url: Url, _orders: &mut impl Orders<Msg>) -> Model {
    Model {
        page: Page::Home,
        color: "red".to_string(),
        search_bar: search_bar::Model::default(),
        searching: false,
    }
}

pub enum Msg {
    SearchBar(search_bar::Msg),
    SearchSong,
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SearchBar(msg) => {
            search_bar::update(msg, &mut model.search_bar);
        },
        Msg::SearchSong => {
            model.searching = true;
            log!("Search for song: ", model.search_bar);
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        title::view(),
        search_bar::view(model),
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::start("app", init, update, view);
}
