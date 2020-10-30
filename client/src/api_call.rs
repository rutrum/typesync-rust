use seed::prelude::*;
use typesync::{Leaderboards, NewScore, Song, SongRequest};

pub async fn get_song_from_id(genius_id: &str) -> fetch::Result<Option<Song>> {
    fetch(format!("http://localhost:8000/lyrics/{}", genius_id))
        .await?
        .json()
        .await
}

pub async fn post_score(score: NewScore) -> fetch::Result<Response> {
    fetch::Request::new("http://localhost:8000/score")
        .method(Method::Post)
        .json(&score)?
        .fetch()
        .await
}

pub async fn get_leaderboards(genius_id: &str) -> fetch::Result<Leaderboards> {
    fetch::Request::new(format!("http://localhost:8000/leaderboards/{}", genius_id))
        .fetch()
        .await?
        .json()
        .await
}

pub async fn post_song_request(song_request: SongRequest) -> fetch::Result<Option<Song>> {
    fetch::Request::new("http://localhost:8000/lyrics")
        .method(Method::Post)
        .json(&song_request)?
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}
