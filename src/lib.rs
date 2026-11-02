#![allow(unused)]
pub(crate) mod errors;
pub use errors::{Error, Exit, Result};
pub mod dispatch;
pub use dispatch::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher};

#[doc(hidden)] pub mod mp3;
#[doc(inline)]
pub use mp3::{
    UNKNOWN_ALBUM,
    UNKNOWN_ARTIST,
    UNKNOWN_TITLE,
    cleanup_rename_mp3_filename,
    is_mp3,
    is_untagged,
    map_albums_and_artists,
    process_mp3,
    sanitize_folder_name,
    scan_for_mp3_files,
};

#[doc(hidden)] pub mod config;
#[doc(inline)] pub use config::{CONFIG_FILENAME, Config};

#[doc(hidden)] pub mod cli;
#[doc(inline)] pub use cli::{OrganizerCli, StampCli};

#[doc(hidden)] pub mod logging;
#[doc(inline)] pub use logging::setup_logger;
