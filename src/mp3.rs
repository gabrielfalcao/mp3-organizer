use std::collections::BTreeMap;

use id3::{Tag, TagLike, Version};
use iocore::Path;
use serde::{Deserialize, Serialize};

use crate::{Error, Result};

pub const UNKNOWN_ARTIST: &'static str = "Unknown Artist";
pub const UNKNOWN_ALBUM: &'static str = "Unknown Album";
pub const UNKNOWN_TITLE: &'static str = "Unknown Title";

pub fn is_mp3(path: &Path) -> bool {
    path.extension().unwrap_or_default().ends_with("mp3")
}
pub fn scan_for_mp3_files(search_target_dir: &Path) -> Result<Vec<Path>> {
    if !search_target_dir.is_dir() {
        return Err(Error::InputError(format!("target dir must be an existing directory")));
    }
    let mut result = Vec::<Path>::new();
    log::info!("scanning path {:#?}", search_target_dir.to_string());
    // TODO: scan search_target_dir, extracting Artist and Album and filling the map
    for path in search_target_dir.list()? {
        if path.is_dir() {
            log::info!("scanning subdir {:#?}", path.to_string());
            result.extend(scan_for_mp3_files(&path)?);
        } else if path.is_file() {
            log::info!("checking file {:#?}", path.to_string());
            if is_mp3(&path) {
                // log::info!("found mp3 file: {path}");
                result.push(path);
                // } else {
                //     log::debug!("skipping non-mp3 file: {path}");
            }
            // } else {
            //     log::debug!("ignoring untagged mp3 file: {path}");
        }
    }
    Ok(result)
}
pub fn map_albums_and_artists(
    search_target_dir: &Path,
) -> Result<BTreeMap<Path, ((String, bool), (String, bool), (String, bool))>> {
    if !search_target_dir.is_dir() {
        return Err(Error::InputError(format!("target dir must be an existing directory")));
    }
    let mp3_files = scan_for_mp3_files(search_target_dir)?;
    log::info!("mapping ID3 tags");
    let mut map = BTreeMap::<Path, ((String, bool), (String, bool), (String, bool))>::new();
    for path in mp3_files.iter() {
        let ((artist, updated_artist), (album, updated_album), (title, updated_title)) =
            process_mp3(path)?;

        map.insert(
            path.clone(),
            (
                (artist.clone(), updated_artist.clone()),
                (album.clone(), updated_album.clone()),
                (title.clone(), updated_title.clone()),
            ),
        );
    }
    Ok(map)
}
pub fn process_mp3(path: &Path) -> Result<((String, bool), (String, bool), (String, bool))> {
    // Attempt to read ID3 tags from the MP3 file
    log::debug!("reading file {path}");
    let mut tag = Tag::read_from_path(path).unwrap_or_else(|e| Tag::new());
    log::warn!("processing mp3 file {path}");
    let artist = tag.artist().map(|t| t.to_string()).unwrap_or_else(|| UNKNOWN_ARTIST.to_string());
    let album = tag.album().map(|t| t.to_string()).unwrap_or_else(|| UNKNOWN_ALBUM.to_string());
    let title = tag.title().map(|t| t.to_string()).unwrap_or_else(|| UNKNOWN_TITLE.to_string());
    let mut updated_artist = false;
    let mut updated_album = false;
    let mut updated_title = false;
    if artist == UNKNOWN_ARTIST || album == UNKNOWN_ALBUM || title == UNKNOWN_TITLE {
        let parts = path
            .without_extension()
            .name()
            .splitn(3, '-')
            .map(|v| v.trim().to_string())
            .collect::<Vec<String>>();
        if parts.len() >= 3 {
            let artist = parts[0].clone();
            let title = parts[1].clone();
            let album = parts[2].clone();
            tag.set_album(&album);
            tag.set_title(&title);
            tag.set_artist(&artist);
            updated_artist = true;
            updated_album = true;
            updated_title = true;
            log::info!("updated artist to: {artist:#?}");
            log::info!("updated album to: {album:#?}");
            log::info!("updated title to: {title:#?}");
            tag.write_to_path(path, Version::Id3v24)?;

            log::warn!(
                "updated ID3 of file {path} with artist {artist:#?}, title {title:#?}, album {album:#?}"
            );
        } else if parts.len() == 2 {
            let title = parts[0].clone();
            let artist = parts[1].clone();
            updated_artist = true;
            updated_title = true;
            log::info!("updated artist to: {artist:#?}");
            log::info!("updated title to: {title:#?}");
            log::info!("updated album to: {title:#?}");
            tag.set_title(&title);
            tag.set_artist(&artist);
            tag.set_album(&artist);
            tag.write_to_path(path, Version::Id3v24)?;

            log::warn!(
                "updated ID3 of file {path} with artist {artist:#?}, title {title:#?}, album {album:#?}"
            );
        }
    }
    Ok((((artist, updated_artist), (album, updated_album), (title, updated_title))))
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
