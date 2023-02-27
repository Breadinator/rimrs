use rimrs::ModList;
use std::path::PathBuf;

#[test]
fn load_steam_mods() {
    let paths: Vec<PathBuf> = vec![PathBuf::from(
        r#"D:\Program Files\steam\steamapps\workshop\content\294100"#,
    )];
    let mod_list = ModList::from_dirs(paths).unwrap();
    let mods = mod_list.mods.lock().unwrap();
    assert_ne!(mods.len(), 0);

    assert_eq!(
        mods.get("unlimitedhugs.allowtool").unwrap().author,
        Some(String::from("UnlimitedHugs"))
    );

    let la = mods
        .get("mlie.tabsorting")
        .unwrap()
        .loadAfter
        .clone()
        .unwrap();
    assert!(la.contains("brrainz.harmony"));
    assert!(la.contains("mlie.removeindustrialstuff"));
    assert!(la.contains("mlie.removespacerstuff"));
    assert!(la.contains("mlie.lordoftherimsthethirdage"));
    println!("{la:?}");
}
