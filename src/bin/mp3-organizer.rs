use std::time::SystemTime;

use clap::Parser;
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

        Ok(())
    }
}
fn setup_logger() -> Result<()> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                record.level(),
                record.target(),
                chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S"),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

fn main() -> Exit {
    Cli::main()
}
