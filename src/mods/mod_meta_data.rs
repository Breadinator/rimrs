use std::{
    collections::{
        HashMap,
        HashSet,
    },
    path::Path,
    fs,
};
#[allow(clippy::wildcard_imports)]
use crate::mods::{Dependency, regex::*};

/// The mod metadata contained in its About.xml file.
/// See [`https://www.rimworldwiki.com/wiki/About.xml`].
#[derive(Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ModMetaData {
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
    #[allow(clippy::missing_errors_doc)] // temp
    pub fn read<P: AsRef<Path> + Clone>(path: P) -> anyhow::Result<Self> {
        let text = String::from_utf8(fs::read(path)?)?;

        Ok(Self {
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

