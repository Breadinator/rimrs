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

