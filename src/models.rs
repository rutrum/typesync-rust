use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Song {
    pub title: String,
    pub artist: String,
    pub lyrics: Vec<String>,
    pub stats: SongStats,
    pub img_url: String,
    pub genius_id: String,
}

impl Song {
    pub fn new(title: String, artist: String, raw_lyrics: String, img_url: String, genius_id: String) -> Self {

        let stats = SongStats::from_lyrics(&raw_lyrics);
        let lyrics = vec![raw_lyrics];

        Self {
            title,
            artist,
            lyrics,
            stats,
            img_url,
            genius_id,
        }
    }
}

#[derive(Clone, Copy, Default, Debug, Serialize, Deserialize)]
pub struct SongStats {
    pub total: usize,
    pub uppercase: usize,
    pub lowercase: usize,
    pub numeric: usize,
    pub whitespace: usize,
    pub punctuation: usize,
    pub difficulty: SongDifficulty,
}

impl SongStats {
    pub fn from_lyrics(lyrics: &String) -> SongStats {
        let mut s = SongStats {
            total: lyrics.len(),
            ..SongStats::default()
        };

        for c in lyrics.chars() {
            if c.is_ascii_digit() {
                s.numeric += 1;
            } else if c.is_ascii_whitespace() {
                s.whitespace += 1;
            } else if c.is_ascii_punctuation() {
                s.punctuation += 1;
            } else if c.is_lowercase() {
                s.lowercase += 1;
            } else if c.is_uppercase() {
                s.uppercase += 1;
            }
        }
        
        // May need some +1/-1 depending on line endings
        
        s.difficulty = Self::rate(s);

        s
    }

    fn rate(s: Self) -> SongDifficulty {
        let score = s.lowercase
            + s.uppercase * 4
            + s.numeric * 4
            + s.whitespace
            + s.punctuation * 6;

        use SongDifficulty::*;
        match score {
            x if x < 1300 * 1 => Novice,
            x if x < 1300 * 2 => Apprentice,
            x if x < 1300 * 3 => Adept,
            x if x < 1300 * 4 => Expert,
            _ => Master,
        }
    }
}

/// Labeled after difficulties in Skyrim.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum SongDifficulty {
    Novice,
    Apprentice,
    Adept,
    Expert,
    Master,
}

impl std::default::Default for SongDifficulty {
    fn default() -> Self {
        SongDifficulty::Novice
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SongRequest {
    pub title: String,
    pub artist: String,
}
