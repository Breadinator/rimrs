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
#[allow(clippy::wildcard_imports)]
use crate::mods::{Dependency, regex::*};

/// The mod metadata contained in its About.xml file.
/// See [`https://www.rimworldwiki.com/wiki/About.xml`].
#[allow(non_snake_case)]
#[non_exhaustive]
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
    /// * [`std::io::Error`]: if [`fs::read`] fails
    pub fn read<P: AsRef<Path> + Clone>(path: P) -> Result<Self, std::io::Error> {
        let text_as_u8 = fs::read(path.clone())?;
        let text = String::from_utf8_lossy(&text_as_u8);

        Ok(Self {
            path: Some(PathBuf::from(path.as_ref())),
            name: parse_name(&text).map(String::from),
            author: parse_author(&text).map(String::from),
            authors: parse_authors(&text).map(|authors| authors.into_iter().map(String::from).collect()),
            url: parse_url(&text).map(String::from),
            packageId: parse_packageId(&text).map(String::from),
            supportedVersions: parse_supportedVersions(&text),
            description: parse_description(&text).map(String::from),
            descriptionsByVersion: None,

            modDependencies: parse_modDependencies(&text),
            modDependenciesByVersion: None,

            loadAfter: parse_loadAfter(&text),
            loadAfterByVersion: None,
            forceLoadAfter: parse_forceLoadAfter(&text),
            loadBefore: parse_loadBefore(&text),
            loadBeforeByVersion: None,
            forceLoadBefore: parse_forceLoadBefore(&text),

            incompatibleWith: parse_incompatibleWith(&text),
            incompatibleWithByVersion: None,
        })
    }
}

