#![allow(unused)]
pub(crate) mod errors;
pub use errors::{Error, Exit, Result};
pub mod dispatch;
pub use dispatch::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher};

pub(crate) mod mp3_organizer;
pub use mp3_organizer::*;
