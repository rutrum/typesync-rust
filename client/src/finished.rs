use seed::{prelude::*, *};
use std::time::Duration;

use crate::Model as SuperModel;
use crate::Msg as SuperMsg;

#[derive(Clone, Default, Debug)]
pub struct Model {
    name: String,
    time: Duration,
    wpm: f32,
}

pub fn init(time: Duration, wpm: f32) -> Model {
    Model {
        time,
        wpm,
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
            orders.send_msg(SuperMsg::ToDiscovery);
        }
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    let time = format!("{:.*}", 2, model.time.as_millis() as f32 / 1000.0);
    let wpm = format!("{:.*}", 1, model.wpm);

    div![
        h2!["Finished!"],
        p![format!("{} seconds", time)],
        p![format!("{} wmp", wpm)],
        button!["I'm done!", ev(Ev::Click, |_| Msg::Submit)],
    ]
}
