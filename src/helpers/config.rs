use std::{
    env::{self, VarError},
    path::PathBuf,
};

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

