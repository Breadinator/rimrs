use crate::{
    mods::Dependency,
    serialization::{about::parse_about, ParseXMLError},
};
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
};

const CORE: &str = "ludeon.rimworld";
const ROYALTY: &str = "ludeon.rimworld.royalty";
const IDEOLOGY: &str = "ludeon.rimworld.ideology";
const BIOTECH: &str = "ludeon.rimworld.biotech";

/// The mod metadata contained in its About.xml file.
/// See [`https://www.rimworldwiki.com/wiki/About.xml`].
#[allow(non_snake_case)]
#[derive(Debug, Clone, Default)]
pub struct ModMetaData {
    pub path: Option<PathBuf>,

    // info
    pub name: Option<String>,
    pub author: Option<String>,
    pub authors: Option<Vec<String>>,
    pub url: Option<String>, // nothing forcing it to be valid
    pub packageId: Option<String>,
    pub supportedVersions: Option<HashSet<String>>,
    pub description: Option<String>,
    pub descriptionsByVersion: Option<HashMap<String, String>>,

    // dependencies
    pub modDependencies: Option<HashSet<Dependency>>,
    pub modDependenciesByVersion: Option<HashMap<String, HashSet<Dependency>>>,

    // load order
    pub loadAfter: Option<HashSet<String>>,
    pub loadAfterByVersion: Option<HashMap<String, HashSet<String>>>,
    pub forceLoadAfter: Option<HashSet<String>>,
    pub loadBefore: Option<HashSet<String>>,
    pub loadBeforeByVersion: Option<HashMap<String, HashSet<String>>>,
    pub forceLoadBefore: Option<HashSet<String>>,

    // incompat
    pub incompatibleWith: Option<HashSet<String>>,
    pub incompatibleWithByVersion: Option<HashMap<String, HashSet<String>>>,
}

impl ModMetaData {
    /// Reads and parses given `About.xml` file path into [`ModMetaData`]
    ///
    /// # Errors
    /// * [`std::io::Error`]: if it fails to read the file at the given path
    /// * [`xml::reader::Error`]: if it tries to parse invalid XML
    #[allow(clippy::missing_panics_doc)]
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Self, ParseXMLError> {
        log::debug!("Parsing {:?}", path.as_ref());
        let file = fs::read(path.as_ref())?;

        // parse
        let mut mmd = parse_about(&file).map_err(ParseXMLError::from)?;
        mmd.path = Some(PathBuf::from(path.as_ref()));

        let mut load_before = HashSet::new();
        if let Some(lb) = mmd.loadBefore.as_ref() {
            load_before.extend(lb);
        }
        if let Some(lb) = mmd.forceLoadBefore.as_ref() {
            load_before.extend(lb);
        }
        let load_before: HashSet<_> = load_before.iter().map(|s| s.as_str()).collect();

        // add load after core if not present
        if mmd.loadAfter.is_none() {
            mmd.loadAfter = Some(HashSet::with_capacity(4));
        }
        let load_after = mmd.loadAfter.as_mut().unwrap();
        #[allow(clippy::collapsible_if)]
        if mmd.packageId.as_ref().map_or(true, |pid| {
            !pid.to_lowercase().starts_with("ludeon.rimworld")
        }) {
            if !load_before.contains(CORE) {
                load_after.insert(String::from(CORE));

                if !load_before.contains(ROYALTY) {
                    load_after.insert(String::from(ROYALTY));

                    if !load_before.contains(IDEOLOGY) {
                        load_after.insert(String::from(IDEOLOGY));

                        if !load_before.contains(BIOTECH) {
                            load_after.insert(String::from(BIOTECH));
                        }
                    }
                }
            }
        }

        Ok(mmd)
    }
}
