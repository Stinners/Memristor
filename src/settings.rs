#![allow(dead_code, unused)]

use miniserde::{json, Serialize, Deserialize, Error};
use thiserror::Error;

use std::env;
use std::fs;
use std::io::{self, prelude::*};
use std::path::{PathBuf, Path};

const CONFIG_DIR: &'static str = ".config/memristor";
const CONFIG_FILE: &'static str = "config.json";

#[derive(Error, Debug)]
pub enum SettingsError {
    #[error("Could not find user config directory")]
    CouldNotGetConfigDir(#[from] io::Error),

    #[error("Could not deserialize settings file")]
    FileReadError,

    #[error("Could not deserialize config data")]
    DeserializationError(#[from] Error),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub root_dir: Option<String>
}

impl Settings {
    fn default() -> Self {
        Settings {
            root_dir: None,
        }
    }

    fn config_path() -> Result<PathBuf, SettingsError> {
        let mut config_dir_path = env::home_dir().ok_or(
            io::Error::new(io::ErrorKind::NotFound, "Couldn't get Home directory")
        )?;
        config_dir_path.push(CONFIG_DIR);

        if !(Path::new(&config_dir_path)).exists() {
            fs::create_dir_all(&config_dir_path).map_err(|err| SettingsError::CouldNotGetConfigDir(err))?;
        };

        let config_file_path = config_dir_path.join(CONFIG_FILE);
        if !(Path::new(&config_file_path)).exists() {
            let mut config_file = fs::File::create_new(&config_file_path)?;
            let settings_text = json::to_string(&Settings::default());
            config_file.write_all(settings_text.as_bytes())?
        }
        Ok(config_file_path)
    }

    pub fn write(&self) -> Result<(), SettingsError> {
        let config_file_path = Self::config_path()?;
        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(config_file_path)?;
        file.write_all(json::to_string(self).as_bytes());
        Ok(())
    }

    pub fn read() -> Result<Settings, SettingsError> {
        let config_file_path = Self::config_path()?;
        let file_contents = fs::read_to_string(&config_file_path)?;
        let config: Settings = json::from_str(&file_contents)?;
        Ok(config)
    }

}
