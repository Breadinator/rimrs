use std::{
    collections::HashMap,
    path::PathBuf,
    io,
    fs::{
        self,
        DirEntry,
    },
};
use crate::ModMetaData;

#[non_exhaustive]
#[derive(Debug, Clone, Default)]
pub struct ModList {
    pub mods: HashMap<String, ModMetaData>,
    pub order: Vec<String>,
}

impl<I: IntoIterator<Item=ModMetaData>> From<I> for ModList {
    fn from(mods: I) -> Self {
        ModList {
            mods: mods.into_iter()
                      .filter_map(|m| m.packageId.clone().map(|pid| (pid, m)))
                      .collect(),
            ..Default::default()
        }
    }
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

