use rand::{prelude::SliceRandom, thread_rng};
use rimrs::{sort, Dependency, ModList, ModMetaData, ModsConfig, RimPyConfig, SortError};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[test]
fn empty() {
    let mods: &[String] = &[];
    let mmd = Arc::new(Mutex::new(HashMap::new()));
    let sorted = sort(mods, &mmd);
    assert!(sorted.unwrap().is_empty());
}

#[test]
fn no_relations() {
    let mut rng = thread_rng();
    let mut mods = vec![
        String::from("b"),
        String::from("d"),
        String::from("c"),
        String::from("a"),
    ];
    mods.shuffle(&mut rng);

    let mut mmd = HashMap::new();
    mmd.insert(String::from("a"), ModMetaData::default());
    mmd.insert(String::from("b"), ModMetaData::default());
    mmd.insert(String::from("c"), ModMetaData::default());
    mmd.insert(String::from("d"), ModMetaData::default());
    let mmd = Arc::new(Mutex::new(mmd));

    let sorted = sort(&mods, &mmd);
    assert_eq!(
        sorted
            .unwrap()
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        vec!["a", "b", "c", "d"]
    );
}

#[test]
fn basic() {
    let mut rng = thread_rng();
    let mut mods = vec![
        String::from("b"),
        String::from("d"),
        String::from("c"),
        String::from("a"),
    ];
    mods.shuffle(&mut rng);

    let mut mmd = HashMap::new();
    mmd.insert(
        String::from("a"),
        ModMetaData {
            packageId: Some(String::from("a")),
            modDependencies: Some(
                [Dependency {
                    packageId: Some(String::from("c")),
                    ..Default::default()
                }]
                .into_iter()
                .collect(),
            ),
            ..Default::default()
        },
    );
    mmd.insert(
        String::from("b"),
        ModMetaData {
            packageId: Some(String::from("b")),
            loadBefore: Some([String::from("a")].into_iter().collect()),
            loadAfter: Some([String::from("c"), String::from("d")].into_iter().collect()),
            ..Default::default()
        },
    );
    mmd.insert(
        String::from("c"),
        ModMetaData {
            packageId: Some(String::from("c")),
            ..Default::default()
        },
    );
    mmd.insert(
        String::from("d"),
        ModMetaData {
            packageId: Some(String::from("d")),
            loadBefore: Some(
                [String::from("a"), String::from("c"), String::from("b")]
                    .into_iter()
                    .collect(),
            ),
            ..Default::default()
        },
    );
    let mmd = Arc::new(Mutex::new(mmd));

    let sorted = sort(&mods, &mmd);
    assert_eq!(
        sorted
            .unwrap()
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        vec!["d", "c", "b", "a"]
    );
}

#[test]
fn cyclic() {
    let mods = vec![String::from("a"), String::from("b")];
    let mut mmd = HashMap::new();
    mmd.insert(
        String::from("a"),
        ModMetaData {
            packageId: Some(String::from("a")),
            loadAfter: Some([String::from("b")].into_iter().collect()),
            ..Default::default()
        },
    );
    mmd.insert(
        String::from("b"),
        ModMetaData {
            packageId: Some(String::from("b")),
            modDependencies: Some(
                [Dependency {
                    packageId: Some(String::from("a")),
                    ..Default::default()
                }]
                .into_iter()
                .collect(),
            ),
            ..Default::default()
        },
    );
    let mmd = Arc::new(Mutex::new(mmd));

    let sorted = sort(&mods, &mmd);
    assert_eq!(sorted, Err(SortError::CyclicError));
}

/// `cargo test from_active --test sorting -- --nocapture`
#[test]
fn from_active() {
    let rimpy_config = RimPyConfig::from_file().unwrap();
    let mod_list = ModList::try_from(&rimpy_config).unwrap();

    let mut mods_config_path = rimpy_config
        .folders
        .config_folder
        .expect("Game config folder not found in RimPy `config.ini`");
    mods_config_path.push("ModsConfig.xml");
    let mods_config = Arc::from(ModsConfig::try_from(mods_config_path.as_path()).unwrap());

    let mods = &mods_config.activeMods;

    let sorted = sort(mods, &mod_list.mods);

    println!("{:?}", sorted.unwrap());
}
