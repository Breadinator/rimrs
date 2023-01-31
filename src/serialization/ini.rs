use std::{
    path::Path,
    fs::File,
    io::Read,
};
use crate::helpers::{read_line, traits::LogIfErr, config::get_config_ini_path};
use thiserror::Error;

pub struct INIReader<'a> {
    reader: Box<dyn Read + 'a>,
    section: Option<String>,
    buf: [u8; 1],
}

impl INIReader<'_> {
    /// Starts an [`INIReader`] using the given [`Path`].
    ///
    /// # Errors
    /// * Failes to open the file at the given [`Path`] using `File::open`.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        Ok(Self::from(Box::new(File::open(path)?) as Box<dyn Read>))
    }

    /// Tries to get the `INIReader` for the rimpy config file.
    ///
    /// # Errors
    /// * If [`get_config_ini_path`] fails
    /// * If `Self::new` fails using that path; i.e. if it can't open it with `File::open`
    pub fn from_rimpy_config_ini() -> Result<Self, INIError> {
        let path = get_config_ini_path().map_err(INIError::VarError)?;
        Self::new(path).map_err(INIError::IOError)
    }
}

impl<'a> From<Box<dyn Read + 'a>> for INIReader<'a> {
    fn from(reader: Box<dyn Read + 'a>) -> Self {
        Self {
            reader,
            section: None,
            buf: [0u8],
        }
    }
}

impl Iterator for INIReader<'_> {
    type Item = Result<INIKeyValuePair, INIError>;

    /// Returns `Some(Ok(_))` if it could parse another line.
    /// Returns `None` if it reached the end of the file.
    /// Returns `Some(Err(_))` if invalid text was found.
    ///
    /// Panics if there's an invalid section header (e.g. starts with `[` but has no closing `]`).
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(Some(mut line)) = read_line(&mut self.reader, &mut self.buf).log_if_err() {
            // Blank line or comment
            if line.len() < 3
                || !line.contains(|c: char| !c.is_whitespace())
                || line.starts_with(';') {
                continue;
            }

            // Line is a section
            if line.trim().starts_with('[') {
                line.drain(0..=line.find('[').unwrap());
                line.drain(line.find(']').unwrap()..line.len());
                self.section = Some(line);
                continue;
            }

            // Line should be an equals-sign-delimited kvpair
            let parts: Vec<&str> = line.split('=').collect();

            // too few parts
            if parts.len() < 2 {
                return Some(Err(INIError::InvalidData(String::from("Expected `=` sign when parsing line of INI file, found none."))))
            }

            let section = self.section.clone();
            let key = String::from(parts[0].trim());
            let value = String::from(parts[1..].join("=").trim()); // probs very unnecessary heap allocation here lol

            return Some(Ok(INIKeyValuePair { section, key, value }))
        }

        None
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct INIKeyValuePair {
    pub section: Option<String>,
    pub key: String,
    pub value: String,
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum INIError {
    #[error("invalid INI syntax: {0}")]
    InvalidData(String),
    #[error("failed to read INI file: {0}")]
    IOError(#[from] std::io::Error),
    #[error("couldn't read env: {0}")]
    VarError(#[from] std::env::VarError),
}

