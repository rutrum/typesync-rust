use seed::{prelude::*, *};
use crate::Msg;

pub fn view() -> Node<Msg> {
    div![
        C!["title", "rainbow"],
        h2![ "TYPESYNC", ],
    ]
}
