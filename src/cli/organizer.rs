use std::fs;

use clap::Parser;
use fern::colors::{Color, ColoredLevelConfig};
use iocore::Path;
use crate::{
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
#[command(author, version, about, long_about = "mp3-organizer command-line")]
pub struct OrganizerCli {
    #[arg()]
    mp3_directory_path: Path,
}
impl OrganizerCli {
    pub fn source_path(&self) -> Path {
        self.mp3_directory_path.try_canonicalize()
    }
}
impl ParserDispatcher<Error> for OrganizerCli {
    fn dispatch(&mut self) -> Result<()> {
        let source_path = self.source_path();
        let map = map_albums_and_artists(&source_path)?;
        println!("{source_path}");
        for (path, ((artist, _updated_artist), (album, _updated_album), (title, _updated_title))) in
            &map
        {
            let target_dir = source_path.join(&artist).join(&album);
            let target_file = target_dir.join(format!("{artist} - {title} - {album}.mp3"));
            if path == &target_file {
                log::info!("skipping already mapped path {path}");
                continue;
            }
            target_dir.mkdir()?;
            if artist != UNKNOWN_ARTIST && title != UNKNOWN_TITLE {
                fs::rename(&path, &target_file)?;
            }
        }
        Ok(())
    }
}
