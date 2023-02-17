use crate::{
    helpers::strip_bom,
    mods::{Dependency, ModMetaData},
};
use std::{collections::HashSet, convert::AsRef};
use xml::{
    name::OwnedName,
    reader::{EventReader, XmlEvent},
};

/// Parses the About.xml file from its reader.
///
/// # Errors
/// * [`xml::reader::Error`]: if it fails to parse an [`XmlEvent`]
///
/// # Panics
/// * If `bytes.len() < 3`. See [`strip_bom`].
#[allow(clippy::module_name_repetitions)]
pub fn parse_about(bytes: &[u8]) -> Result<ModMetaData, xml::reader::Error> {
    let reader = EventReader::new(strip_bom(bytes));

    let mut mmd = ModMetaData::default();
    let mut xml_path = Vec::new();
    let mut mem = ParsingMem::default();

    for event in reader {
        match event? {
            XmlEvent::StartElement { name, .. } => {
                xml_path.push(name.local_name);
            }
            XmlEvent::EndElement { name } => {
                end_element(&mut xml_path, &name, &mut mem);
            }
            XmlEvent::Characters(text) => {
                add_data_to_mmd(&mut mmd, &xml_path, text, &mut mem);
            }
            XmlEvent::EndDocument => break,
            _ => {}
        }
    }

    mem.commit_to_mmd(&mut mmd);
    Ok(mmd)
}

fn add_data_to_mmd(mmd: &mut ModMetaData, xml_path: &[String], text: String, mem: &mut ParsingMem) {
    if xml_path.get(0).map(AsRef::as_ref) != Some("ModMetaData") {
        return;
    }

    match xml_path.get(1).map(AsRef::as_ref) {
        // top-level strings
        Some("name") => {
            mmd.name = Some(text);
        }
        Some("description") => {
            mmd.description = Some(text);
        }
        Some("packageId") => {
            mmd.packageId = Some(text);
        }
        Some("author") => {
            mmd.author = Some(text);
        }
        Some("url") => {
            mmd.url = Some(text);
        }

        // 1 deep lists
        Some("loadAfter") => {
            mem.loadAfter.insert(text);
        }
        Some("forceLoadAfter") => {
            mem.forceLoadAfter.insert(text);
        }
        Some("loadBefore") => {
            mem.loadBefore.insert(text);
        }
        Some("forceLoadBefore") => {
            mem.forceLoadBefore.insert(text);
        }
        Some("supportedVersions") => {
            mem.supportedVersions.insert(text);
        }
        Some("authors") => {
            mem.authors.push(text);
        }
        Some("incompatibleWith") => {
            mem.incompatibleWith.insert(text);
        }

        // mod dependencies
        Some("modDependencies") => match xml_path.get(3).map(AsRef::as_ref) {
            Some("packageId") => mem.curr_modDependencies.packageId = Some(text),
            Some("displayName") => mem.curr_modDependencies.displayName = Some(text),
            Some("steamWorkshopUrl") => mem.curr_modDependencies.steamWorkshopUrl = Some(text),
            Some("downloadUrl") => mem.curr_modDependencies.downloadUrl = Some(text),
            _ => {}
        },

        // anything else
        _ => {
            log::warn!("Detected unknown XML tag when parsing an About.xml file");
        }
    }
}

fn end_element(xml_path: &mut Vec<String>, name: &OwnedName, mem: &mut ParsingMem) {
    if xml_path.get(1).map(AsRef::as_ref) == Some("modDependencies") && name.local_name == "li" {
        let dep = mem.curr_modDependencies.clone();
        mem.modDependencies.insert(dep);

        // should be better than mem.curr_modDependencies = Dependency::default(), i think?
        mem.curr_modDependencies.packageId = None;
        mem.curr_modDependencies.displayName = None;
        mem.curr_modDependencies.downloadUrl = None;
        mem.curr_modDependencies.steamWorkshopUrl = None;
    }

    xml_path.pop();
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default)]
struct ParsingMem {
    pub loadAfter: HashSet<String>,
    pub forceLoadAfter: HashSet<String>,
    pub loadBefore: HashSet<String>,
    pub forceLoadBefore: HashSet<String>,
    pub supportedVersions: HashSet<String>,
    pub authors: Vec<String>,
    pub incompatibleWith: HashSet<String>,

    pub curr_modDependencies: Dependency,
    pub modDependencies: HashSet<Dependency>,
}

impl ParsingMem {
    pub fn commit_to_mmd(self, mmd: &mut ModMetaData) {
        if !self.loadAfter.is_empty() {
            mmd.loadAfter = Some(self.loadAfter);
        }
        if !self.forceLoadAfter.is_empty() {
            mmd.forceLoadAfter = Some(self.forceLoadAfter);
        }
        if !self.loadBefore.is_empty() {
            mmd.loadBefore = Some(self.loadBefore);
        }
        if !self.forceLoadBefore.is_empty() {
            mmd.forceLoadBefore = Some(self.forceLoadBefore);
        }
        if !self.supportedVersions.is_empty() {
            mmd.supportedVersions = Some(self.supportedVersions);
        }
        if !self.authors.is_empty() {
            mmd.authors = Some(self.authors);
        }
        if !self.incompatibleWith.is_empty() {
            mmd.incompatibleWith = Some(self.incompatibleWith);
        }

        if !self.modDependencies.is_empty() {
            mmd.modDependencies = Some(self.modDependencies);
        }
    }
}
