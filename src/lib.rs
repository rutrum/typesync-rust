#[macro_use]
#[cfg(feature = "database")]
extern crate diesel;

use serde::{Deserialize,Serialize};

mod lyrics;
mod score;
mod song;

pub use lyrics::{Lyrics, LyricsDifficulty, LyricsStats};
pub use score::{DbScore, Leaderboards, NewScore, Score};
pub use song::{Song, SongRequest, TestMode, Tests};

pub type GeniusId = String;

#[cfg(feature = "database")]
use diesel::sql_types::{BigInt, Text};

#[cfg(feature = "database")]
#[derive(Debug, Clone, QueryableByName)]
pub struct GeniusIdPopularity {
    #[sql_type = "Text"]
    pub genius_id: GeniusId,
    #[sql_type = "BigInt"]
    pub plays: i64,
}

#[cfg(feature = "database")]
impl GeniusIdPopularity {
    pub fn sql_query_from_scores() -> &'static str {
        "SELECT genius_id, COUNT(genius_id) as plays \
             FROM scores \
             GROUP BY genius_id \
             ORDER BY plays DESC \
             LIMIT 5"
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SongPlays {
    pub song: Song,
    pub plays: i64,
}

#[cfg(feature = "database")]
pub mod db;
