extern crate reqwest;
extern crate select;
extern crate serde_json;

use reqwest::Url;
use serde_json::Value;
use typesync::{Song, SongRequest};

use select::document::Document;
use select::node::Node;
use select::predicate::{Class, Name};

struct SongScrape {
    artist: String,
    title: String,
    img_url: String,
    lyrics_route: String,
    genius_id: String,
}

#[derive(Clone, Copy, Debug)]
pub enum GeniusError {
    ApiFetch,
    ApiScrape,
    WebFetch,
    WebScrape,
}

pub fn search_song_on_genius(sr: SongRequest) -> Result<Song, GeniusError> {
    use GeniusError::*;
    
    let text = query_genius_search(sr.title, sr.artist).map_err(|_| ApiFetch)?;
    let song_scrape = json_to_song(text).ok_or(ApiScrape)?;
    let raw_html = query_genius_lyrics_page(&song_scrape.lyrics_route).map_err(|_| WebFetch)?;
    let raw_lyrics = scrape_for_lyrics(raw_html).ok_or(WebScrape)?;
    
    Ok(Song::new(
        song_scrape.title,
        song_scrape.artist,
        raw_lyrics,
        song_scrape.img_url,
        song_scrape.genius_id,
    ))
}

fn scrape_for_lyrics(raw: String) -> Option<String> {
    let doc: Document = Document::from_read(raw.as_bytes()).unwrap();
    let div: Node = doc.find(Class("lyrics")).next()?;
    let lyrics = div.find(Name("p")).next()?.text();
    Some(lyrics)
}

fn query_genius_lyrics_page(route: &str) -> reqwest::Result<String> {
    let client = reqwest::blocking::Client::new();
    let url = Url::parse(&format!("https://genius.com{}", route)).unwrap();
    println!("{:?}", url);
    client
        .get(url)
        .bearer_auth("rk7Bf0CVL9lOWaEaxZnrOTIiAp2qXwMaKfJfWd3XPoLGGxAgJWz1zl1dwwgoCz17")
        .send()?
        .text()
}

fn json_to_song(text: String) -> Option<SongScrape> {
    let json: Value = serde_json::from_str(&text).unwrap();

    let metadata = json.get("response")?.get("hits")?.get(0)?.get("result")?;

    let img_url = metadata.get("header_image_url")?.as_str()?.to_string();
    let title = metadata.get("title")?.as_str()?.to_string();
    let lyrics_route = metadata.get("path")?.as_str()?.to_string();
    let genius_id = metadata.get("id")?.as_u64()?.to_string();
    let artist = metadata
        .get("primary_artist")?
        .get("name")?
        .as_str()?
        .to_string();

    Some(SongScrape {
        artist,
        title,
        lyrics_route,
        img_url,
        genius_id,
    })
}

fn query_genius_search(title: String, artist: String) -> reqwest::Result<String> {
    let client = reqwest::blocking::Client::new();
    let url = Url::parse_with_params(
        "https://api.genius.com/search",
        &[("q", format!("{} {}", title, artist))],
    )
    .unwrap();

    client
        .get(url)
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
