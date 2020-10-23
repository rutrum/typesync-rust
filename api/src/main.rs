#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_cors;

use typesync::{Song, SongRequest};

use rocket_contrib::json::Json;
use rocket_cors::{AllowedOrigins, Error};

#[post("/lyrics", data = "<request>")]
fn song_request(request: Json<SongRequest>) -> Json<Option<Song>> {
    let sr = request.into_inner();
    println!("Searching \"{}\" by {}", sr.title, sr.artist);
    let response = api::search_song_on_genius(sr);
    if let Ok(song) = &response {
        println!("Found \"{}\" by {}", song.title, song.artist);
    }
    Json(response.ok())
}

fn main() -> Result<(), Error> {
    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        //allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        //allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    rocket::ignite()
        .mount("/", routes![song_request])
        .attach(cors)
        .launch();

    Ok(())
}
