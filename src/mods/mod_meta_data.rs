use std::{
    collections::{
        HashMap,
        HashSet,
    },
    path::{
        Path,
        PathBuf,
    },
    fs,
};
use crate::{
    mods::Dependency,
    serialization::{
        ParseXMLError,
        about::parse_about,
    },
};

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
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Self, ParseXMLError> {
        log::debug!("Parsing {:?}", path.as_ref());
        let file = fs::read(path.as_ref())?;
        let mut mmd = parse_about(&file)
            .map_err(ParseXMLError::from)?;
        mmd.path = Some(PathBuf::from(path.as_ref()));
        Ok(mmd)
    }
}

