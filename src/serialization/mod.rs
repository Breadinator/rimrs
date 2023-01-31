use thiserror::Error;

pub mod about;
pub mod mods_config;
pub mod rimpy_config;
pub mod ini;

#[derive(Debug, Error)]
pub enum ParseXMLError {
    #[error("couldn't read file: {0}")]
    IOError(#[from] std::io::Error),
    #[error("invalid XML: {0}")]
    XMLError(#[from] xml::reader::Error),
}

