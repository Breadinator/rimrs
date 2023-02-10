use crate::serialization::ini::{
    INIReader,
    INIError,
};
use std::{
    collections::HashMap,
    path::{PathBuf, Path},
};
use thiserror::Error;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Default)]
pub struct RimPyConfig {
    pub colors: Option<HashMap<String, String>>,
    pub folders: RimPyConfigFolders,
    pub startup_params: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct RimPyConfigFolders {
    pub config_folder: Option<PathBuf>,
    pub game_folder: Option<PathBuf>,
    pub local_mods: Option<PathBuf>,
    pub expansions: Option<PathBuf>,
    pub steam_mods: Option<PathBuf>,
    pub steamcmd: Option<PathBuf>,
}

impl RimPyConfig {
    /// Tries to read the rimpy config from its `config.ini` file.
    ///
    /// # Errors
    /// * If it can't find the `config.ini` file in the first place
    /// * If it can't open the file (e.g. doesn't exist, perms)
    /// * If it can't parse that file as INI-syntax
    pub fn from_file() -> Result<Self, ReadRimPyConfigError> {
        Self::try_from(INIReader::from_rimpy_config_ini()?)
            .map_err(Into::into)
    }

    /// Tries to read the rimpy config from a given path.
    ///
    /// # Errors
    /// * If it can't open the file (e.g. doesn't exist, perms)
    /// * If it can't parse that file as INI-syntax
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, INIError> {
        Self::try_from(INIReader::new(path).map_err(INIError::IOError)?)
            .map_err(Into::into)
    }
}

impl TryFrom<INIReader<'_>> for RimPyConfig {
    type Error = INIError;
    fn try_from(reader: INIReader<'_>) -> Result<Self, INIError> {
        let mut conf = Self::default();
        let mut colors = HashMap::new();

        for kvp in reader {
            let kvp = kvp?; // kinda annoying
            match kvp.section.as_deref() {
                Some("Folders") => {
                    match kvp.key.as_str() {
                        "Config folder" => conf.folders.config_folder = Some(PathBuf::from(kvp.value)),
                        "Game folder" => conf.folders.game_folder = Some(PathBuf::from(kvp.value)),
                        "Local mods" => conf.folders.local_mods = Some(PathBuf::from(kvp.value)),
                        "Expansions" => conf.folders.expansions = Some(PathBuf::from(kvp.value)),
                        "Steam mods" => conf.folders.steam_mods = Some(PathBuf::from(kvp.value)),
                        "SteamCMD" => conf.folders.steamcmd = Some(PathBuf::from(kvp.value)),
                        _ => {}
                    }
                }
                Some("Colors") => {
                    colors.insert(kvp.key, kvp.value);
                }
                Some("StartupParams") => {
                    conf.startup_params = Some(kvp.value);
                }
                _ => {}
            }
        }

        if !colors.is_empty() {
            conf.colors = Some(colors);
        }

        Ok(conf)
    }
}

#[derive(Debug, Error)]
pub enum ReadRimPyConfigError {
    #[error("couldn't read env variable: {0}")]
    VarError(#[from] std::env::VarError),
    #[error("{0}")]
    INIError(#[from] crate::serialization::ini::INIError),
}

