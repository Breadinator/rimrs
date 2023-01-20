use std::path::PathBuf;
use rimrs::ModMetaData;

#[test]
fn parse_mod_meta_data() {
    let path = PathBuf::from(r#"D:\Program Files\steam\steamapps\workshop\content\294100\2842502659\About\About.xml"#);
    let mmd = ModMetaData::read(path).unwrap();

    assert_eq!(mmd.name.unwrap(), "Vanilla Psycasts Expanded");
    assert_eq!(mmd.author.unwrap(), "erdelf, Oskar Potocki, legodude17, Taranchuk, xrushha, Sarg Bjornson, Sir Van, Reann Shepard");
    assert!(mmd.supportedVersions.unwrap().contains(&String::from("1.3")));
}

