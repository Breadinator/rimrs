use rimrs::ModList;
use std::path::PathBuf;

#[test]
fn load_steam_mods() {
    let paths: Vec<PathBuf> = vec![PathBuf::from(r#"D:\Program Files\steam\steamapps\workshop\content\294100"#)];
    let mods = ModList::from_dirs(paths).unwrap();
    assert_ne!(mods.mods.len(), 0);
    assert_eq!(mods.mods.get("UnlimitedHugs.AllowTool").unwrap().author, Some(String::from("UnlimitedHugs")));

    let la = mods.mods.get("Mlie.TabSorting").unwrap().loadAfter.clone().unwrap();
    assert!(la.contains("brrainz.harmony"));
    assert!(la.contains("Mlie.RemoveIndustrialStuff"));
    assert!(la.contains("Mlie.RemoveSpacerStuff"));
    assert!(la.contains("Mlie.LordoftheRimsTheThirdAge"));
    assert_eq!(la.len(), 4);
}

