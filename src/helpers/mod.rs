use std::{
    io::Read,
    sync::atomic::AtomicUsize,
};
use once_cell::sync::Lazy;
use thiserror::Error;

pub mod config;
pub mod traits;
pub mod ui;
pub mod vec_ops;

mod vec_mut_accessor;
pub use vec_mut_accessor::VecMutAccessor;

mod atomic_flag;
pub use atomic_flag::AtomicFlag;

/// Used to represent inactive mods with [`Side::Left`] and active mods with [`Side::Right`].
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
    const BOM: [u8; 3] = [239, 187, 191];
    if bytes[0..3] == BOM {
        &bytes[3..]
    } else {
        bytes
    }
}

/// Takes a [`Vec`] of items (which don't contain the `li` tags).
///
/// # Examples
/// ```
/// use rimrs::helpers::fold_lis;
///
/// let lis = vec!["a", "b"];
/// let folded = fold_lis(&lis, 1);
///
/// assert_eq!(&folded, "    <li>a</li>\n    <li>b</li>\n");
/// ```
#[must_use]
pub fn fold_lis<S: AsRef<str>>(items: &[S], indenting: usize) -> String {
    let single_indent = "    ";
    let indent = single_indent.repeat(indenting);

    items.iter()
        .fold(String::new(), |mut acc, item| {
            acc.push_str(&format!("{indent}<li>{}</li>\n", item.as_ref()));
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

#[derive(Debug, Error)]
pub enum ReadLineError {
    #[error("couldn't read file: {0}")]
    IOError(#[from] std::io::Error),
    #[error("invalid utf-8: {0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
}

static ID_COUNTER: Lazy<AtomicUsize> = Lazy::new(|| AtomicUsize::new(0));

#[must_use]
pub fn fetch_inc_id() -> usize {
    ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
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

