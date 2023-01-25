pub mod about;
pub mod mods_config;
pub mod rimpy_config;

/// Not actually used lol
pub mod ini;

#[derive(Debug)]
pub enum ParseXMLError {
    IOError(std::io::Error),
    XMLError(xml::reader::Error),
}

impl From<std::io::Error> for ParseXMLError {
    fn from(err: std::io::Error) -> Self {
        Self::IOError(err)
    }
}

impl From<xml::reader::Error> for ParseXMLError {
    fn from(err: xml::reader::Error) -> Self {
        Self::XMLError(err)
    }
}

