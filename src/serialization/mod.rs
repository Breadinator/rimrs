/// For parsing a mod's `About.xml` file.
pub mod about;

/// For parsing rimworld's `ModsConfig.xml` file, which stores the list of mods that will be loaded when played.
pub mod mods_config;

/// For parsing the rimpy's configuration file.
pub mod rimpy_config;

/// For parsing files in the `.ini` format.
pub mod ini;

#[derive(Debug, thiserror::Error)]
pub enum ParseXMLError {
    #[error("couldn't read file: {0}")]
    IOError(#[from] std::io::Error),
    #[error("invalid XML: {0}")]
    XMLError(#[from] xml::reader::Error),
}
