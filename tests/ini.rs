use rimrs::serialization::ini::*;
use std::io::Read;

#[test]
fn empty() {
    let text = "".as_bytes();
    let mut reader = INIReader::from(Box::new(text) as Box<dyn Read>);
    assert!(reader.next().is_none());
}

#[test]
fn min_nonempty() {
    let text = "a = b".as_bytes();
    let mut reader = INIReader::from(Box::new(text) as Box<dyn Read>);

    let first = reader.next().unwrap().unwrap();
    assert!(first.section.is_none());
    assert_eq!(first.key, "a");
    assert_eq!(first.value, "b");

    assert!(reader.next().is_none());
}

/// Assumes the file exists lol
/// `cargo test from_file --test ini -- --nocapture`
#[test]
fn from_file() {
    let mut path = rimrs::helpers::config::get_config_dir().unwrap();
    path.push("config.ini");
    let text = std::fs::read(path).unwrap();

    let reader = INIReader::from(Box::new(text.as_slice()) as Box<dyn Read>);
    let mut some_lines = false;
    for line in reader {
        println!("{line:?}");
        assert!(line.is_ok());
        some_lines = true;
    }
    assert!(some_lines);
}

#[test]
fn open_config_ini() {
    let reader = INIReader::from_rimpy_config_ini().unwrap();
    let mut some_lines = false;
    for line in reader {
        println!("{line:?}");
        assert!(line.is_ok());
        some_lines = true;
    }
    assert!(some_lines);
}
