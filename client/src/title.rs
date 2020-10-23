use crate::Msg;
use seed::{prelude::*, *};

pub fn view() -> Node<Msg> {
    div![C!["title", "rainbow"], h2!["TYPESYNC",],]
}
