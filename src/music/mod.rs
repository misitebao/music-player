use std::time::Duration;

use crate::file_ops::read_audio_file;
use crate::utils::split_path::split_path_to_name;

#[derive(PartialEq, Clone)]
pub struct Music {
    pub path: String,
    pub name: String,
    pub artist: String,
    pub title: String,
    pub album: String,
    pub play_position: Duration,
    pub total_duration: Duration,
}

impl Music {
    pub fn new(path: &str) -> Result<Music, String> {
        let name = split_path_to_name(&path)
            .split('.')
            .next()
            .unwrap()
            .to_string();
        match read_audio_file(path) {
            Ok(audio) => Ok(Music {
                path: path.to_string(),
                name,
                artist: audio.artist,
                title: audio.title,
                album: audio.album,
                play_position: Duration::from_secs(0),
                total_duration: audio.duration,
            }),
            Err(err) => Err(err.to_string()),
        }
    }
}
