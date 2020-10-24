use seed::{prelude::*, *};

use crate::Model as SuperModel;
use crate::Msg as SuperMsg;

#[derive(Clone, Default, Debug)]
pub struct Model {
    name: String,
}

pub fn init() -> Model {
    Model {
        ..Default::default()
    }
}

pub enum Msg {
    Submit,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<SuperMsg>) {
    use Msg::*;
    match msg {
        Submit => {
            log!("done!");
            orders.send_msg(SuperMsg::ToDiscovery);
        }
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    div![
        h2!["Finished!"],
        button![ "I'm done!", ev(Ev::Click, |_| Msg::Submit)], 
    ]
}
