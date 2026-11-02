use std::fs;

use clap::Parser;
use fern::colors::{Color, ColoredLevelConfig};
use id3::{Tag, TagLike, Version};
use iocore::Path;

use crate::{
    Config,
    Error,
    Exit,
    Result,
    UNKNOWN_ARTIST,
    UNKNOWN_TITLE,
    dispatch::ParserDispatcher,
    map_albums_and_artists,
    setup_logger,
};

#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about,
    long_about = "mp3-stamp - set ID3 tags in any number of mp3 files"
)]
pub struct StampCli {
    #[arg()]
    album_name: String,
    #[arg(short, long)]
    artist_name: Option<String>,
    #[arg()]
    mp3_files: Vec<Path>,
    #[arg(short, long, help = "path to toml file containing default artist name")]
    config_path: Option<Path>,
}
impl StampCli {
    pub fn mp3_files(&self) -> Vec<Path> {
        self.mp3_files
            .clone()
            .into_iter()
            .filter(|path| path.extension().unwrap_or_default() == ".mp3")
            .collect::<Vec<Path>>()
    }
    pub fn artist_name(&self) -> Result<String> {
        match &self.artist_name {
            Some(name) => Ok(name.to_string()),
            None => {
                let config = Config::load(self.config_path.clone())?;
                Ok(config.get_artist_name()?)
            }
        }
    }
}
impl ParserDispatcher<Error> for StampCli {
    fn dispatch(&mut self) -> Result<()> {
        let artist = self.artist_name()?;
        for path in self.mp3_files() {
            let mut tag = Tag::read_from_path(&path).unwrap_or_else(|e| Tag::new());
            tag.set_album(&self.album_name);
            tag.set_artist(&artist);
            tag.write_to_path(&path, Version::Id3v24)?;
        }
        Ok(())
    }
}
