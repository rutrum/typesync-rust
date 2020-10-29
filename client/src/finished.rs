use seed::{prelude::*, *};
use std::time::Duration;
use typesync::{NewScore, Song, TestMode};

use crate::api_call;
use crate::Msg as SuperMsg;

#[derive(Clone, Debug)]
pub struct Model {
    song: Song,
    name: String,
    time: Duration,
    wpm: f32,
    mode: TestMode,
    failed: bool,
}

pub fn init(time: Duration, wpm: f32, song: Song, mode: TestMode) -> Model {
    Model {
        name: String::new(),
        time,
        wpm,
        song,
        mode,
        failed: false,
    }
}

pub enum Msg {
    UpdateName(String),
    Submit,
    SendSuccess,
    SendFailure,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<SuperMsg>) {
    use Msg::*;
    match msg {
        UpdateName(s) => model.name = s,
        Submit => {
            let name = model.name.clone();

            let score = NewScore {
                name,
                genius_id: model.song.genius_id.clone(),
                milliseconds: model.time.as_millis() as i64,
                mode: model.mode,
            };

            orders.perform_cmd({
                async move {
                    match api_call::post_score(score).await {
                        Ok(s) if s.status().is_ok() => SuperMsg::Finished(SendSuccess),
                        _ => SuperMsg::Finished(SendFailure),
                    }
                }
            });
        }
        SendSuccess => {
            orders.send_msg(SuperMsg::SubmitScores(std::mem::take(&mut model.song)));
        }
        SendFailure => model.failed = true,
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    let time = format!("{:.*}", 2, model.time.as_millis() as f32 / 1000.0);
    let wpm = format!("{:.*}", 1, model.wpm);

    form![
        C!["finished"],
        ev(Ev::Submit, |ev| {
            ev.prevent_default();
            Msg::Submit
        }),
        div![
            C!["results"],
            h2![C!["victory-message"], "Finished!"],
            div![
                C!["stats"],
                div![
                    div![C!["value"], wpm],
                    div!["wpm"],
                ],
                div![
                    div![C!["value"], time],
                    div!["time"],
                ],
            ],
        ],
        div![
            C!["name-submission"],
            input![
                attrs!(
                    At::Type => "text",
                    At::AutoComplete => "off",
                    At::Placeholder => "Enter your name:",
                ),
                input_ev(Ev::Input, Msg::UpdateName)
            ],
            button![
                "Submit", 
                attrs!(At::Type => "submit"),
            ],
        ],
        p![if model.failed {
            "Failed to submit. Try again."
        } else {
            ""
        }],
    ]
}
