use minidom::Element;

#[derive(Clone, Debug, Default)]
#[allow(non_snake_case)]
pub struct Dependency {
    pub packageId: Option<String>,
    pub displayName: Option<String>,
    pub downloadUrl: Option<String>,
    pub steamWorkshopUrl: Option<String>,
}

impl From<&Element> for Dependency {
    fn from(elem: &Element) -> Self {
        let mut dep = Dependency::default();

        for child in elem.children() {
            match child.name() {
                "packageId" => dep.packageId = Some(child.text()),
                "displayName" => dep.displayName = Some(child.text()),
                "steamWorkshopUrl" => dep.steamWorkshopUrl = Some(child.text()),
                "downloadUrl" => dep.downloadUrl = Some(child.text()),
                _ => {},
            }
        }

        dep
    }
}

