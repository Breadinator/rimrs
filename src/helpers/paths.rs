use std::path::PathBuf;
use crate::traits::PushChained;

const MODS_CONFIG_FILE_NAME: &str = "ModsConfig.xml";
const MOD_LIST_DIR_NAME: &str = "ModLists";

#[must_use]
pub fn push_mods_config_path(path: PathBuf) -> PathBuf {
    path.push_chained(MODS_CONFIG_FILE_NAME)
}

#[must_use]
pub fn push_mod_lists_path(path: PathBuf) -> PathBuf {
    path.push_chained(MOD_LIST_DIR_NAME)
}

#[allow(clippy::ptr_arg)]
#[must_use]
pub fn path_to_str(path: &PathBuf) -> Option<&str> {
    path.as_os_str()
        .to_str()
}

