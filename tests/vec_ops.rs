use rimrs::helpers::{
    vec_ops::*,
    Side,
};

/* SINGLE OPS */

#[test]
fn swap() {
    let mut a = vec![1, 2, 3];
    VecOp::Swap(0, 2).run(&mut a).unwrap();
    assert_eq!(a, vec![3, 2, 1]);
}

#[test]
fn swap_out_of_bounds() {
    let mut a = vec![1, 2, 3];
    let err = VecOp::Swap(0, 3).run(&mut a).unwrap_err();
    assert_eq!(err, RunError::IndexOutOfBounds);
}

#[test]
fn push() {
    let mut a = vec![1, 2, 3];
    let op = VecOp::Push(4);
    op.run(&mut a).unwrap();
    assert_eq!(a, vec![1, 2, 3, 4]);
}

#[test]
fn remove() {
    let mut a = vec!["a", "b", "c"];
    VecOp::Remove(1).run(&mut a).unwrap();
    assert_eq!(a, vec!["a", "c"]);
}

#[test]
fn for_each_mut_single() {
    let mut a = vec![4, 1, 13];
    VecOp::ForEachMut(Box::new(|x| *x += 2)).run(&mut a).unwrap();
    assert_eq!(a, vec![6, 3, 15]);
}

/* MULTI OPS */

#[test]
fn single_op_left() {
    let mut a = vec!["a", "b", "c"];
    let mut b = vec!["d", "e"];
    let op = MultiVecOp::SingleOp(Side::Left, VecOp::Remove(1));
    op.run(&mut a, &mut b).unwrap();
    assert_eq!(a, vec!["a", "c"]);
}

#[test]
fn single_op_right() {
    let mut a = vec!["a", "b", "c"];
    let mut b = vec!["d", "e"];
    let op = MultiVecOp::SingleOp(Side::Right, VecOp::Remove(1));
    op.run(&mut a, &mut b).unwrap();
    assert_eq!(b, vec!["d"]);
}

#[test]
fn move_from_left() {
    let mut a = vec!["a", "b", "c"];
    let mut b = vec!["d", "e"];
    MultiVecOp::MoveFrom(Side::Left, Box::new(|s| s == &"b")).run(&mut a, &mut b).unwrap();
    assert_eq!(a, vec!["a", "c"]);
    assert_eq!(b, vec!["d", "e", "b"]);
}

#[test]
fn move_from_right() {
    let mut a = vec!["a", "b", "c"];
    let mut b = vec!["d", "e"];
    MultiVecOp::MoveFrom(Side::Right, Box::new(|s| *s == "e")).run(&mut a, &mut b).unwrap();
    assert_eq!(a, vec!["a", "b", "c", "e"]);
    assert_eq!(b, vec!["d"]);
}

#[test]
fn move_from_failing() {
    let mut a = vec!["a", "b", "c"];
    let mut b = vec!["d", "e"];
    let err = MultiVecOp::MoveFrom(Side::Left, Box::new(|s| s == &"e")).run(&mut a, &mut b).unwrap_err();
    assert_eq!(err, RunError::NotFound);
}

#[test]
fn swap_from_left() {
    let mut a = vec!["a", "b", "c"];
    let mut b = vec!["d", "e"];
    MultiVecOp::Swap(Box::new(|s| *s == "c")).run(&mut a, &mut b).unwrap();
    assert_eq!(a, vec!["a", "b"]);
    assert_eq!(b, vec!["d", "e", "c"]);
}

#[test]
fn swap_from_right() {
    let mut a = vec!["a", "b", "c"];
    let mut b = vec!["d", "e", "g", "h"];
    MultiVecOp::Swap(Box::new(|s| *s == "g")).run(&mut a, &mut b).unwrap();
    assert_eq!(a, vec!["a", "b", "c", "g"]);
    assert_eq!(b, vec!["d", "e", "h"]);
}

#[test]
fn swap_not_found() {
    let mut a = vec!["a", "b", "c"];
    let mut b = vec!["d", "e"];
    let err = MultiVecOp::Swap(Box::new(|s| s == &"lol")).run(&mut a, &mut b).unwrap_err();
    assert_eq!(err, RunError::NotFound);
}

#[test]
fn swap_with_predicate_from_factory() {
    fn matches<'a, T: PartialEq>(val: &'a T) -> Box<dyn Fn(&'_ T) -> bool + 'a> {
        Box::new(move |x: &T| x == val)
    }
    let matches_12 = matches(&12);

    let mut a = vec![5, 32, 23, 9];
    let mut b = vec![9, 7, 12, 19];

    MultiVecOp::Swap(matches_12).run(&mut a, &mut b).unwrap();
    assert_eq!(a, vec![5, 32, 23, 9, 12]);
    assert_eq!(b, vec![9, 7, 19]);
}

#[test]
fn swap_with_fn_predicate() {
    fn matches_12(x: &i32) -> bool {
        *x == 12
    }

    let mut a = vec![5, 32, 23, 9];
    let mut b = vec![9, 7, 12, 19];

    MultiVecOp::Swap(Box::new(matches_12)).run(&mut a, &mut b).unwrap();
    assert_eq!(a, vec![5, 32, 23, 9, 12]);
    assert_eq!(b, vec![9, 7, 19]);
}

#[test]
fn for_each_mut_multi() {
    let mut a = vec![5, 32, 23, 9];
    let mut b = vec![9, 7, 12, 19];

    MultiVecOp::ForEachMut(Box::new(|x| *x *= 2)).run(&mut a, &mut b).unwrap();
    assert_eq!(a, vec![10, 64, 46, 18]);
    assert_eq!(b, vec![18, 14, 24, 38]);
}

