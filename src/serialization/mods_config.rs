use crate::helpers::{strip_bom, fold_lis};
use std::path::Path;
use xml::reader::{
    EventReader,
    XmlEvent,
};

/// Represents the file `ModsConfig.xml` in rimworld's config directory,
/// `.../AppData/LocalLow/Ludeon Studios/RimWorld by Ludeon Studios/Config/`.
///
/// Fields directly represent the XML tag names, so aren't snakecase.
/// The `ModsConfig.xml` file uses `<li>` tags for elements of lists, represented in this struct as [`Vec`]s.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Default)]
pub struct ModsConfig {
    /// RimWorld version.
    /// E.g. `1.4.3613 rev641`
    pub version: Option<String>,
    pub activeMods: Vec<String>,
    pub knownExpansions: Vec<String>,
}

impl TryFrom<&[u8]> for ModsConfig {
    type Error = xml::reader::Error;

    /// Tries to convert from the given bytes into [`ModsConfig`].
    ///
    /// # Errors
    /// * [`xml::reader::Error`]: invalid XML
    ///
    /// # Panics
    /// * If [`XmlEvent::Characters`] is detected outside of one of the expected tags.
    /// * [`bytes.len() < 3`]
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        enum Section {
            Version,
            ActiveMods,
            KnownExpansions,
        }

        let reader = EventReader::new(strip_bom(bytes));
        let mut mods_config = Self::default();

        let mut section: Option<Section> = None;
        for event in reader {
            match event? {
                XmlEvent::StartElement { name, .. } => {
                    if name.local_name == "version" {
                        section = Some(Section::Version);
                    } else if name.local_name == "activeMods" {
                        section = Some(Section::ActiveMods);
                    } else if name.local_name == "knownExpansions" {
                        section = Some(Section::KnownExpansions);
                    }
                }
                XmlEvent::EndElement { name } => {
                    if name.local_name == "version" || name.local_name == "activeMods" || name.local_name == "knownExpansions" {
                        section = None;
                    }
                }
                XmlEvent::Characters(text) => {
                    match section.as_ref().expect("Unexpected Characters when parsing `ModsConfig`") {
                        Section::Version => mods_config.version = Some(text),
                        Section::ActiveMods => mods_config.activeMods.push(text),
                        Section::KnownExpansions => mods_config.knownExpansions.push(text),
                    }
                }
                _ => {}
            }
        }

        Ok(mods_config)
    }
}

impl TryFrom<&Path> for ModsConfig {
    type Error = super::ParseXMLError;

    /// Tries to read a file, then parse its XML as [`ModsConfig`].
    ///
    /// # Errors
    /// * [`xml::reader::Error`]: invalid XML (see `Self::TryFrom<&[u8]>`)
    /// * [`std::io::Error`]: failed to read the file at the given path
    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let bytes = std::fs::read(path)?;
        Self::try_from(bytes.as_slice())
            .map_err(std::convert::Into::into)
    }
}

impl From<ModsConfig> for String {
    fn from(mods_config: ModsConfig) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<ModsConfigData>\n");

        if let Some(version) = &mods_config.version {
            out.push_str(&format!("    <version>{version}</version>\n"));
        }
        if !mods_config.activeMods.is_empty() {
            out.push_str("    <activeMods>\n");
            out.push_str(&fold_lis(mods_config.activeMods, 2));
            out.push_str("    </activeMods>\n");
        }
        if !mods_config.knownExpansions.is_empty() {
            out.push_str("    <knownExpansions>\n");
            out.push_str(&fold_lis(mods_config.knownExpansions, 2));
            out.push_str("    </knownExpansions>\n");
        }

        out.push_str("</ModsConfigData>");

        out
    }
}

