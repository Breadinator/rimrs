use std::{
    collections::HashMap,
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
    pub supportedVersions: Option<Vec<String>>,
    pub description: Option<String>,
    pub descriptionsByVersion: Option<HashMap<String, String>>,

    // dependencies
    pub modDependencies: Option<Vec<Dependency>>,
    pub modDependenciesByVersion: Option<HashMap<String, Vec<Dependency>>>,

    // load order
    pub loadAfter: Option<Vec<String>>,
    pub loadAfterByVersion: Option<HashMap<String, Vec<String>>>,
    pub forceLoadAfter: Option<Vec<String>>,
    pub loadBefore: Option<Vec<String>>,
    pub loadBeforeByVersion: Option<HashMap<String, Vec<String>>>,
    pub forceLoadBefore: Option<Vec<String>>,

    // incompat
    pub incompatibleWith: Option<Vec<String>>,
    pub incompatibleWithByVersion: Option<HashMap<String, Vec<String>>>,
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
            supportedVersions: parse_supportedVersions(&text).map(|versions| versions.into_iter().map(String::from).collect()),
            description: parse_description(&text).map(String::from),
            descriptionsByVersion: None,

            modDependencies: None,
            modDependenciesByVersion: None,

            loadAfter: parse_loadAfter(&text).map(|ids| ids.into_iter().map(String::from).collect()),
            loadAfterByVersion: None,
            forceLoadAfter: parse_forceLoadAfter(&text).map(|ids| ids.into_iter().map(String::from).collect()),
            loadBefore: parse_loadBefore(&text).map(|ids| ids.into_iter().map(String::from).collect()),
            loadBeforeByVersion: None,
            forceLoadBefore: parse_forceLoadBefore(&text).map(|ids| ids.into_iter().map(String::from).collect()),

            incompatibleWith: parse_incompatibleWith(&text).map(|ids| ids.into_iter().map(String::from).collect()),
            incompatibleWithByVersion: None,
        })
    }
}

