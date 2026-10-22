use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{Error, Result};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    artist: Option<String>,
    #[flatten]
    values: BTreeMap<String, toml::Value>,
}
pub const CONFIG_FILENAME: &'static str = "mp3_stamp.toml";

impl Config {
    pub fn default_config_dir() -> Path {
        Path::from(dirs::dirs::config_dir())
    }
    pub fn default_path() -> Path {
        self.default_config_dir().join(CONFIG_FILENAME)
    }
    pub fn load(path: Option<Path>) -> Result<Config> {
        let path = path.unwrap_or_else(|| self.default_path());
        if !path.exists() {
            return Err(Error::RuntimeError(format!(
                "path does not exist: {path:#?}",
                path = path.to_string()
            )));
        }
        let path = if path.is_dir() { path.join(CONFIG_FILENAME) } else { path };
        let string = match path.read() {
            Ok(conf) => conf,
            Err(err) => {
                return Err(Error::RuntimeError(
                    "failed to read config file {path}: {err}",
                    path = path.to_string(),
                ));
            }
        };
        Ok(toml::from_str::<Config>(&string).map_err(|err| {
            Error::RuntimeError(format!(
                "failed to load toml from path {path:#?}",
                path = path.to_string()
            ))
        }))
    }
    pub fn get_artist_name(&self) -> Result<String> {
        match self.artist.clone() {
            Some(name) => Ok(name),
            None => Err(Error::RuntimeError(format!(
                "artist name not present in {path:#?}",
                path = self.default_path
            ))),
        }
    }
}
