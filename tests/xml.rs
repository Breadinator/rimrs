use rimrs::xml::*;
use std::{
    fs,
    path::PathBuf,
};

#[test]
fn test_parse_about() {
    let path = PathBuf::from(r#"D:\Program Files\steam\steamapps\workshop\content\294100\2842502659\About\About.xml"#);
    let file = fs::read(path).unwrap();
    let mmd = parse_about(&file).unwrap();

    assert_eq!(mmd.name.unwrap(), String::from("Vanilla Psycasts Expanded"));
    assert!(mmd.description.is_some());
    assert_eq!(mmd.packageId.unwrap(), String::from("VanillaExpanded.VPsycastsE"));
    assert_eq!(mmd.author.unwrap(), String::from("erdelf, Oskar Potocki, legodude17, Taranchuk, xrushha, Sarg Bjornson, Sir Van, Reann Shepard"));

    assert!(mmd.loadBefore.unwrap().contains("steve.betterquestrewards"));

    let load_after = mmd.loadAfter.unwrap();
    assert!(load_after.contains("Ludeon.RimWorld"));
    assert!(load_after.contains("OskarPotocki.VanillaFactionsExpanded.Core"));

    let supported_versions = mmd.supportedVersions.unwrap();
    assert!(supported_versions.contains("1.4"));
    assert!(supported_versions.contains("1.3"));

    assert!(!mmd.modDependencies.unwrap().is_empty());
}

