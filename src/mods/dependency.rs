use crate::mods::regex;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[allow(non_snake_case)]
pub struct Dependency {
    pub packageId: Option<String>,
    pub displayName: Option<String>,
    pub downloadUrl: Option<String>,
    pub steamWorkshopUrl: Option<String>,
}

impl From<&str> for Dependency {
    fn from(text: &str) -> Self {
        Self {
            packageId: regex::parse_packageId(text).map(String::from),
            displayName: regex::parse_displayName(text).map(String::from),
            downloadUrl: regex::parse_downloadUrl(text).map(String::from),
            steamWorkshopUrl: regex::parse_steamWorkshopUrl(text).map(String::from),
        }
    }
}

