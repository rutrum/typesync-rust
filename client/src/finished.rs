//use chrono::Utc;
use seed::{prelude::*, *};
use std::time::Duration;
use typesync::{ScoreRecord, Song, TestMode};

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

            let score = ScoreRecord {
                name,
                genius_id: model.song.genius_id.clone(),
                milliseconds: model.time.as_millis() as i64,
                absolute_time: 0, //Utc::now().timestamp(),
                mode: model.mode,
            };

            orders.perform_cmd({
                async move {
                    match post_score(score).await {
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

/// Returns fetch that requests a song from the API.  Currently
/// tries to parse result as a SongRequest, it will fail.
async fn post_score(score: ScoreRecord) -> fetch::Result<Response> {
    fetch::Request::new("http://localhost:8000/score")
        .method(Method::Post)
        .json(&score)?
        .fetch()
        .await
}

pub fn view(model: &Model) -> Node<Msg> {
    let time = format!("{:.*}", 2, model.time.as_millis() as f32 / 1000.0);
    let wpm = format!("{:.*}", 1, model.wpm);

    div![
        h2!["Finished!"],
        p![format!("{} seconds", time)],
        p![format!("{} wmp", wpm)],
        input![
            attrs!(
                At::Type => "text",
                At::AutoComplete => "off",
                At::Placeholder => "Enter your name:",
            ),
            input_ev(Ev::Input, |s| Msg::UpdateName(s))
        ],
        button!["Submit", ev(Ev::Click, |_| Msg::Submit)],
        p![if model.failed {
            "Failed to submit. Try again."
        } else {
            ""
        }],
    ]
}
