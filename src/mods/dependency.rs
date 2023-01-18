#[derive(Clone, Debug, Default)]
#[allow(non_snake_case)]
pub struct Dependency {
    pub packageId: Option<String>,
    pub displayName: Option<String>,
    pub downloadUrl: Option<String>,
    pub steamWorkshopUrl: Option<String>,
}

