use reqwest::Url;
use serde_json::Value;
use typesync::{Song, SongRequest};

use select::document::Document;
use select::node::{Find, Node};
use select::predicate::Name;

// Specifically need to search for "Classes that start with 'x'"
use select::predicate::Predicate;
pub struct ClassBeginsWith<T>(pub T);

impl<'a> Predicate for ClassBeginsWith<&'a str> {
    fn matches(&self, node: &Node) -> bool {
        node.attr("class").map_or(false, |classes| {
            classes
                .split_whitespace()
                .any(|class| class.starts_with(self.0))
        })
    }
}

lazy_static! {
    static ref GENIUS_BEARER_AUTH: String = std::env::var("GENIUS_BEARER_AUTH").unwrap();
}

struct SongScrape {
    pub artist: String,
    pub title: String,
    pub img_url: String,
    pub lyrics_route: String,
    pub genius_id: String,
}

#[derive(Clone, Copy, Debug)]
pub enum GeniusError {
    ApiFetch,
    ApiScrape,
    WebFetch,
    WebScrape,
}

pub fn search_song_with_genius_id(genius_id: &str) -> Result<Song, GeniusError> {
    use GeniusError::*;

    let text = query_genius_id(&genius_id).map_err(|_| ApiFetch)?;
    let song_scrape = json_to_song(&text, false).ok_or(ApiScrape)?;
    let raw_html = query_genius_lyrics_page(&song_scrape.lyrics_route).map_err(|_| WebFetch)?;
    let raw_lyrics = scrape_for_lyrics(&raw_html).ok_or(WebScrape)?;

    Ok(Song::new(
        song_scrape.title,
        song_scrape.artist,
        raw_lyrics,
        song_scrape.img_url,
        song_scrape.genius_id,
    ))
}

pub fn search_song_on_genius(sr: &SongRequest) -> Result<Song, GeniusError> {
    use GeniusError::*;

    let text = query_genius_search(&sr.title, &sr.artist).map_err(|_| ApiFetch)?;
    let song_scrape = json_to_song(&text, true).ok_or(ApiScrape)?;
    let raw_html = query_genius_lyrics_page(&song_scrape.lyrics_route).map_err(|_| WebFetch)?;
    let raw_lyrics = scrape_for_lyrics(&raw_html).ok_or(WebScrape)?;

    Ok(Song::new(
        song_scrape.title,
        song_scrape.artist,
        raw_lyrics,
        song_scrape.img_url,
        song_scrape.genius_id,
    ))
}

fn scrape_for_lyrics(raw: &str) -> Option<String> {
    let doc: Document = Document::from_read(raw.as_bytes()).unwrap();
    let divs = doc
        .find(ClassBeginsWith("Lyrics__Container"))
        .into_selection();
    let lyrics = divs.iter().map(|div| get_text_from_node(&div)).collect();
    println!("{}", lyrics);
    //let lyrics = div.find(Name("p")).next()?.text();
    Some(lyrics)
}

/// Modified from https://docs.rs/select/0.5.0/src/select/node.rs.html#127-140
fn get_text_from_node(n: &Node) -> String {
    let mut string = String::new();
    recur(n, &mut string);
    return string;

    fn recur(node: &Node, string: &mut String) {
        if let Some(text) = node.as_text() {
            string.push_str(text);
        }
        if &node.html() == "<br>" {
            string.push_str("\n");
        }
        for child in node.children() {
            recur(&child, string)
        }
    }
}

fn query_genius_lyrics_page(route: &str) -> reqwest::Result<String> {
    let client = reqwest::blocking::Client::new();
    let url = Url::parse(&format!("https://genius.com{}", route)).unwrap();
    println!("{}", url);
    client
        .get(url)
        .bearer_auth(GENIUS_BEARER_AUTH.clone())
        .send()?
        .text()
}

fn json_to_song(text: &str, many: bool) -> Option<SongScrape> {
    let json: Value = serde_json::from_str(&text).unwrap();

    let metadata = if many {
        json.get("response")?.get("hits")?.get(0)?.get("result")?
    } else {
        json.get("response")?.get("song")?
    };

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

fn query_genius_id(id: &str) -> reqwest::Result<String> {
    let client = reqwest::blocking::Client::new();
    let url = Url::parse(&format!("https://api.genius.com/songs/{}", id)).unwrap();

    client
        .get(url)
        .bearer_auth(GENIUS_BEARER_AUTH.clone())
        .send()?
        .text()
}

fn query_genius_search(title: &str, artist: &str) -> reqwest::Result<String> {
    let client = reqwest::blocking::Client::new();
    let url = Url::parse_with_params(
        "https://api.genius.com/search",
        &[("q", format!("{} {}", title, artist))],
    )
    .unwrap();

    client
        .get(url)
        .bearer_auth(GENIUS_BEARER_AUTH.clone())
        .send()?
        .text()
}
