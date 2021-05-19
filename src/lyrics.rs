use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Lyrics {
    pub lyrics: Vec<String>,
    pub stats: LyricsStats,
    pub score: usize,
    pub difficulty: LyricsDifficulty,
}

impl Lyrics {
    pub fn new(lyrics: Vec<String>) -> Lyrics {
        let stats = LyricsStats::from_lyrics(&lyrics);

        let score = stats.lowercase
            + stats.uppercase * 4
            + stats.numeric * 4
            + stats.whitespace
            + stats.punctuation * 6;

        Lyrics {
            lyrics,
            stats,
            score,
            difficulty: LyricsDifficulty::from_score(score),
        }
    }
}

#[derive(Clone, Copy, Default, Debug, Serialize, Deserialize)]
pub struct LyricsStats {
    pub total: usize,
    pub uppercase: usize,
    pub lowercase: usize,
    pub numeric: usize,
    pub whitespace: usize,
    pub punctuation: usize,
}

impl LyricsStats {
    pub fn from_lyrics(lyrics: &[String]) -> LyricsStats {
        let mut total = lyrics.iter().fold(0, |acc, s| acc + s.len() + 1);
        if total > 0 {
            total -= 1;
        }

        let mut stats = LyricsStats {
            total,
            ..LyricsStats::default()
        };

        for lyric in lyrics {
            for c in lyric.chars() {
                if c.is_ascii_digit() {
                    stats.numeric += 1;
                } else if c.is_ascii_whitespace() {
                    stats.whitespace += 1;
                } else if c.is_ascii_punctuation() {
                    stats.punctuation += 1;
                } else if c.is_lowercase() {
                    stats.lowercase += 1;
                } else if c.is_uppercase() {
                    stats.uppercase += 1;
                }
            }
            stats.whitespace += 1; // end of line
        }
        stats.whitespace -= if stats.whitespace > 0 { 1 } else { 0 }; // remove end of file

        stats
    }
}

/// Labeled after difficulties in Skyrim.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum LyricsDifficulty {
    Novice,
    Apprentice,
    Adept,
    Expert,
    Master,
}

impl LyricsDifficulty {
    fn from_score(score: usize) -> Self {
        use LyricsDifficulty::*;
        match score {
            x if x < 1300 => Novice,
            x if x < 1300 * 2 => Apprentice,
            x if x < 1300 * 3 => Adept,
            x if x < 1300 * 4 => Expert,
            _ => Master,
        }
    }
}

// Ultimately so I can use take
impl std::default::Default for LyricsDifficulty {
    fn default() -> Self {
        LyricsDifficulty::Novice
    }
}
