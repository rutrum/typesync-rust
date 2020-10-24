use serde::{Deserialize, Serialize};

use crate::lyrics::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SongRequest {
    pub title: String,
    pub artist: String,
}

#[derive(Clone, Copy, Debug)]
pub enum TestMode {
    Simple,
    Standard,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Song {
    pub title: String,
    pub artist: String,
    pub img_url: String,
    pub genius_id: String,
    pub tests: Tests,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tests {
    pub simple: Lyrics,
    pub standard: Lyrics,
}

impl Song {
    pub fn new(
        title: String,
        artist: String,
        raw_lyrics: String,
        img_url: String,
        genius_id: String,
    ) -> Self {
        let lyrics: Vec<String> = raw_lyrics
            .split('\n')
            .filter(|x| !x.is_empty())
			.filter(|x| !(x.starts_with("[") && x.ends_with("]")))
            .map(|x| x.to_string())
            .collect(); 

        let tests = Tests {
            simple: Lyrics::new(lyrics.clone()),
            standard: Lyrics::new(lyrics),
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
