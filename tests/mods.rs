use std::path::PathBuf;
use rimrs::{
    ModMetaData,
    Dependency,
};

#[test]
fn parse_mod_meta_data() {
    let path = PathBuf::from(r#"D:\Program Files\steam\steamapps\workshop\content\294100\2842502659\About\About.xml"#);
    let mmd = ModMetaData::read(path).unwrap();

    assert_eq!(mmd.name.unwrap(), "Vanilla Psycasts Expanded");
    assert_eq!(mmd.author.unwrap(), "erdelf, Oskar Potocki, legodude17, Taranchuk, xrushha, Sarg Bjornson, Sir Van, Reann Shepard");
    assert!(mmd.supportedVersions.unwrap().contains(&String::from("1.3")));
}

#[test]
fn parse_dependency() {
    let dep_str = r#"<modDependency>
        <packageId>abc</packageId>
        <displayName>A.B.C.</displayName>
        <downloadUrl>https://foo.bar</downloadUrl>
        <steamWorkshopUrl>steam://idkwhatavalidurllookslike</steamWorkshopUrl>"#;
    let dep = Dependency::from(dep_str);
    assert_eq!(&dep.packageId.unwrap(), "abc");
    assert_eq!(&dep.displayName.unwrap(), "A.B.C.");
    assert_eq!(&dep.downloadUrl.unwrap(), "https://foo.bar");
    assert_eq!(&dep.steamWorkshopUrl.unwrap(), "steam://idkwhatavalidurllookslike");
}

