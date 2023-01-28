use std::{
    collections::HashMap,
    sync::{
        Arc,
        Mutex,
    },
    path::PathBuf,
    io,
    fs::{
        self,
        DirEntry,
    },
};
use crate::{
    ModMetaData,
    RimPyConfig,
};

#[non_exhaustive]
#[derive(Debug, Clone, Default)]
pub struct ModList {
    pub mods: Arc<Mutex<HashMap<String, ModMetaData>>>,
}

impl ModList {
    /// Looks for `*/About/About.xml` files in the given dirs, then parses them.
    ///
    /// # Errors
    /// * [`fs::read_dir`] fails on one of the given dirs
    pub fn from_dirs(dirs: impl IntoIterator<Item = PathBuf>) -> Result<Self, io::Error> {
        let mut paths = Vec::new();
        for dir in dirs {
            for mod_dir in fs::read_dir(dir)? {
                if let Ok(p) = mod_dir.as_ref().map(DirEntry::path) {
                    if mod_dir
                            .and_then(|md| md.file_type())
                            .map_or(false, |ft| ft.is_dir()) {
                        paths.push(p);
                    }
                }
            }
        }

        for path in &mut paths {
            path.push("About");
            path.push("About.xml");
        }
        paths.retain(|path| path.exists());
        let mods: Vec<_> = paths.into_iter()
            .filter_map(|path| ModMetaData::read(path).ok())
            .collect();
        Ok(ModList::from(mods))
    }
}

impl<I: IntoIterator<Item=ModMetaData>> From<I> for ModList {
    fn from(mods_iter: I) -> Self {
        let mods: HashMap<String, ModMetaData> = mods_iter.into_iter()
              .filter_map(|m| m.packageId.clone().map(|pid| (pid.to_lowercase(), m)))
              .collect();

        ModList {
            mods: Arc::new(Mutex::new(mods)),
        }
    }
}

impl TryFrom<&RimPyConfig> for ModList {
    type Error = io::Error;

    fn try_from(rimpy_config: &RimPyConfig) -> Result<Self, Self::Error> {
        let mut paths = Vec::new();

        if let Some(p) = &rimpy_config.folders.expansions {
            paths.push(p.clone());
        }
        if let Some(p) = &rimpy_config.folders.steam_mods {
            paths.push(p.clone());
        }
        if let Some(p) = &rimpy_config.folders.local_mods {
            paths.push(p.clone());
        }

        ModList::from_dirs(paths)
    }
}

