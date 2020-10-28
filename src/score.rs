use crate::TestMode;
use std::time::Duration;
use chrono::{TimeZone, DateTime, Utc};

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

#[cfg_attr(feature = "database", derive(Insertable, Queryable), table_name = "scores")]
pub struct NewScoreRecordDb {
    pub name: String,
    pub genius_id: String,
    pub milliseconds: i64,
    pub absolute_time: i64,
    pub mode: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScoreRecord {
    pub name: String,
    pub genius_id: String,
    pub milliseconds: i64,
    pub absolute_time: String,
    pub mode: TestMode,
}

impl From<NewScoreRecordDb> for ScoreRecord {
    fn from(db_record: NewScoreRecordDb) -> Self {
        let mode = match db_record.mode.as_ref() {
            "Simple" => TestMode::Simple,
            _ => TestMode::Standard,
        };

        ScoreRecord {
            name: db_record.name,
            genius_id: db_record.genius_id,
            milliseconds: db_record.milliseconds,
            absolute_time: seconds_since_unix_to_string(db_record.absolute_time),
            mode: mode,
        }
    }
}

fn seconds_since_unix_to_string(secs: i64) -> String {
    use chrono::Duration;

    let unix = Utc.ymd(1970, 1, 1).and_hms(0, 0, 0);
    let datetime = unix + Duration::seconds(secs);
    format!("{}", datetime.format("%B %e, %Y"))
}
