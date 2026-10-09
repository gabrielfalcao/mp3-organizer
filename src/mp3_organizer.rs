use crate::{Result, Error};
use std::collections::BTreeMap;
use iocore::Path;
use id3::{Tag, TagLike};


#[derive(Clone, Debug, Default)]
pub struct AlbumAndArtistMap {
    pub albums: BTreeMap<String, Path>,
    pub artists: BTreeMap<String, Path>,
}

pub fn map_albums_and_artists(search_target_dir: Path) -> Result<AlbumAndArtistMap> {
    if !search_target_dir.is_dir() {
        return Err(Error::InputError(format!("target dir must be an existing directory")))
    }
    let mut map = AlbumAndArtistMap::default();
    // TODO: scan search_target_dir, extracting Artist and Album and filling the map
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define your source folder containing loose MP3 files
    let source_dir = Path::new("./music_inbox");
    // Define where organized folders should be created
    let target_dir = Path::new("./organized_music");

    // Ensure target root exists
    fs::create_dir_all(target_dir)?;

    // Read the source directory contents
    for entry in fs::read_dir(source_dir)? {
        let entry = entry?;
        let path = entry.path();

        // Process only .mp3 files
        if path.is_file() && path.extension().map_or(false, |ext| ext == "mp3") {
            process_mp3(&path, target_dir)?;
        }
    }

    Ok(())
}

pub fn process_mp3(file_path: &Path, target_base: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Attempt to read ID3 tags from the MP3 file
    let (artist, album) = match Tag::read_from_path(file_path) {
        Ok(tag) => {
            // Fallback to "Unknown" variations if tags are empty or missing
            let artist = tag.artist().unwrap_or("Unknown Artist").trim().to_string();
            let album = tag.album().unwrap_or("Unknown Album").trim().to_string();
            (artist, album)
        }
        Err(_) => {
            // Handle files with no valid ID3 tags
            ("Unknown Artist".to_string(), "Unknown Album".to_string())
        }
    };

    // Sanitize folder names to remove characters forbidden by OS filesystems
    let safe_artist = sanitize_folder_name(&artist);
    let safe_album = sanitize_folder_name(&album);

    // Build the full target directory: target_base/Artist/Album/
    let destination_dir = target_base.join(&safe_artist).join(&safe_album);

    // Create the nested folders (equivalent to `mkdir -p`)
    fs::create_dir_all(&destination_dir)?;

    // Get the original filename to preserve it
    if let Some(file_name) = file_path.file_name() {
        let destination_path = destination_dir.join(file_name);

        println!("Moving: {:?} -> {:?}", file_path, destination_path);

        // Move the file to its new location
        fs::rename(file_path, destination_path)?;
    }

    Ok(())
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
