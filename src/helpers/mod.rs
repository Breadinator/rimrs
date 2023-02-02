use std::{
    io::Read,
    sync::{
        Arc,
        Mutex,
        atomic::AtomicUsize,
    },
};
use once_cell::sync::Lazy;
use thiserror::Error;

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

static ID_COUNTER: Lazy<AtomicUsize> = Lazy::new(|| AtomicUsize::new(0));

#[must_use]
pub fn fetch_inc_id() -> usize {
    ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

#[derive(Debug, Error)]
pub enum ReadLineError {
    #[error("couldn't read file: {0}")]
    IOError(#[from] std::io::Error),
    #[error("invalid utf-8: {0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
}

/// Truncates a long `String`. If too long, it'll make it end in "...".
///
/// Very scuffed, because the font isn't monospace.
/// Would be better to instead go `char`-by-`char` and calculate the actual width.
#[must_use]
pub fn truncate(s: &String, width: f32) -> String {
    /// String to add to truncated strings.
    const APPEND: &str = "...";

    // should always be positive,
    // and shouldn't truncate unless uuuuuuuuuuuuultra wide screen monitor lol
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let max_chars: usize = (width / 7.0) as usize;

    if s.len() > max_chars && max_chars > APPEND.len() {
        let mut st: String = s.chars().take(max_chars - APPEND.len()).collect();
        st.push_str(APPEND);
        st
    } else {
        s.clone()
    }
}

