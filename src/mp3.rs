use std::collections::BTreeMap;

use id3::{Tag, TagLike};
use iocore::Path;
use serde::{Deserialize, Serialize};

use crate::{Error, Result};

pub fn is_mp3(path: &Path) -> bool {
    path.extension().unwrap_or_default().ends_with("mp3")
}
pub fn map_albums_and_artists(
    search_target_dir: &Path,
) -> Result<BTreeMap<Path, (String, String, String)>> {
    if !search_target_dir.is_dir() {
        return Err(Error::InputError(format!("target dir must be an existing directory")));
    }
    let mut map = BTreeMap::<Path, (String, String, String)>::new();

    // TODO: scan search_target_dir, extracting Artist and Album and filling the map
    for path in search_target_dir.list()? {
        if path.is_dir() {
            map.extend(map_albums_and_artists(&path)?);
        } else if path.is_file() {
            if is_mp3(&path) {
                log::info!("found mp3 at {path}");
                match process_mp3(&path)? {
                    Some((artist, album, title)) => {
                        log::info!("processing mp3 of {artist}/{album} {title}");
                        map.insert(path.clone(), (artist.clone(), album.clone(), title.clone()));
                    }
                    None => {
                        eprintln!("ignoring untagged mp3 file: {path}");
                    }
                }
            } else {
                eprintln!("skipping non-mp3 file: {path}");
            }
        } else {
            eprintln!("ignoring untagged mp3 file: {path}");
        }
    }
    Ok(map)
}
pub fn process_mp3(file_path: &Path) -> Result<Option<(String, String, String)>> {
    // Attempt to read ID3 tags from the MP3 file
    log::debug!("reading file {file_path}");
    let tag = Tag::read_from_path(file_path)?;
    log::warn!("processing mp3 file {file_path}");
    match (tag.artist(), tag.album(), tag.title()) {
        (Some(artist), Some(album), Some(title)) => {
            Ok(Some((artist.to_string(), album.to_string(), title.to_string())))
        }
        _ => Ok(None),
    }
}

// Helper function to remove filesystem-breaking characters
pub fn sanitize_folder_name(name: &str) -> String {
    let sanitized: String = name
        .chars()
        .map(|c| match c {
            '/' | '\\' | '<' | '>' | ':' | '"' | '|' | '?' | '*' => '_',
            _ => c,
        })
        .collect();

    if sanitized.is_empty() { "Unknown".to_string() } else { sanitized }
}
