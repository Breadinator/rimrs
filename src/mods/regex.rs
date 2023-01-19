#![allow(non_snake_case)]

#[allow(unused_imports)]
use std::collections::{
    HashMap,
    HashSet,
};
use once_cell::sync::Lazy;
use regex::Regex;
use crate::mods::Dependency;

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
fn parse_lis_basic_hashset(text: &str, regex: &'static Regex) -> Option<HashSet<String>> {
    regex.captures(text)
        .and_then(|cap| cap.get(0))
        .map(|p| parse_lis_hashset(p.as_str()))
}

// LI MATCHER
static LI_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<li>([\s\S]*?)</li>"#).unwrap());
pub fn parse_lis(text: &str) -> Vec<&str> {
    LI_MATCHER.captures_iter(text)
        .filter_map(|cap| cap.get(1).map(|li| li.as_str())).collect()
}
pub fn parse_lis_hashset(text: &str) -> HashSet<String> {
    LI_MATCHER.captures_iter(text)
        .filter_map(|cap| cap.get(1).map(|li| String::from(li.as_str())))
        .collect()
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
pub fn parse_supportedVersions(text: &str) -> Option<HashSet<String>> {
    parse_lis_basic_hashset(text, &SUPPORTEDVERSIONS_MATCHER)
}

// description
static DESCRIPTION_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<description>([\w\W]*)</description>"#).unwrap());
pub fn parse_description(text: &str) -> Option<&str> {
    parse_first(text, &DESCRIPTION_MATCHER)
}

// descriptionsByVersions

// modDependencies
static MODDEPENDENCIES_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<modDependencies>[\w\W]*</modDependencies>"#).unwrap());
pub fn parse_modDependencies(text: &str) -> Option<HashSet<Dependency>> {
    MODDEPENDENCIES_MATCHER.captures(text)
        .and_then(|cap| cap.get(0))
        .map(|cap| parse_lis(cap.as_str()))
        .map(|lis| {
            let mut deps = HashSet::new();
            for li in lis {
                deps.insert(Dependency::from(li));
            }
            deps
        })
}

// mod dependency subcaptures (could maybe be done in the main modDependencies regex?
static DISPLAYNAME_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<displayName>([\w\W]*)</displayName>"#).unwrap());
static DOWNLOADURL_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<downloadUrl>([\w\W]*)</downloadUrl>"#).unwrap());
static STEAMWORKSHOPURL_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<steamWorkshopUrl>([\w\W]*)</steamWorkshopUrl>"#).unwrap());
pub fn parse_displayName(text: &str) -> Option<&str> {
    parse_first(text, &DISPLAYNAME_MATCHER)
}
pub fn parse_downloadUrl(text: &str) -> Option<&str> {
    parse_first(text, &DOWNLOADURL_MATCHER)
}
pub fn parse_steamWorkshopUrl(text: &str) -> Option<&str> {
    parse_first(text, &STEAMWORKSHOPURL_MATCHER)
}

// modDependenciesByVersions

// loadAfter
static LOADAFTER_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<loadAfter>[\w\W]*</loadAfter>"#).unwrap());
pub fn parse_loadAfter(text: &str) -> Option<HashSet<String>> {
    parse_lis_basic_hashset(text, &LOADAFTER_MATCHER)
}

// loadAfterByVersion

// forceLoadAfter
static FORCELOADAFTER_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<forceLoadAfter>[\w\W]*</forceLoadAfter>"#).unwrap());
pub fn parse_forceLoadAfter(text: &str) -> Option<HashSet<String>> {
    parse_lis_basic_hashset(text, &FORCELOADAFTER_MATCHER)
}

// loadBefore
static LOADBEFORE_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<loadBefore>[\w\W]*</loadBefore>"#).unwrap());
pub fn parse_loadBefore(text: &str) -> Option<HashSet<String>> {
    parse_lis_basic_hashset(text, &LOADBEFORE_MATCHER)
}

// loadBeforeByVersion

// forceLoadBefore
static FORCELOADBEFORE_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<forceLoadBefore>[\w\W]*</forceLoadBefore>"#).unwrap());
pub fn parse_forceLoadBefore(text: &str) -> Option<HashSet<String>> {
    parse_lis_basic_hashset(text, &FORCELOADBEFORE_MATCHER)
}

// incompatableWith
static INCOMPATIBLEWITH_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<incompatibleWith>[\w\W]*</incompatibleWith>"#).unwrap());
pub fn parse_incompatibleWith(text: &str) -> Option<HashSet<String>> {
    parse_lis_basic_hashset(text, &INCOMPATIBLEWITH_MATCHER)
}

// incompatableWithByVersion

#[cfg(test)]
mod tests {
    use super::*;


    static TEXT: &str = r#"
        <root>
            <li>abc</li>
            <li>123</li>
        </root>"#;
    #[test]
    fn parse_lis() {
        let lis = super::parse_lis(TEXT);
        assert_eq!(lis, vec!["abc", "123"]);
    }

    #[test]
    fn parse_lis_basic() {
         static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<root>[\w\W]*</root>"#).unwrap());
         let lis = super::parse_lis_basic(TEXT, &REGEX);
         assert_eq!(lis.unwrap(), vec!["abc", "123"]);
    }

    #[test]
    fn parse_lis_hashset() {
        let lis = super::parse_lis_hashset(TEXT);
        assert!(lis.contains(&String::from("abc")));
        assert!(lis.contains(&String::from("123")));
    }
}

