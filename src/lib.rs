#[macro_use]
#[cfg(feature = "database")]
extern crate diesel;

#[cfg(feature = "database")]
use diesel::{Insertable, Queryable};

use serde::{Deserialize, Serialize};

mod lyrics;
mod song;

pub use lyrics::{Lyrics, LyricsDifficulty, LyricsStats};
pub use song::{Leaderboards, Song, TestMode, Tests};

#[cfg(feature = "database")]
pub mod db;
#[cfg(feature = "database")]
use db::schema::scores;

#[cfg(feature = "database")]
pub struct TestModeString(String);

#[cfg(feature = "database")]
impl Into<TestMode> for TestModeString {
    fn into(self) -> TestMode {
        match self.0.as_ref() {
            "Simple" => TestMode::Simple,
            _ => TestMode::Standard,
        }
    }
}

#[cfg(feature = "database")]
impl<DB, ST> Queryable<ST, DB> for TestModeString
where
    DB: diesel::backend::Backend,
    String: Queryable<ST, DB>,
{
    type Row = <String as Queryable<ST, DB>>::Row;

    fn build(row: Self::Row) -> Self {
        TestModeString(String::build(row))
    }
}

// TODO make existing scorerecord have string as absolute time.
//  Store date in mysql database.  The serialization should be to a string.
//
// Read only struct shouldn't have absolute time at all, it will be calculated
// as needed.  But then what do I store...mysql has a date format, so I should
// use that.  Maybe this should get tackled after moving over to mysql.

#[cfg_attr(feature = "database", derive(Queryable))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScoreRecord {
    pub name: String,
    pub genius_id: String,
    pub milliseconds: i64,
    pub absolute_time: i64,
    #[cfg_attr(feature = "database", diesel(deserialize_as = "TestModeString"))]
    pub mode: TestMode,
}

#[cfg_attr(feature = "database", derive(Insertable), table_name = "scores")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewScoreRecord<'a> {
    pub name: &'a str,
    pub genius_id: &'a str,
    pub milliseconds: i64,
    pub absolute_time: i64,
    pub mode: &'a str,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct SongRequest {
    pub title: String,
    pub artist: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
