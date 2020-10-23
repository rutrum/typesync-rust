use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Song {
    pub title: String,
    pub artist: String,
    pub lyrics: Vec<String>,
    pub stats: SongStats,
    pub difficulty: SongDifficulty,
    pub img_url: String,
    pub genius_id: String,
}

#[derive(Clone, Copy, Default, Debug, Serialize, Deserialize)]
pub struct SongStats {
    pub total: usize,
    pub uppercase: usize,
    pub lowercase: usize,
    pub numeric: usize,
    pub whitespace: usize,
    pub punctuation: usize,
}

impl SongStats {
    pub fn from_lyrics(lyrics: String) -> SongStats {
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

        s
    }

    pub fn rate(&self) -> SongDifficulty {
        let score = self.lowercase
            + self.uppercase * 4
            + self.numeric * 4
            + self.whitespace
            + self.punctuation * 6;

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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SongRequest {
    pub title: String,
    pub artist: String,
}
