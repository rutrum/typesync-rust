use seed::prelude::*;
use typesync::{SongPlays, Leaderboards, NewScore, Song, SongRequest};

const API_URL: &'static str = env!("API_URL");

pub async fn get_song_from_id(genius_id: &str) -> fetch::Result<Option<Song>> {
    fetch(format!("{}/lyrics/{}", API_URL, genius_id))
        .await?
        .json()
        .await
}

pub async fn post_score(score: NewScore) -> fetch::Result<Response> {
    fetch::Request::new(format!("{}/score", API_URL))
        .method(Method::Post)
        .json(&score)?
        .fetch()
        .await
}

pub async fn get_leaderboards(genius_id: &str) -> fetch::Result<Leaderboards> {
    fetch::Request::new(format!("{}/leaderboards/{}", API_URL, genius_id))
        .fetch()
        .await?
        .json()
        .await
}

pub async fn post_song_request(song_request: SongRequest) -> fetch::Result<Option<Song>> {
    fetch::Request::new(format!("{}/lyrics", API_URL))
        .method(Method::Post)
        .json(&song_request)?
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

pub async fn get_popular() -> fetch::Result<Vec<SongPlays>> {
    fetch::Request::new(format!("{}/popular", API_URL))
        .fetch()
        .await?
        .json()
        .await
}
