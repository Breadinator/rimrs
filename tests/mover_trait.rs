use rimrs::helpers::traits::{Mover, MoverMatcher, VecMoveError};

type Result = std::result::Result<(), VecMoveError>;

#[test]
fn basic_working() -> Result {
    let mut v = vec![12, 4, 26, 99, 12];
    v.move_up(2)?;
    assert_eq!(v, vec![12, 26, 4, 99, 12]);
    v.move_down(3)?;
    assert_eq!(v, vec![12, 26, 4, 12, 99]);
    Ok(())
}

#[test]
fn move_up_second_element() -> Result {
    let mut v = vec![12, 4, 26, 99, 12];
    v.move_up(1)?;
    assert_eq!(v, vec![4, 12, 26, 99, 12]);
    Ok(())
}

#[test]
fn move_up_last_element() -> Result {
    let mut v = vec![12, 4, 26, 99, 12];
    v.move_up(4)?;
    assert_eq!(v, vec![12, 4, 26, 12, 99]);
    Ok(())
}

#[test]
fn move_up_first_element() {
    let mut v = vec![12, 4, 26, 99, 12];
    assert!(v.move_up(0).is_err());
    assert_eq!(v, vec![12, 4, 26, 99, 12]);
}

#[test]
fn move_up_last_plus_one() {
    let mut v = vec![12, 4, 26, 99, 12];
    assert!(v.move_up(5).is_err());
    assert_eq!(v, vec![12, 4, 26, 99, 12]);
}

#[test]
fn move_down_penultimate() -> Result {
    let mut v = vec!["a", "b", "c", "d", "e"];
    v.move_down(3)?;
    assert_eq!(v, vec!["a", "b", "c", "e", "d"]);
    Ok(())
}

#[test]
fn move_down_final() {
    let mut v = vec!["a", "b", "c", "d", "e"];
    assert!(v.move_down(4).is_err());
    assert_eq!(v, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn move_up_n_basic() -> Result {
    let mut v = vec!["a", "b", "c", "d", "e"];
    v.move_up_n(4, 2)?;
    assert_eq!(v, vec!["a", "b", "e", "c", "d"]);
    Ok(())
}

#[test]
fn move_down_n_basic() -> Result {
    let mut v = vec!["a", "b", "c", "d", "e"];
    v.move_down_n(1, 2)?;
    assert_eq!(v, vec!["a", "c", "d", "b", "e"]);
    Ok(())
}

#[test]
fn move_down_n_to_final() -> Result {
    let mut v = vec!["a", "b", "c"];
    v.move_down_n(0, 2)?;
    assert_eq!(v, vec!["b", "c", "a"]);
    Ok(())
}

#[test]
fn move_up_n_to_start() -> Result {
    let mut v = vec!["a", "b", "c"];
    v.move_up_n(2, 2)?;
    assert_eq!(v, vec!["c", "a", "b"]);
    Ok(())
}

#[test]
fn move_down_n_into_oob() {
    let mut v = vec!["a", "b"];
    assert!(v.move_down_n(0, 3).is_err());
    assert_eq!(v, vec!["a", "b"]);
}

#[test]
fn move_down_n_from_oob() {
    let mut v = vec!["a", "b"];
    assert!(v.move_down_n(2, 100).is_err());
    assert_eq!(v, vec!["a", "b"]);
}

#[test]
fn move_up_n_into_oob() {
    let mut v = vec!["a", "b", "c", "d"];
    assert!(v.move_up_n(3, 4).is_err());
    assert_eq!(v, vec!["a", "b", "c", "d"]);
}

#[test]
fn move_up_n_from_oob() {
    let mut v = vec!["a", "b", "c"];
    assert!(v.move_down_n(3, 2).is_err());
    assert_eq!(v, vec!["a", "b", "c"]);
}

#[test]
fn move_match_up() -> Result {
    let mut v = vec!["a", "b", "c"];
    v.move_match_up(Box::new(|item| item == &"b"))?;
    assert_eq!(v, vec!["b", "a", "c"]);
    Ok(())
}

#[test]
fn move_match_down() -> Result {
    let mut v = vec!["a", "b", "c"];
    v.move_match_down(Box::new(|item| item == &"b"))?;
    assert_eq!(v, vec!["a", "c", "b"]);
    Ok(())
}

