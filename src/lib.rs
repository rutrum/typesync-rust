mod lyrics;
mod song;

pub use lyrics::{Lyrics, LyricsDifficulty, LyricsStats};
pub use song::{Song, SongRequest, TestMode};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
