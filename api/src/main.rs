#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate lru;
extern crate rocket_contrib;
extern crate rocket_cors;

use lru::LruCache;
use rocket::http::Status;
use rocket::State;
use rocket_contrib::json::Json;
use rocket_cors::{AllowedOrigins, Error};
use typesync::{Leaderboards, NewScore, Song, SongRequest};

use api::db;
use api::genius;
use api::DbPool;

use std::sync::Mutex;

type SongCache = Mutex<LruCache<SongRequest, Song>>;

#[post("/score", data = "<record>")]
fn save_score(conn: DbPool, record: Json<NewScore>) -> Status {
    let record = record.into_inner();
    println!("Saving score: {:?}", record);
    match db::insert_record(conn, record) {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}

#[get("/lyrics/<genius_id>")]
fn song_by_id(_state: State<SongCache>, genius_id: String) -> Json<Option<Song>> {
    println!("Searching for song with id {}", genius_id);

    //let mut cache = state.lock().unwrap();

    let song = genius::search_song_with_genius_id(&genius_id);
    println!("{:?}", song);
    match song {
        Err(_) => Json(None),
        Ok(song) => {
            println!("Found \"{}\" by {}", song.title, song.artist);
            Json(Some(song))
        }
    }
}

#[post("/lyrics", data = "<request>")]
fn song_request(state: State<SongCache>, request: Json<SongRequest>) -> Json<Option<Song>> {
    let sr = request.into_inner();
    println!("Searching \"{}\" by {}", sr.title, sr.artist);

    let mut cache = state.lock().unwrap();

    match cache.peek(&sr) {
        None => {
            let song = genius::search_song_on_genius(&sr);
            match song {
                Err(_) => Json(None),
                Ok(song) => {
                    cache.put(sr.clone(), song.clone());
                    println!("Found \"{}\" by {}", song.title, song.artist);
                    Json(Some(song))
                }
            }
        }
        Some(song) => {
            println!("Found cached \"{}\" by {}", song.title, song.artist);
            Json(Some(song.clone()))
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

    let cache: SongCache = Mutex::new(LruCache::new(100));

    rocket::ignite()
        .mount("/", routes![song_request, song_by_id, save_score, fetch_leaderboards])
        .manage(cache)
        .attach(cors)
        .attach(DbPool::fairing())
        .launch();

    Ok(())
}
