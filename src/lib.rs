mod song;
mod lyrics;

pub use song::{Song, SongRequest, TestMode};
pub use lyrics::{Lyrics, LyricsDifficulty, LyricsStats};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
