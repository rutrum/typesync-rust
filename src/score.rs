use crate::TestMode;
use std::time::Duration;
use chrono::{TimeZone, Utc};

#[cfg(feature = "database")]
use crate::db::schema::scores;
#[cfg(feature = "database")]
use diesel::{Insertable, Queryable};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Leaderboards {
    pub simple: Vec<Score>,
    pub standard: Vec<Score>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewScore {
    pub name: String,
    pub genius_id: String,
    pub milliseconds: i64,
    pub mode: TestMode,
}

impl NewScore {
    pub fn into_db_with_time(self, time: Duration) -> DbScore {
        DbScore {
            name: self.name,
            genius_id: self.genius_id,
            milliseconds: self.milliseconds,
            absolute_time: time.as_secs() as i64,
            mode: self.mode.as_i8(),
        }
    }
}

#[cfg_attr(feature = "database", derive(Insertable, Queryable), table_name = "scores")]
pub struct DbScore {
    pub name: String,
    pub genius_id: String,
    pub milliseconds: i64,
    pub absolute_time: i64,
    pub mode: i8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Score {
    pub name: String,
    pub genius_id: String,
    pub milliseconds: i64,
    pub date: String,
    //pub mode: TestMode,
}

impl From<DbScore> for Score {
    fn from(db_record: DbScore) -> Self {
        /*let mode = match db_record.mode.as_ref() {
            "Simple" => TestMode::Simple,
            _ => TestMode::Standard,
        };*/

        Score {
            name: db_record.name,
            genius_id: db_record.genius_id,
            milliseconds: db_record.milliseconds,
            date: seconds_since_unix_to_string(db_record.absolute_time),
            //mode: mode,
        }
    }
}

fn seconds_since_unix_to_string(secs: i64) -> String {
    use chrono::Duration;

    let unix = Utc.ymd(1970, 1, 1).and_hms(0, 0, 0);
    let datetime = unix + Duration::seconds(secs);
    format!("{}", datetime.format("%B %e, %Y"))
}
