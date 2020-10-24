mod lyrics;
mod song;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub use lyrics::{Lyrics, LyricsDifficulty, LyricsStats};
pub use song::{Song, SongRequest, TestMode};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScoreRecord {
    name: String,
    genius_id: String,
    milliseconds: u32,
    absolute_time: String,
    mode: TestMode,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
