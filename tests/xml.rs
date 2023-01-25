use rimrs::serialization::{
    about::*,
    mods_config::*,
};
use std::{
    env,
    fs,
    path::PathBuf,
};

/// Assumes Vanilla Psycasts Expanded installed via steam, and workshop files installed to `D:/` drive.
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

#[test]
fn parse_mods_config_from_path() {
    let appdata = env::var("APPDATA").unwrap();
    let mut path = PathBuf::from(appdata);
    path.pop();
    path.push("LocalLow");
    path.push("Ludeon Studios");
    path.push("RimWorld by Ludeon Studios");
    path.push("Config");
    path.push("ModsConfig.xml");

    let mods_config = ModsConfig::try_from(path.as_path()).unwrap();
    assert!(mods_config.version.is_some());
    assert!(!mods_config.activeMods.is_empty());
    assert!(!mods_config.knownExpansions.is_empty());
}

#[test]
fn parse_mods_config_from_bytes() {
    // Literally just my file as of writing this with most of the mods removed
    let bytes: &[u8] = r#"
<?xml version="1.0" encoding="utf-8"?>
<ModsConfigData>
    <version>1.4.3613 rev641</version>
    <activeMods>
        <li>brrainz.harmony</li>
        <li>me.samboycoding.betterloading.dev</li>
        <li>ludeon.rimworld</li>
        <li>ludeon.rimworld.royalty</li>
        <li>ludeon.rimworld.ideology</li>
        <li>ludeon.rimworld.biotech</li>
        <li>vanillaexpanded.backgrounds</li>
        <li>unlimitedhugs.hugslib</li>
        <li>brrainz.achtung</li>
        <li>unlimitedhugs.allowtool</li>
    </activeMods>
    <knownExpansions>
        <li>ludeon.rimworld</li>
        <li>ludeon.rimworld.royalty</li>
        <li>ludeon.rimworld.ideology</li>
        <li>ludeon.rimworld.biotech</li>
    </knownExpansions>
</ModsConfigData>
    "#.as_bytes();
    let mods_config = ModsConfig::try_from(bytes).unwrap();

    assert_eq!(mods_config.version.unwrap(), "1.4.3613 rev641");

    assert_eq!(mods_config.activeMods.len(), 10);
    assert_eq!(mods_config.activeMods, vec![
        "brrainz.harmony",
        "me.samboycoding.betterloading.dev",
        "ludeon.rimworld",
        "ludeon.rimworld.royalty",
        "ludeon.rimworld.ideology",
        "ludeon.rimworld.biotech",
        "vanillaexpanded.backgrounds",
        "unlimitedhugs.hugslib",
        "brrainz.achtung",
        "unlimitedhugs.allowtool",
    ]);

    assert_eq!(mods_config.knownExpansions.len(), 4);
    assert_eq!(mods_config.knownExpansions, vec![
       "ludeon.rimworld",
       "ludeon.rimworld.royalty",
       "ludeon.rimworld.ideology",
       "ludeon.rimworld.biotech",
    ]);
}

#[test]
fn serialize_mods_config() {
    let mods_config = ModsConfig {
        version: Some(String::from("1.4.3613 rev641")),
        activeMods: vec![
            String::from("brrainz.harmony"),
            String::from("me.samboycoding.betterloading.dev"),
            String::from("ludeon.rimworld"),
            String::from("ludeon.rimworld.royalty"),
            String::from("ludeon.rimworld.ideology"),
            String::from("ludeon.rimworld.biotech"),
            String::from("vanillaexpanded.backgrounds"),
            String::from("unlimitedhugs.hugslib"),
            String::from("brrainz.achtung"),
            String::from("unlimitedhugs.allowtool"),
        ],
        knownExpansions: vec![
           String::from("ludeon.rimworld"),
           String::from("ludeon.rimworld.royalty"),
           String::from("ludeon.rimworld.ideology"),
           String::from("ludeon.rimworld.biotech"),
        ],
    };
    let serialized = String::from(mods_config);

    assert_eq!(serialized, r#"<?xml version="1.0" encoding="utf-8"?>
<ModsConfigData>
    <version>1.4.3613 rev641</version>
    <activeMods>
        <li>brrainz.harmony</li>
        <li>me.samboycoding.betterloading.dev</li>
        <li>ludeon.rimworld</li>
        <li>ludeon.rimworld.royalty</li>
        <li>ludeon.rimworld.ideology</li>
        <li>ludeon.rimworld.biotech</li>
        <li>vanillaexpanded.backgrounds</li>
        <li>unlimitedhugs.hugslib</li>
        <li>brrainz.achtung</li>
        <li>unlimitedhugs.allowtool</li>
    </activeMods>
    <knownExpansions>
        <li>ludeon.rimworld</li>
        <li>ludeon.rimworld.royalty</li>
        <li>ludeon.rimworld.ideology</li>
        <li>ludeon.rimworld.biotech</li>
    </knownExpansions>
</ModsConfigData>"#);
}

#[test]
fn deserialize_serialize_mods_config() {
     let data = r#"<?xml version="1.0" encoding="utf-8"?>
<ModsConfigData>
    <version>1.4.3613 rev641</version>
    <activeMods>
        <li>brrainz.harmony</li>
        <li>me.samboycoding.betterloading.dev</li>
        <li>ludeon.rimworld</li>
        <li>ludeon.rimworld.royalty</li>
        <li>ludeon.rimworld.ideology</li>
        <li>ludeon.rimworld.biotech</li>
        <li>vanillaexpanded.backgrounds</li>
        <li>unlimitedhugs.hugslib</li>
        <li>brrainz.achtung</li>
        <li>unlimitedhugs.allowtool</li>
    </activeMods>
    <knownExpansions>
        <li>ludeon.rimworld</li>
        <li>ludeon.rimworld.royalty</li>
        <li>ludeon.rimworld.ideology</li>
        <li>ludeon.rimworld.biotech</li>
    </knownExpansions>
</ModsConfigData>"#;

     let deserialized = ModsConfig::try_from(data.as_bytes()).unwrap();
     let reserialized = String::from(deserialized);

     assert_eq!(data, reserialized);
}

