extern crate reqwest;
extern crate select;
extern crate serde_json;
#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::databases::diesel;

pub mod db;
pub mod genius;

use typesync::{Song, SongRequest, Tests, Leaderboards};

pub struct SongMetadata {
    pub title: String,
    pub artist: String,
    pub raw_lyrics: String,
    pub img_url: String,
    pub genius_id: String,
}

impl SongMetadata {
    pub fn into_song(self, leaderboards: Leaderboards) -> Song {
        Song::new(
            self.title,
            self.artist,
            self.raw_lyrics,
            self.img_url,
            self.genius_id,
            leaderboards,
        )
    }
}

#[database("typesync")]
pub struct DbPool(diesel::SqliteConnection);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
