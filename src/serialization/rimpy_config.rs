use crate::helpers::config::get_config_dir;
use std::{
    collections::HashMap,
    path::{
        PathBuf,
        Path
    },
};
use configparser::ini::Ini;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
pub struct RimPyConfig {
    pub colors: Option<HashMap<String, String>>,
    pub folders: RimPyConfigFolders,
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
    #[allow(clippy::missing_errors_doc)]
    pub fn from_file() -> Result<Self, ReadRimPyConfigError> {
        let mut path = get_config_dir()?;
        path.push("config.ini");
        Self::from_path(path)
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, ReadRimPyConfigError> {
        let ini = Ini::new().load(path)
            .map_err(ReadRimPyConfigError::INIError)?;
        Self::from_ini_map(ini)
    }

    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::needless_pass_by_value)]
    pub fn from_ini_map(ini: HashMap<String, HashMap<String, Option<String>>>) -> Result<Self, ReadRimPyConfigError> {
        // probably a way to do this without cloning, but this is only ran once and i can't afford the brain cells
        let colors: Option<HashMap<String, String>> = ini.get("colors").map(|colors| colors.iter()
            .filter_map(|(k, v)| { v.as_ref().map(|v| (k.clone(), v.clone())) })
            .collect()
        );

        let folders = if let Some(ini_folders) = ini.get("folders") {
            RimPyConfigFolders::from(ini_folders)
        } else {
            RimPyConfigFolders::default()
        };

        Ok(Self { colors, folders })
    }
}

impl From<&HashMap<String, Option<String>>> for  RimPyConfigFolders {
    fn from(map: &HashMap<String, Option<String>>) -> Self {
        let mut out = RimPyConfigFolders::default();

        if let Some(config_folder) = map.get("config folder") {
            out.config_folder = config_folder.clone().map(PathBuf::from);
        }
        if let Some(game_folder) = map.get("game folder") {
            out.game_folder = game_folder.clone().map(PathBuf::from);
        }
        if let Some(local_mods) = map.get("local mods") {
            out.local_mods = local_mods.clone().map(PathBuf::from);
        }
        if let Some(expansions) = map.get("expansions") {
            out.expansions = expansions.clone().map(PathBuf::from);
        }
        if let Some(steam_mods) = map.get("steam mods") {
            out.steam_mods = steam_mods.clone().map(PathBuf::from);
        }
        if let Some(steamcmd) = map.get("steam cmd") {
            out.steamcmd = steamcmd.clone().map(PathBuf::from);
        }

        out
    }
}

#[derive(Debug)]
pub enum ReadRimPyConfigError {
    VarError(std::env::VarError),
    INIError(String),
}

impl From<std::env::VarError> for ReadRimPyConfigError {
    fn from(err: std::env::VarError) -> Self {
        Self::VarError(err)
    }
}

