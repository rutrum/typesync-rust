#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_cors;

use typesync::SongRequest;

use rocket_cors::{AllowedHeaders, AllowedOrigins, Error};
use rocket_contrib::json::Json;

#[post("/lyrics", data = "<request>")]
fn song_request(request: Json<SongRequest>) -> String {
    let sr = request.into_inner();
    println!("Requesting \"{}\" by {}", sr.title, sr.artist);
    let response = api::search_song_on_genius(sr);
    println!("{:?}", response);
    String::from("Request!")
}

fn main() -> Result<(), Error> {
    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }.to_cors()?;

    rocket::ignite()
        .mount("/", routes![song_request])
        .attach(cors)
        .launch();

    Ok(())
}
