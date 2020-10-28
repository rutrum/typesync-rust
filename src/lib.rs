#[macro_use]
#[cfg(feature = "database")]
extern crate diesel;

mod lyrics;
mod song;
mod score;

pub use lyrics::{Lyrics, LyricsDifficulty, LyricsStats};
pub use song::{Song, SongRequest, TestMode, Tests};
pub use score::{Leaderboards, DbScore, NewScore, Score};

#[cfg(feature = "database")]
pub mod db;
