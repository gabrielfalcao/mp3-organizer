use std::fs;

use clap::Parser;
use fern::colors::{Color, ColoredLevelConfig};
use iocore::Path;
use mp3_organizer::{
    Config,
    Error,
    Exit,
    Result,
    UNKNOWN_ARTIST,
    UNKNOWN_TITLE,
    dispatch::ParserDispatcher,
    map_albums_and_artists,
};

#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about,
    long_about = "mp3-stamp - set ID3 tags in any number of mp3 files"
)]
pub struct Cli {
    #[arg()]
    album_name: String,
    #[arg(short, long)]
    artist_name: Option<String>,
    #[arg()]
    mp3_files: Vec<Path>,
    #[arg(short, long, help = "path to toml file containing default artist name")]
    config_path: Option<Path>,
}
impl Cli {
    pub fn mp3_files(&self) -> Vec<Path> {
        self.mp3_files
            .clone
            .into_iter()
            .filter(|path| path.extension() == ".mp3")
            .collect::<Vec<Path>>()
    }
    pub fn artist_name(&self) -> Result<String> {
        match &self.artist_name {
            Some(name) => Ok(name.to_string()),
            None => Ok(Config::load(self.config_path.clone()).get_artist_name()?),
        }
    }
}
impl ParserDispatcher<Error> for Cli {
    fn dispatch(&mut self) -> Result<()> {
        setup_logger()?;
        let artist = self.artist_name()?;
        for path in self.mp3_files() {
            let mut tag = Tag::read_from_path(path).unwrap_or_else(|e| Tag::new());
            tag.set_album(&self.album_name);
            tag.set_artist(&artist);
            tag.write_to_path(&path, Version::Id3v24)?;
        }
        Ok(())
    }
}
fn setup_logger() -> Result<()> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            let datetime = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S");
            let colors = ColoredLevelConfig::new()
                .debug(Color::Green)
                .info(Color::Blue)
                .warn(Color::TrueColor { r: 0xE0u8, g: 0x68u8, b: 0x26u8 });

            out.finish(format_args!(
                "[{}] {} {} {}",
                colors.color(record.level()),
                record.target(),
                datetime,
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stderr())
        // .chain(fern::log_file("mp3-organizer.log"))
        .apply()?;
    Ok(())
}

fn main() -> Exit {
    Cli::main()
}
