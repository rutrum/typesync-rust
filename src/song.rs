use serde::{Deserialize, Serialize};
use unicode_normalization::UnicodeNormalization;

use crate::*;

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct SongRequest {
    pub title: String,
    pub artist: String,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum TestMode {
    Simple,
    Standard,
}

impl TestMode {
    pub fn as_i8(&self) -> i8 {
        match self {
            TestMode::Standard => 0,
            TestMode::Simple => 1,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Tests {
    pub simple: Lyrics,
    pub standard: Lyrics,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Song {
    pub title: String,
    pub artist: String,
    pub img_url: String,
    pub genius_id: String,
    pub tests: Tests,
}

impl Song {
    pub fn new(
        title: String,
        artist: String,
        raw_lyrics: String,
        img_url: String,
        genius_id: String,
    ) -> Self {
        let cleaned = clean(raw_lyrics);

        let tests = Tests {
            simple: to_simple(cleaned.clone()),
            standard: to_standard(cleaned),
        };

        Self {
            title,
            artist,
            tests,
            img_url,
            genius_id,
        }
    }

    pub fn lyrics(&self, mode: TestMode) -> &Lyrics {
        use TestMode::*;
        match mode {
            Simple => &self.tests.simple,
            Standard => &self.tests.standard,
        }
    }
}

fn to_simple(cleaned: Vec<String>) -> Lyrics {
    let simple = cleaned
        .iter()
        .map(|x| {
            x.to_lowercase()
                .chars()
                .filter(|&x| !x.is_ascii_punctuation())
                .collect()
        })
        .collect();
    Lyrics::new(simple)
}

fn to_standard(cleaned: Vec<String>) -> Lyrics {
    Lyrics::new(cleaned)
}

fn clean(raw: String) -> Vec<String> {
    let ascii = raw.nfkd().filter(|x| x.is_ascii()).collect::<String>();

    ascii
        .split('\n')
        .filter(|x| !x.is_empty())
        .filter(|x| !(x.starts_with('[') && x.ends_with(']')))
        .map(|x| x.to_string())
        .collect()
}
