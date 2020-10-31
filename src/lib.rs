#[macro_use]
#[cfg(feature = "database")]
extern crate diesel;
use diesel::deserialize::{QueryableByName, Result};
use diesel::backend::Backend;
use diesel::row::NamedRow;

use serde::{Serialize};

mod lyrics;
mod song;
mod score;

pub use lyrics::{Lyrics, LyricsDifficulty, LyricsStats};
pub use song::{Song, SongRequest, TestMode, Tests};
pub use score::{Leaderboards, DbScore, NewScore, Score};

pub type GeniusId = String;

use diesel::sql_types::{Text, BigInt};

#[cfg(feature = "database")]
#[derive(Debug, Clone, QueryableByName)]
pub struct GeniusIdPopularity {
    #[sql_type = "Text"]
    genius_id: GeniusId,
    #[sql_type = "BigInt"]
    popularity: i64,
}

impl GeniusIdPopularity {
    pub fn sql_query_from_scores() -> &'static str {
        "SELECT genius_id, COUNT(genius_id) as popularity \
             FROM scores \
             GROUP BY genius_id \
             ORDER BY popularity DESC \
             LIMIT 5"
    }
}

/*
#[cfg(feature = "database")]
impl<DB: Backend> QueryableByName<DB> for GeniusIdPopularity {
    fn build<R: NamedRow<DB>>(row: &R) -> Result<Self> {
        Ok(GeniusIdPopularity {
            genius_id: row.get("genius_id")?, 
            popularity: row.get("count")?,
        })
    }
}
*/

#[derive(Serialize, Clone, Copy, Debug)]
pub struct Plays {
    total: u32,
}

#[cfg(feature = "database")]
pub mod db;
