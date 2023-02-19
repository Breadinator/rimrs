use rimrs::{Dependency, ModList, ModListValidator, ModMetaData, ModsConfig, RimPyConfig};
use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

fn generate_mod_meta_data() -> Arc<Mutex<HashMap<String, ModMetaData>>> {
    let mut mmd = HashMap::new();

    mmd.insert(
        String::from("a"),
        ModMetaData {
            loadBefore: Some(HashSet::from_iter(vec![String::from("c")])),
            ..Default::default()
        },
    );
    mmd.insert(
        String::from("b"),
        ModMetaData {
            modDependencies: Some(HashSet::from_iter(vec![Dependency {
                packageId: Some(String::from("a")),
                ..Default::default()
            }])),
            ..Default::default()
        },
    );
    mmd.insert(String::from("c"), ModMetaData::default());
    mmd.insert(
        String::from("d"),
        ModMetaData {
            loadAfter: Some(HashSet::from_iter(vec![String::from("c")])),
            ..Default::default()
        },
    );
    mmd.insert(
        String::from("e"),
        ModMetaData {
            incompatibleWith: Some(HashSet::from_iter(vec![String::from("b")])),
            ..Default::default()
        },
    );

    Arc::new(Mutex::new(mmd))
}

macro_rules! validate {
    ($x:expr) => {{
        let mmd = generate_mod_meta_data();
        let validator = ModListValidator::from(&mmd);
        validator.validate($x)
    }};
}

#[test]
fn empty() {
    let res = validate!(&[]);
    assert!(res.is_ok());
}

#[test]
fn no_mod_metadata() {
    let res = validate!(&[String::from("z")]);
    assert!(res.is_warn());
    assert_eq!(
        res.warnings().unwrap(),
        &vec![String::from("Couldn't find metadata for z")]
    );
}

#[test]
fn all_in_order() {
    let res = validate!(&[
        String::from("a"),
        String::from("b"),
        String::from("c"),
        String::from("d")
    ]);
    assert!(res.is_ok());
}

#[test]
fn hard_dependency_missing() {
    let res = validate!(&[String::from("b"), String::from("c")]);
    assert!(res.is_err());
    assert_eq!(res.errors().unwrap(), &vec![String::from("b requires a")]);
}

#[test]
fn load_after_wrong_order() {
    let res = validate!(&[String::from("d"), String::from("c")]);
    assert!(res.is_warn());
    assert_eq!(
        res.warnings().unwrap(),
        &vec![String::from("d should be loaded after c")]
    );
}

#[test]
fn load_before_wrong_order() {
    let res = validate!(&[String::from("c"), String::from("a")]);
    assert!(res.is_warn());
    assert_eq!(
        res.warnings().unwrap(),
        &vec![String::from("a should be loaded before c")]
    );
}

#[test]
fn incompatible() {
    let res = validate!(&[String::from("e"), String::from("b")]);
    assert!(res.is_err());
    assert_eq!(
        res.errors().unwrap(),
        &vec![
            String::from("b requires a"),
            String::from("e is incompatible with b")
        ]
    );
}

/// Reads mod list from disk then validates it.
///
/// `cargo test full_test --test mod_list_validation -- --ignored --nocapture`
#[test]
#[ignore]
fn full_test() {
    let rimpy_config = RimPyConfig::from_file().unwrap();
    let mod_list = ModList::try_from(&rimpy_config).unwrap();
    let mmd = &mod_list.mods;

    let mut mods_config_path = rimpy_config.folders.config_folder.unwrap();
    mods_config_path.push("ModsConfig.xml");
    let mods_config = ModsConfig::try_from(mods_config_path.as_path()).unwrap();
    let active_mods = &mods_config.activeMods;

    let res = ModListValidator::from(mmd).validate(active_mods);

    println!("res: {res:?}");
}
