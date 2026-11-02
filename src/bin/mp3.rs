#![allow(unused)]
use clap::{Parser, Subcommand};
use mp3_organizer::{
    Error,
    Exit,
    Result,
    cli::{OrganizerCli, StampCli},
    dispatch::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher},
};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = "mp3 tools command-line")]
pub struct Cli {
    #[command(subcommand)]
    command: TopLevelCommand,
}
impl Cli {
    pub fn command(&self) -> TopLevelCommand {
        self.command.clone()
    }
}

impl ParserDispatcher<Error> for Cli {
    fn dispatch(&mut self) -> Result<()> {
        self.command.dispatch()?;

        Ok(())
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum TopLevelCommand {
    /// Guesses the missing ID3 tags for artist, album and title from filename then moves properly tagged mp3 files into subfolders: <Artist>/<Album>/<Filename>
    Organize(OrganizerCli),
    /// Writes Artist name configured in ~/.config/mp3.toml and optionally an album name and track name
    Stamp(StampCli),
}
impl SubcommandDispatcher<Error> for TopLevelCommand {
    fn dispatch(&mut self) -> Result<()> {
        match self {
            TopLevelCommand::Organize(op) => op.dispatch()?,
            TopLevelCommand::Stamp(op) => op.dispatch()?,
        }
        Ok(())
    }
}

fn main() -> Exit {
    Cli::main()
}
