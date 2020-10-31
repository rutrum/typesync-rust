use seed::{prelude::*, *};
use wasm_timer::Instant;

use crate::Msg as SuperMsg;
use typesync::{Song, TestMode};

#[derive(Clone, Debug)]
pub struct Model {
    buffer: String,        // stores the users keystrokes
    accurate: bool,        // is buffer align with current_lyric?
    finished_line: bool,   // did the user just complete a line? used for events
    time: Option<Instant>, // time the test starts
    total_chars: usize,
    song: Song,
    mode: TestMode,
    lyric_num: usize,
    typed_chars: usize,
}

impl Model {
    fn lyrics(&self) -> &Vec<String> {
        &self.song.lyrics(self.mode).lyrics
    }

    fn advance_lyric(&mut self) {
        self.lyric_num += 1;
        self.typed_chars += self.buffer.len() + 1;
        self.buffer.clear();
    }

    fn this_lyric(&self) -> Option<&String> {
        self.lyrics().get(self.lyric_num)
    }

    fn next_lyric(&self) -> Option<&String> {
        self.lyrics().get(self.lyric_num + 1)
    }

    fn update_buffer(&mut self, s: String) {
        self.buffer = s;
        self.accurate = match self.this_lyric() {
            None => false,
            Some(lyric) => lyric.starts_with(&self.buffer),
        }
    }

    fn on_last_lyric(&self) -> bool {
        self.lyric_num == self.lyrics().len() - 1
    }

    fn will_match_lyric(&self, key: &str) -> bool {
        let after = format!("{}{}", self.buffer, key);
        match self.this_lyric() {
            None => false,
            Some(lyric) => &after == lyric,
        }
    }

    fn does_match(&self) -> bool {
        match self.this_lyric() {
            None => false,
            Some(lyric) => &self.buffer == lyric,
        }
    }
}

pub fn init(song: Song, mode: TestMode) -> Model {
    Model {
        buffer: String::new(),
        accurate: true,
        finished_line: false,
        time: None,
        total_chars: song.lyrics(mode).stats.total,
        song,
        mode,
        lyric_num: 0,
        typed_chars: 0,
    }
}

pub enum Msg {
    KeyPress(String),
    InputChange(String),
    Tick,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<SuperMsg>) {
    use Msg::*;
    match msg {
        KeyPress(key) => {
            // Start timer if necessary
            if model.time.is_none() {
                model.time = Some(Instant::now());
                orders.send_msg(SuperMsg::TypingTest(Msg::Tick));
            }

            let line_complete = model.does_match() && (key == " " || key == "Enter");
            let final_line_complete = model.on_last_lyric() && model.will_match_lyric(&key);

            if line_complete || final_line_complete {
                model.finished_line = true;
                model.advance_lyric();
            }

            if final_line_complete {
                let final_time = model.time.unwrap().elapsed();
                let words = model.total_chars as f32 / 6.0;
                let minutes = final_time.as_secs_f32() / 60.0;
                orders.send_msg(SuperMsg::TestDone(
                    model.song.clone(),
                    model.mode,
                    final_time,
                    words / minutes,
                ));
            }
        }
        Tick => {
            // Call the next tick (this tick will cause a rerender)
            orders.perform_cmd(cmds::timeout(10, || SuperMsg::TypingTest(Msg::Tick)));
        }
        InputChange(s) => {
            // check/bool are necessary because key event fires first
            if model.finished_line {
                model.finished_line = false;
            } else {
                model.update_buffer(s);
            }
        }
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    let timer = match model.time {
        Some(time) => format!("{:.*}", 2, time.elapsed().as_millis() as f32 / 1000.0),
        None => "0.00".to_string(),
    };

    use chrono::DateTime;
    use chrono::NaiveDateTime;
    use chrono::Utc;
    let _dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(61, 0), Utc);
    let total_typed = model.typed_chars + model.buffer.len();
    let percentage = {
        let p = total_typed as f32 / model.total_chars as f32 * 100.0;
        if p < 100.0 {
            p
        } else {
            100.0
        }
    };

    div![
        C!["typing-test"],
        small_song_view(&model.song, model.mode, timer),
        input![
            attrs! {
                At::AutoComplete => "off",
                At::Type => "text",
                At::AutoFocus => AtValue::None,
                At::Placeholder => "Type to begin...",
                At::Value => model.buffer,
                At::SpellCheck => "false",
            },
            C![if model.accurate { "good" } else { "bad" }],
            keyboard_ev(Ev::KeyPress, |ev| Msg::KeyPress(ev.key())), // fires first
            input_ev(Ev::Input, Msg::InputChange),                   // fires second
            ev(Ev::Paste, |ev| ev.prevent_default()),
        ],
        progress_bar_view(percentage),
        div![
            C!["lyrics"],
            div![C!["top-line"], model.this_lyric()],
            div![C!["bot-line"], model.next_lyric()],
        ],
    ]
}

fn small_song_view(song: &Song, mode: TestMode, timer: String) -> Node<Msg> {
    div![
        C!["small-song"],
        img![attrs!(At::Alt => "album art", At::Src => song.img_url),],
        div![
            C!["details"],
            div![
                h1![C!["song-title"], &song.title],
                h2![C!["song-artist"], &song.artist],
            ],
            div![
                p![C!["mode"], format!("{:?}", mode)],
                p![
                    C!["difficulty"],
                    C![format!("{:?}", song.lyrics(mode).difficulty)],
                    format!("{:?}", &song.lyrics(mode).difficulty)
                ],
            ],
            div![C!["timer"], timer],
        ]
    ]
}

fn progress_bar_view(percentage: f32) -> Node<Msg> {
    div![
        C!["progress-bar"],
        div![
            C!["filler"],
            style! { "width" => format!("{}%", percentage) },
        ]
    ]
}
