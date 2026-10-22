use clap::{Parser, Subcommand};
use tmptestdeleteme::{
    Error,
    Exit,
    Result,
    cli::{OrganizeCli, StampCli},
    dispatch::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher},
};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = "tmptestdeleteme command-line")]
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
    Organize(OrganizeOpt),
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
