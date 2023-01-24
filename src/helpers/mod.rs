pub mod config;
pub mod traits;

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

