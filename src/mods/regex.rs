#![allow(non_snake_case)]

use once_cell::sync::Lazy;
use regex::Regex;

fn parse_first<'a>(text: &'a str, regex: &'static Regex) -> Option<&'a str> {
    regex.captures(text)
        .and_then(|captures| captures.get(1))
        .map(|p| p.as_str())
}

fn parse_lis_basic<'a>(text: &'a str, regex: &'static Regex) -> Option<Vec<&'a str>> {
    regex.captures(text)
        .and_then(|captures| captures.get(0))
        .map(|p| parse_lis(p.as_str()))
}

// LI MATCHER
static LI_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<li>([\s\S]*?)</li>"#).unwrap());
pub fn parse_lis(text: &str) -> Vec<&str> {
    LI_MATCHER.captures_iter(text).filter_map(|capture| {
        capture.get(1).map(|li| li.as_str())
    }).collect()
}

// (mod) NAME
static NAME_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<name>([\w\W]*)</name>"#).unwrap());
pub fn parse_name(text: &str) -> Option<&str> {
    parse_first(text, &NAME_MATCHER)
}

// AUTHOR(s)
static AUTHOR_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<author>([\w\W]*)</author>"#).unwrap());
pub fn parse_author(text: &str) -> Option<&str> {
    parse_first(text, &AUTHOR_MATCHER)
}

static AUTHORS_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<authors>[\w\W]*</authors>"#).unwrap());
pub fn parse_authors(text: &str) -> Option<Vec<&str>> {
    parse_lis_basic(text, &AUTHORS_MATCHER)
}

// url
static URL_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<url>([\w\W]*)</url>"#).unwrap());
pub fn parse_url(text: &str) -> Option<&str> {
    parse_first(text, &URL_MATCHER)
}

// packageId
static PACKAGEID_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<packageId>([\w\W]*)</packageId>"#).unwrap());
pub fn parse_packageId(text: &str) -> Option<&str> {
    parse_first(text, &PACKAGEID_MATCHER)
}

// supported versions
static SUPPORTEDVERSIONS_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<supportedVersions>([\w\W]*)</supportedVersions>"#).unwrap());
pub fn parse_supportedVersions(text: &str) -> Option<Vec<&str>> {
    parse_lis_basic(text, &SUPPORTEDVERSIONS_MATCHER)
}

// description
static DESCRIPTION_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<description>([\w\W]*)</description>"#).unwrap());
pub fn parse_description(text: &str) -> Option<&str> {
    parse_first(text, &DESCRIPTION_MATCHER)
}

// descriptionsByVersions

// modDependencies

// modDependenciesByVersions

// loadAfter
static LOADAFTER_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<loadAfter>[\w\W]*</loadAfter>"#).unwrap());
pub fn parse_loadAfter(text: &str) -> Option<Vec<&str>> {
    parse_lis_basic(text, &LOADAFTER_MATCHER)
}

// loadAfterByVersion

// forceLoadAfter
static FORCELOADAFTER_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<forceLoadAfter>[\w\W]*</forceLoadAfter>"#).unwrap());
pub fn parse_forceLoadAfter(text: &str) -> Option<Vec<&str>> {
    parse_lis_basic(text, &FORCELOADAFTER_MATCHER)
}

// loadBefore
static LOADBEFORE_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<loadBefore>[\w\W]*</loadBefore>"#).unwrap());
pub fn parse_loadBefore(text: &str) -> Option<Vec<&str>> {
    parse_lis_basic(text, &LOADBEFORE_MATCHER)
}

// loadBeforeByVersion

// forceLoadBefore
static FORCELOADBEFORE_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<forceLoadBefore>[\w\W]*</forceLoadBefore>"#).unwrap());
pub fn parse_forceLoadBefore(text: &str) -> Option<Vec<&str>> {
    parse_lis_basic(text, &FORCELOADBEFORE_MATCHER)
}

// incompatableWith
static INCOMPATIBLEWITH_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<incompatibleWith>[\w\W]*</incompatibleWith>"#).unwrap());
pub fn parse_incompatibleWith(text: &str) -> Option<Vec<&str>> {
    parse_lis_basic(text, &INCOMPATIBLEWITH_MATCHER)
}

// incompatableWithByVersion

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_lis() {
        let text = r#"
            <root>
                <li>abc</li>
                <li>123</li>
            </root>"#;
        let lis = super::parse_lis(text);
        assert_eq!(lis, vec!["abc", "123"]);
    }

    #[test]
    fn parse_lis_basic() {
         static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<root>[\w\W]*</root>"#).unwrap());
         let text = r#"
            <root>
                <li>abc</li>
                <li>123</li>
            </root>"#;
         let lis = super::parse_lis_basic(text, &REGEX);
         assert_eq!(lis.unwrap(), vec!["abc", "123"]);
    }
}

