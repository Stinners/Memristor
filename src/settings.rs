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

pub struct ConfigStore {
    dir_path: PathBuf,
    config_file: PathBuf,
}

impl ConfigStore {
    pub fn init() -> Result<ConfigStore, SettingsError> {
        // Init config dir
        let mut config = get_config_dir()?;
        if !(Path::new(&config)).exists() {
            fs::create_dir_all(&config)?;
        }

        // Init config File
        let config_file_path = config.join(CONFIG_FILE);
        if !(Path::new(&config_file_path)).exists() {
            let mut config_file = fs::File::create_new(&config_file_path)?;
            let settings_text = json::to_string(&Settings::default());
            config_file.write_all(settings_text.as_bytes())?
        }
        Ok(ConfigStore {
            dir_path: config,
            config_file: config_file_path,
        })
    }

    pub fn read(&self) -> Result<Settings, SettingsError> {
        dbg!(&self.config_file);
        let file_contents = fs::read_to_string(&self.config_file)?;
        let config: Settings = json::from_str(&file_contents)?;
        Ok(config)
    }

    pub fn write(&self, settings: &Settings) -> Result<(), SettingsError> {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.config_file)?;
        file.write_all(json::to_string(settings).as_bytes());
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    root_dir: Option<String>
}

impl Settings {
    pub fn default() -> Self {
        Settings {
            root_dir: None,
        }
    }
}

fn get_config_dir() -> Result<PathBuf, SettingsError> {
    let mut config = env::home_dir().ok_or(
        io::Error::new(io::ErrorKind::NotFound, "Couldn't get Home directory")
    )?;
    config.push(CONFIG_DIR);
    Ok(config)
}
