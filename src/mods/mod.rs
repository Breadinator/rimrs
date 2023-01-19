use std::{
    path::Path,
    ops::{
        Deref,
        DerefMut,
    },
    fs::{
        self,
        DirEntry,
    },
};
use anyhow::Result;

mod mod_meta_data;
pub use mod_meta_data::ModMetaData;

mod dependency;
pub use dependency::Dependency;

mod mod_list;
pub use mod_list::*;

mod regex;

pub struct Mod {}

impl TryFrom<DirEntry> for Mod {
    type Error = anyhow::Error;
    fn try_from(_dir_entry: DirEntry) -> Result<Self, Self::Error> {
        Ok(Mod{})
    }
}

pub trait ModGetter {
    /// Tries to get mods from a given path.
    ///
    /// # Errors
    /// * Returns an error if it can't [`fs::read_dir`] the given path
    fn try_get_mods(&self) -> Result<Mods>;
}

impl<P: AsRef<Path>> ModGetter for P {
    fn try_get_mods(&self) -> Result<Mods> {
        // I tried way too hard to make this in a functional style.
        // Maybe it's possible but I couldn't get `fs::read_dir?` to map or filter or fold

        let mut mods = Vec::new();

        for file in fs::read_dir(self.as_ref())? {
            if let Ok(file) = file {
                if let Ok(m) = Mod::try_from(file) {
                    mods.push(m);
                }
            } else if let Err(err) = file {
                return Err(err.into());
            }
        }

        Ok(Mods::from(mods))
    }
}

/// Wrapper type for `Vec<Mod>`
pub struct Mods(Vec<Mod>);
impl Deref for Mods {
    type Target = Vec<Mod>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Mods {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<Vec<Mod>> for Mods {
    fn from(mods: Vec<Mod>) -> Self {
        Self(mods)
    }
}

