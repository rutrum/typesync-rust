extern crate serde_json;
extern crate reqwest;

use typesync::{SongRequest, Song};
use reqwest::{Client, Url};
use serde_json::Value;

pub fn search_song_on_genius(sr: SongRequest) -> Result<Song, ()> {
    let text = query_genius_search(sr.title, sr.artist).unwrap();
    let id = find_genius_id(text).ok_or(())?;
    let text = fetch_genius_song(&id).unwrap();
    json_to_song(text, id).ok_or(())
}

fn find_genius_id(text: String) -> Option<String> {
    let id = serde_json::from_str::<Value>(&text)
        .unwrap()
        .get("response")?
        .get("hits")?
        .get(0)?
        .get("result")?
        .get("id")?
        .as_str()?
        .to_string();
    Some(id)
}

fn json_to_song(text: String, genius_id: String) -> Option<Song> {
    let json: Value = serde_json::from_str(&text).unwrap();

    let metadata = json.get("response")?
        .get("song")?;

    let img_url = metadata.get("header_image_url")?.as_str()?.to_string();
    let artist = String::new(); 
    let title = metadata.get("title")?.as_str()?.to_string();
    let lyrics = String::new(); 

    Some(Song::new(artist, title, lyrics, img_url, genius_id))
}

fn fetch_genius_song(id: &String) -> reqwest::Result<String> {
    let client = reqwest::blocking::Client::new();
    let url = Url::parse(&format!("https://api.genius.com/songs/{}", id)).unwrap();
    client.get(url)
        .bearer_auth("rk7Bf0CVL9lOWaEaxZnrOTIiAp2qXwMaKfJfWd3XPoLGGxAgJWz1zl1dwwgoCz17")
        .send()?
        .text()
}

fn query_genius_search(title: String, artist: String) -> reqwest::Result<String> {
    let client = reqwest::blocking::Client::new();
    let url = Url::parse_with_params(
        "https://api.genius.com/search",
        &[("q", format!("{} {}", title, artist))],
    ).unwrap();

    client.get(url)
        .bearer_auth("rk7Bf0CVL9lOWaEaxZnrOTIiAp2qXwMaKfJfWd3XPoLGGxAgJWz1zl1dwwgoCz17")
        .send()?
        .text()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
