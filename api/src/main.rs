#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate rocket_cors;

use rocket::http::Status;
use rocket_contrib::databases::diesel;
use rocket_contrib::json::Json;
use rocket_cors::{AllowedOrigins, Error};
use typesync::{Leaderboards, NewScoreRecord, ScoreRecord, Song, SongRequest};

use api::db;
use api::genius;
use api::DbPool;

#[post("/score", data = "<record>")]
fn save_score(conn: DbPool, record: Json<NewScoreRecord>) -> Status {
    let record = record.into_inner();
    println!("Saving score: {:?}", record);
    match db::insert_record(conn, record) {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}

#[post("/lyrics", data = "<request>")]
fn song_request(conn: DbPool, request: Json<SongRequest>) -> Json<Option<Song>> {
    let sr = request.into_inner();
    println!("Searching \"{}\" by {}", sr.title, sr.artist);
    let song = genius::search_song_on_genius(sr);

    match song {
        Err(_) => Json(None),
        Ok(song) => {
            println!("Found \"{}\" by {}", song.title, song.artist);
            Json(Some(song))
        }
    }
}

#[get("/leaderboards/<genius_id>")]
fn fetch_leaderboards(conn: DbPool, genius_id: String) -> Json<Option<Leaderboards>> {
    Json(db::get_leaderboards(conn, &genius_id).ok())
}

fn main() -> Result<(), Error> {
    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        ..Default::default()
    }
    .to_cors()?;

    rocket::ignite()
        .mount("/", routes![song_request, save_score, fetch_leaderboards])
        .attach(cors)
        .attach(DbPool::fairing())
        .launch();

    Ok(())
}
