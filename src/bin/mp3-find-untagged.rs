#![allow(unused)]
use std::fs;

use clap::Parser;
use iocore::Path;
use mp3_organizer::{
    Error,
    Exit,
    Result,
    dispatch::ParserDispatcher,
    is_untagged,
    scan_for_mp3_files,
};

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
        let source_path = self.source_path();

        if !source_path.is_dir() {
            return Err(Error::InputError(format!("target dir must be an existing directory")));
        }
        let mp3_files = scan_for_mp3_files(source_path)?;
        for path in mp3_files.iter() {
            if is_untagged(&path) {
                println!("UNTAGGED: {path}")
            }
            Ok(())
        }
    }
}

fn main() -> Exit {
    Cli::main()
}
