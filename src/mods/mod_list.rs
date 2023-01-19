use std::collections::HashMap;
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

