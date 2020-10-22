use seed::{prelude::*, *};

enum Page {
    Home,
}

struct Model {
    page: Page,
    color: String,
}

enum Msg {
    Click,
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Click => {
            model.color = if model.color == "red" {
                "blue"
            } else {
                "red"
            }.to_string()
        },
    }
}

fn view(model: &Model) -> impl IntoNodes<Msg> {
    div![
        format!("{}", model.color), 
        ev(Ev::Click, |_| Msg::Click), 
    ]
}

fn init(_url: Url, _orders: &mut impl Orders<Msg>) -> Model {
    Model {
        page: Page::Home,
        color: "red".to_string(),
    }
}

#[wasm_bindgen(start)]
pub fn render() {
    App::start("app", init, update, view);
}
