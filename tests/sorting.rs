use rand::{prelude::SliceRandom, thread_rng};
use rimrs::{sort, Dependency, ModMetaData, SortError};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[test]
fn empty() {
    let mods: &[String] = &[];
    let mmd = Rc::new(RefCell::new(HashMap::new()));
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
    let mmd = Rc::new(RefCell::new(mmd));

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
    let mmd = Rc::new(RefCell::new(mmd));

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
    let mmd = Rc::new(RefCell::new(mmd));

    let sorted = sort(&mods, &mmd);
    assert_eq!(sorted.unwrap_err(), SortError::CyclicError);
}
