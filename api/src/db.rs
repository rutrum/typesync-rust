use crate::DbPool;
use diesel::{prelude::*, MysqlConnection};
use std::time::SystemTime;
use typesync::db::schema;
use typesync::{DbScore, GeniusIdPopularity, Leaderboards, NewScore, Score, TestMode};

type Result<T> = std::result::Result<T, diesel::result::Error>;

pub fn create_connection() -> MysqlConnection {
    const DATABASE_URL: &'static str = env!("DATABASE_URL");

    MysqlConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Cannot connect to database at {}", DATABASE_URL))
}

pub fn insert_record(conn: DbPool, record: NewScore) -> Result<usize> {
    use schema::scores::dsl::*;
    let now = SystemTime::now();
    let how_long = now.duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let record_db = record.into_db_with_time(how_long);
    diesel::insert_into(scores)
        .values(record_db)
        .execute(&*conn)
}

pub fn select_records(conn: DbPool) -> Result<Vec<Score>> {
    use schema::scores::dsl::*;
    let db_scores: Vec<DbScore> = scores.load(&*conn)?;
    Ok(db_scores.into_iter().map(|x| x.into()).collect())
}

pub fn get_leaderboards(conn: DbPool, g_id: &str) -> Result<Leaderboards> {
    use schema::scores::dsl::*;
    let standard = scores
        .filter(mode.eq(TestMode::Standard.as_i8()))
        .filter(genius_id.eq(&g_id))
        .order(milliseconds)
        .limit(10)
        .load::<DbScore>(&*conn)?
        .into_iter()
        .map(|x| x.into())
        .collect();
    let simple = scores
        .filter(mode.eq(TestMode::Simple.as_i8()))
        .filter(genius_id.eq(&g_id))
        .order(milliseconds)
        .limit(10)
        .load::<DbScore>(&*conn)?
        .into_iter()
        .map(|x| x.into())
        .collect();
    Ok(Leaderboards { simple, standard })
}

pub fn popular_songs(conn: DbPool) -> Result<Vec<GeniusIdPopularity>> {
    diesel::sql_query(GeniusIdPopularity::sql_query_from_scores()).load(&*conn)
}
