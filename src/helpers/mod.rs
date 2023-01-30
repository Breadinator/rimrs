use std::{
    io::Read,
    sync::{
        Arc,
        Mutex,
        atomic::AtomicUsize,
    },
};
use once_cell::sync::Lazy;

pub mod config;
pub mod traits;
pub mod ui;
pub mod vec_ops;

mod atomic_flag;
pub use atomic_flag::AtomicFlag;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

/// To get rid of BOM <https://en.wikipedia.org/wiki/Byte_order_mark>, which `xml-rs` doesn't allow.
///
/// # Panics
/// If `bytes.len() < 3`, the length of BOM. Just checks if the first 3 bytes match the BOM sequence.
/// All valid XML should be more than 3 characters long, so this should  be fine.
#[must_use]
pub fn strip_bom(bytes: &[u8]) -> &[u8] {
    if bytes[0..3] == [239, 187, 191] {
        &bytes[3..]
    } else {
        bytes
    }
}

#[must_use]
pub fn fold_lis(items: Vec<String>, indenting: usize) -> String {
    let indent = "    ".repeat(indenting);

    items
    .into_iter()
    .fold(String::new(), |mut acc, item| {
        acc.push_str(&format!("{indent}<li>{item}</li>\n"));
        acc
    })
}

/// Reads a line from a reader.
///
/// # Errors
/// If `reader.read(buf)` returns `Err`
pub fn read_line(reader: &mut impl Read, buf: &mut [u8;1]) -> Result<Option<String>, ReadLineError> {
    let mut line: Vec<u8> = Vec::new();

    loop {
        let n = reader.read(&mut buf[..])?;
        if n == 0 || buf[0] == b'\n' {
            break;
        }
        line.push(buf[0]);
    }

    if line.is_empty() {
        Ok(None)
    } else {
        Ok(Some(String::from_utf8(line)?))
    }
}

pub fn arc_mutex_none<T>() -> Arc<Mutex<Option<T>>> {
    Arc::new(Mutex::new(None))
}

static ID_COUNTER: Lazy<Arc<AtomicUsize>> = Lazy::new(|| Arc::new(AtomicUsize::new(0)));

#[must_use]
pub fn fetch_inc_id() -> usize {
    ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

#[derive(Debug)]
pub enum ReadLineError {
    IOError(std::io::Error),
    FromUtf8Error(std::string::FromUtf8Error),
}

impl From<std::io::Error> for ReadLineError {
    fn from(err: std::io::Error) -> Self {
        Self::IOError(err)
    }
}

impl From<std::string::FromUtf8Error> for ReadLineError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Self::FromUtf8Error(err)
    }
}

impl std::fmt::Display for ReadLineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::IOError(err)       => err.fmt(f),
            Self::FromUtf8Error(err) => err.fmt(f),
        }
    }
}

