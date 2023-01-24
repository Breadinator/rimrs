use crate::helpers::config::get_config_dir;
use serde::{Serialize, Deserialize};
use std::{
    collections::HashMap,
    path::PathBuf,
    fs::File,
};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RimPyConfig {
    #[serde(default, rename = "Colors")]
    pub colors: Option<HashMap<String, String>>,
    #[serde(default, rename = "Folders")]
    pub folders: RimPyConfigFolders,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RimPyConfigFolders {
    #[serde(default, rename="Config folder")]
    pub config_folder: Option<PathBuf>,
    #[serde(default, rename="Game folder")]
    pub game_folder: Option<PathBuf>,
    #[serde(default, rename="Local mods")]
    pub local_mods: Option<PathBuf>,
    #[serde(default, rename="Expansions")]
    pub expansions: Option<PathBuf>,
    #[serde(default, rename="Steam mods")]
    pub steam_mods: Option<PathBuf>,
    #[serde(default, rename="SteamCMD")]
    pub steamcmd: Option<PathBuf>,
}

impl RimPyConfig {
    /// Tries to get the config file, `config.ini`
    ///
    /// # Errors
    /// * [`std::env::VarError`]: If env var, "APPDATA", can't be read. See [`env::var`].
    /// * [`std::io::Error`]: If it can't open `config.ini` in the config dir.
    /// * [`serde_ini::de::Error`]: If it can't parse that file into a [`RimPyConfig`].
    pub fn from_file() -> Result<Self, ReadRimPyConfigError> {
        let mut conf_dir = get_config_dir()?;
        conf_dir.push("config.ini");
        let file = File::open(conf_dir)?;
        serde_ini::from_read(file)
            .map_err(std::convert::Into::into)
    }
}

#[derive(Debug)]
pub enum ReadRimPyConfigError {
    VarError(std::env::VarError),
    IOError(std::io::Error),
    SerdeError(serde_ini::de::Error),
}

impl From<std::env::VarError> for ReadRimPyConfigError {
    fn from(err: std::env::VarError) -> Self {
        Self::VarError(err)
    }
}

impl From<std::io::Error> for ReadRimPyConfigError {
    fn from(err: std::io::Error) -> Self {
        Self::IOError(err)
    }
}

impl From<serde_ini::de::Error> for ReadRimPyConfigError {
    fn from(err: serde_ini::de::Error) -> Self {
        Self::SerdeError(err)
    }
}

