use rimrs::serialization::rimpy_config::*;

#[test]
fn get_local_mods_location() {
    let conf = RimPyConfig::from_file().unwrap();
    println!("{conf:?}");
    assert!(conf.folders.local_mods.is_some());
}
