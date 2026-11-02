#![allow(unused)]
use std::fs;

use clap::Parser;
use iocore::Path;
use mp3_organizer::{
    dispatch::ParserDispatcher,
};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = "mp3-dedup command-line")]
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
        eprintln!("WIP");
        Ok(())
    }
}

fn main() -> Exit {
    Cli::main()
}
