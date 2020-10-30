use seed::{prelude::*, *};

use crate::Msg as SuperMsg;

pub fn view() -> Node<SuperMsg> {
    div![
        C!["title", "rainbow"],
        h2!["TYPESYNC"],
        ev(Ev::Click, |_| SuperMsg::ToHomeScreen),
    ]
}
