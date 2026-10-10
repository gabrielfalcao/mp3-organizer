use std::time::SystemTime;
use std::fs;
use clap::Parser;
use fern::colors::{Color, ColoredLevelConfig};
use iocore::Path;
use log::{debug, error, info, trace, warn};
use mp3_organizer::{Error, Exit, Result, dispatch::ParserDispatcher, map_albums_and_artists};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = "mp3-organizer command-line")]
pub struct Cli {
    #[arg()]
    mp3_directory_path: Path,
}
impl Cli {
    pub fn source_path(&self) -> Path {
        self.mp3_directory_path.try_canonicalize()
    }
}
impl ParserDispatcher<Error> for Cli {
    fn dispatch(&mut self) -> Result<()> {
        setup_logger()?;

        let source_path = self.source_path();
        let map = map_albums_and_artists(&source_path)?;
        println!("{source_path}");
        for (path, (artist,album , title)) in &map {
            let target_dir = source_path.join(&artist).join(&album);
            let target_file = target_dir.join(format!("{title}.mp3"));
            if path == &target_file {
                log::info!("skipping already mapped path {path}");
                continue
            }
            target_dir.mkdir()?;
            fs::rename(&path, &target_file)?;
        }
        Ok(())
    }
}
fn setup_logger() -> Result<()> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            let datetime = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S");
            let colors = ColoredLevelConfig::new().debug(Color::Green ).info(Color::Blue).warn(Color::TrueColor {
                r: 0xE0u8,
                g: 0x68u8,
                b: 0x26u8,
            });

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
