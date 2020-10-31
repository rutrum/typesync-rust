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
use typesync::{GeniusId, Leaderboards, NewScore, SongPlays, Song, SongRequest};

use api::db;
use api::genius;
use api::DbPool;

use std::sync::Mutex;

type SongCache = Mutex<LruCache<GeniusId, Song>>;
type GeniusIdCache = Mutex<LruCache<SongRequest, GeniusId>>;

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
fn song_by_id(state: State<SongCache>, genius_id: String) -> Json<Option<Song>> {
    println!("Searching for song with id {}", genius_id);
    Json(get_song_by_id(&state, &genius_id))
}

#[post("/lyrics", data = "<request>")]
fn song_request(
    song_state: State<SongCache>,
    id_state: State<GeniusIdCache>,
    request: Json<SongRequest>,
) -> Json<Option<Song>> {
    let sr = request.into_inner();
    println!("Searching \"{}\" by {}", sr.title, sr.artist);

    let mut id_cache = id_state.lock().unwrap();

    match id_cache.peek(&sr) {
        None => {
            let song = genius::search_song_on_genius(&sr);
            match song {
                Err(_) => Json(None),
                Ok(song) => {
                    let mut song_cache = song_state.lock().unwrap();
                    id_cache.put(sr.clone(), song.genius_id.clone());
                    song_cache.put(song.genius_id.clone(), song.clone());

                    println!("Found \"{}\" by {}", song.title, song.artist);
                    Json(Some(song))
                }
            }
        }
        /*
        Some(genius_id) => match song_cache.peek(genius_id) {
            None => {
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
            Some(song) => {
                println!("Found cached \"{}\" by {}", song.title, song.artist);
                Json(Some(song.clone()))
            }
        },
        */
        Some(genius_id) => Json(get_song_by_id(&song_state, &genius_id))
    }
}

fn get_song_by_id(state: &State<SongCache>, genius_id: &GeniusId) -> Option<Song> {
    let mut cache = state.lock().unwrap();

    match cache.peek(genius_id) {
        None => {
            let song = genius::search_song_with_genius_id(&genius_id);
            println!("{:?}", song);
            match song {
                Err(_) => None,
                Ok(song) => {
                    cache.put(song.genius_id.clone(), song.clone());

                    println!("Found \"{}\" by {}", song.title, song.artist);
                    Some(song)
                }
            }
        }
        Some(song) => {
            println!("Found cached \"{}\" by {}", song.title, song.artist);
            Some(song.clone())
        }
    }
}

#[get("/leaderboards/<genius_id>")]
fn fetch_leaderboards(conn: DbPool, genius_id: String) -> Json<Option<Leaderboards>> {
    Json(db::get_leaderboards(conn, &genius_id).ok())
}

#[get("/popular")]
fn popular_songs(state: State<SongCache>, conn: DbPool) -> Json<Vec<SongPlays>> {
    let populars = db::popular_songs(conn).unwrap_or_default();

    let songs = populars.iter().filter_map(|popular| {
        get_song_by_id(&state, &popular.genius_id).map({
            |song| SongPlays {
                song,
                plays: popular.plays,
            }
        })
    }).collect();

    Json(songs)
}

fn main() -> Result<(), Error> {
    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        ..Default::default()
    }
    .to_cors()?;

    let song_cache: SongCache = Mutex::new(LruCache::new(100));
    let id_cache: GeniusIdCache = Mutex::new(LruCache::new(100));

    rocket::ignite()
        .mount(
            "/",
            routes![
                popular_songs,
                song_request,
                song_by_id,
                save_score,
                fetch_leaderboards
            ],
        )
        .manage(song_cache)
        .manage(id_cache)
        .attach(cors)
        .attach(DbPool::fairing())
        .launch();

    Ok(())
}
