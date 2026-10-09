use std::collections::BTreeMap;

use id3::{Tag, TagLike};
use iocore::Path;
use serde::{Deserialize, Serialize};

use crate::{Error, Result};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AlbumAndArtistMap {
    pub albums: BTreeMap<Path, String>,
    pub artists: BTreeMap<Path, String>,
}
impl AlbumAndArtistMap {
    pub fn extend(&mut self, other: &AlbumAndArtistMap) {
        self.albums.extend(other.albums.clone());
        self.artists.extend(other.artists.clone());
    }
}

pub fn is_mp3(path: &Path) -> bool {
    path.extension().unwrap_or_default().ends_with("mp3")
}
pub fn map_albums_and_artists(search_target_dir: &Path) -> Result<AlbumAndArtistMap> {
    if !search_target_dir.is_dir() {
        return Err(Error::InputError(format!("target dir must be an existing directory")));
    }
    let mut map = AlbumAndArtistMap::default();
    // TODO: scan search_target_dir, extracting Artist and Album and filling the map
    for path in search_target_dir.list()? {
        if path.is_dir() {
            map.extend(&map_albums_and_artists(&path)?);
        } else if path.is_file() {
            if is_mp3(&path) {
                match process_mp3(&path)? {
                    Some((artist, album)) => {
                        map.artists.insert(path.clone(), artist.clone());
                        map.albums.insert(path.clone(), album.clone());
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
pub fn process_mp3(file_path: &Path) -> Result<Option<(String, String)>> {
    // Attempt to read ID3 tags from the MP3 file
    log::debug!("reading file {file_path}");
    let tag = Tag::read_from_path(file_path)?;
    log::info!("processing mp3 file {file_path}");
    match (tag.artist(), tag.album()) {
        (Some(artist), Some(album)) => Ok(Some((artist.to_string(), album.to_string()))),
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
