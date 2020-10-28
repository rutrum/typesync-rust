use crate::TestMode;
use std::time::Duration;

#[cfg(feature = "database")]
use crate::db::schema::scores;
#[cfg(feature = "database")]
use diesel::{Insertable, Queryable};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Leaderboards {
    pub simple: Vec<ScoreRecord>,
    pub standard: Vec<ScoreRecord>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewScoreRecord {
    pub name: String,
    pub genius_id: String,
    pub milliseconds: i64,
    pub mode: TestMode,
}

impl NewScoreRecord {
    pub fn into_db_with_time(self, time: Duration) -> NewScoreRecordDb {
        NewScoreRecordDb {
            name: self.name,
            genius_id: self.genius_id,
            milliseconds: self.milliseconds,
            absolute_time: time.as_secs() as i64,
            mode: format!("{:?}", self.mode),
        }
    }
}

#[cfg_attr(feature = "database", derive(Insertable), table_name = "scores")]
pub struct NewScoreRecordDb {
    pub name: String,
    pub genius_id: String,
    pub milliseconds: i64,
    pub absolute_time: i64,
    pub mode: String,
}

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
