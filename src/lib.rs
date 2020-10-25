mod lyrics;
mod song;
use serde::{Deserialize, Serialize};

pub use lyrics::{Lyrics, LyricsDifficulty, LyricsStats};
pub use song::{Song, SongRequest, TestMode};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScoreRecord {
    pub name: String,
    pub genius_id: String,
    pub milliseconds: u128,
    pub absolute_time: i64,
    pub mode: TestMode,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
