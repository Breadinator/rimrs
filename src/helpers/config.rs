use std::{
    env::{self, VarError},
    path::PathBuf,
    collections::HashMap,
    fs::File,
};
use serde::{Serialize, Deserialize};

/// Gets the `RimPy` config directory.
/// Its main config file is in `config.ini`, and mod lists are in `ModLists/`
///
/// # Errors
/// See [`env::var`]
pub fn get_config_dir() -> Result<PathBuf, VarError> {
    get_config_dir_from_env()
}

#[cfg(target_os="windows")]
fn get_config_dir_from_env() -> Result<PathBuf, VarError> {
    env::var("APPDATA")
        .map(|path_raw| {
            let mut path = PathBuf::from(path_raw);
            path.pop();
            path.push("LocalLow");
            path.push("RimPy Mod Manager");
            path
        })
}

#[cfg(not(target_os="windows"))]
fn get_config_dir_from_env() -> Result<PathBuf, VarError> {
    log::error!("Unimplemented for operating systems other than Windows.");
    Err(VarError::NotPresent)
}

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
    pub fn from_file() -> anyhow::Result<Self> {
        let mut conf_dir = get_config_dir()?;
        conf_dir.push("config.ini");
        let file = File::open(conf_dir)?;
        serde_ini::from_read(file)
            .map_err(std::convert::Into::into)
    }
}

