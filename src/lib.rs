#![allow(unused)]
pub(crate) mod errors;
pub use errors::{Error, Exit, Result};
pub mod dispatch;
pub use dispatch::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher};

#[doc(hidden)] pub mod mp3;
#[doc(inline)]
pub use mp3::{
    is_mp3,
    map_albums_and_artists,
    process_mp3,
    sanitize_folder_name,
};
