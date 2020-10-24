use crate::Msg as SuperMsg;
use seed::{prelude::*, *};

pub fn view() -> Node<SuperMsg> {
    div![
        C!["title", "rainbow"], 
        h2!["TYPESYNC"],
        ev(Ev::Click, |_| SuperMsg::GoHome),
    ]
}
