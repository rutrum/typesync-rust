#[derive(Clone, Copy, Default, Debug)]
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

        s
    }
}
