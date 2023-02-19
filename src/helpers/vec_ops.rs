use crate::{
    helpers::{Side, VecMutAccessor},
    traits::{LockIgnorePoisoned, MoverMatcher, VecMoveError},
};
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum RunError {
    #[error("given index out of bounds")]
    IndexOutOfBounds,
    #[error("couldn't grow Vec: {0}")]
    TryReserveError(#[from] std::collections::TryReserveError),
    #[error("predicate didn't match any items")]
    NotFound,
    #[error("{0}")]
    VecMoveError(#[from] VecMoveError),
}
pub type RunResult = Result<(), RunError>;

pub enum VecOp<'a, T> {
    /// Swaps items at given indices.
    ///
    /// # Errors
    /// [`RunError::IndexOutOfBounds`] if either index not in the given `Vec`.
    Swap(usize, usize),

    /// Pushes an item to the given `Vec`.
    ///
    /// # Errors
    /// [`RunError::TryReserveError`] if it can't make enough space in the given `Vec`.
    Push(T),

    /// Removes item at given index.
    ///
    /// # Errors
    /// [`RunError::IndexOutOfBounds`] if given index not in given `Vec`.
    Remove(usize),

    ForEachMut(Box<dyn Fn(&mut T) + 'a>),

    MoveUp(Box<crate::traits::MoverPredicate<'a, T>>),
    MoveDown(Box<crate::traits::MoverPredicate<'a, T>>),
}

impl<T: std::fmt::Debug> std::fmt::Debug for VecOp<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "VecOps::{}({})",
            match self {
                Self::Swap(_, _) => "Swap",
                Self::Push(_) => "Push",
                Self::Remove(_) => "Remove",
                Self::ForEachMut(_) => "ForEachMut",
                Self::MoveUp(_) => "MoveUp",
                Self::MoveDown(_) => "MoveDown",
            },
            match self {
                Self::Swap(a, b) => format!("{a}, {b}"),
                Self::Push(item) => format!("{item:?}"),
                Self::Remove(index) => index.to_string(),
                Self::ForEachMut(_) => String::from("Fn(&mut T)"),
                Self::MoveUp(_) | Self::MoveDown(_) => String::from("Fn(&T) -> bool"),
            }
        ))
    }
}

impl<'a, T> VecOp<'a, T> {
    /// Runs the operation.
    ///
    /// # Errors
    /// See [`VecOps`] variant documentation.
    pub fn run(self, mut vec: VecMutAccessor<'_, T>) -> RunResult {
        match self {
            Self::Swap(a, b) => Self::swap(vec, a, b),
            Self::Push(item) => Self::push(vec, item),
            Self::Remove(index) => Self::remove(vec, index),
            Self::ForEachMut(operation) => {
                Self::for_each_mut(vec, &operation);
                Ok(())
            }
            Self::MoveUp(predicate) => vec.move_match_up(predicate).map_err(Into::into),
            Self::MoveDown(predicate) => vec.move_match_down(predicate).map_err(Into::into),
        }
    }

    fn swap(mut vec: VecMutAccessor<'_, T>, a: usize, b: usize) -> RunResult {
        if a >= vec.len() || b >= vec.len() {
            return Err(RunError::IndexOutOfBounds);
        }

        vec.swap(a, b);
        Ok(())
    }

    fn push(mut vec: VecMutAccessor<'_, T>, item: T) -> RunResult {
        vec.try_reserve(1).map_err(RunError::TryReserveError)?;
        vec.push(item);
        Ok(())
    }

    fn remove(mut vec: VecMutAccessor<'_, T>, index: usize) -> RunResult {
        if index >= vec.len() {
            return Err(RunError::IndexOutOfBounds);
        }

        vec.remove(index);
        Ok(())
    }

    fn for_each_mut(vec: VecMutAccessor<'_, T>, operation: &dyn Fn(&mut T)) {
        match vec {
            VecMutAccessor::ExclRef(vec) => vec.iter_mut().for_each(operation),
            VecMutAccessor::ArcMutex(armu) => {
                armu.lock_ignore_poisoned().iter_mut().for_each(operation);
            }
        }
    }
}

pub enum MultiVecOp<'a, T> {
    /// Performs a single operation on a given side.
    ///
    /// # Errors
    /// See [`VecOps`] variant documentation.
    SingleOp(Side, VecOp<'a, T>),

    /// Moves item matching given predicate from given side to other side.
    ///
    /// # Errors
    /// * [`RunError::NotFound`] if no items match predicate.
    /// * [`RunError::TryReserveError`] if it can't make space for the item in the other side.
    MoveFrom(Side, Box<dyn Fn(&T) -> bool + 'a>),

    /// Moves item from either side matching predicate to other side.
    /// Searches left side first.
    ///
    /// # Errors
    /// * [`RunError::NotFound`] if no items match predicate.
    /// * [`RunError::TryReserveError`] if it can't make space for the item in the other side.
    Swap(Box<dyn Fn(&'_ T) -> bool + 'a>),

    /// Applies the given operation on every element of both `Vec`s.
    /// Always returns `Ok`.
    ForEachMut(Box<dyn Fn(&mut T)>),

    MoveUp(crate::traits::MoverPredicate<'a, T>),
    MoveDown(crate::traits::MoverPredicate<'a, T>),
}

impl<'a, T> MultiVecOp<'a, T> {
    /// Runs the operation.
    ///
    /// # Errors
    /// See [`MultiVecOp`] variant documentation.
    pub fn run(self, left: VecMutAccessor<'_, T>, right: VecMutAccessor<'_, T>) -> RunResult {
        match self {
            Self::SingleOp(Side::Left, op) => op.run(left),
            Self::SingleOp(Side::Right, op) => op.run(right),
            Self::MoveFrom(Side::Left, predicate) => Self::move_from(left, right, predicate),
            Self::MoveFrom(Side::Right, predicate) => Self::move_from(right, left, predicate),
            Self::Swap(predicate) => Self::swap(left, right, predicate),
            Self::ForEachMut(operation) => {
                Self::for_each_mut(left, right, &operation);
                Ok(())
            }
            Self::MoveUp(predicate) => Self::move_up(left, right, &predicate),
            Self::MoveDown(predicate) => Self::move_down(left, right, &predicate),
        }
    }

    fn move_from(
        mut from: VecMutAccessor<'_, T>,
        to: VecMutAccessor<'_, T>,
        predicate: Box<dyn Fn(&T) -> bool + 'a>,
    ) -> RunResult {
        let index = from.position(predicate).ok_or(RunError::NotFound)?;
        let item = from.remove(index);

        VecOp::Push(item).run(to)
    }

    fn swap(
        mut left: VecMutAccessor<'_, T>,
        mut right: VecMutAccessor<'_, T>,
        predicate: Box<dyn Fn(&T) -> bool + 'a>,
    ) -> RunResult {
        let mut index: Option<usize> = left.position(&predicate);
        let mut from_side = Side::Left;
        if index.is_none() {
            index = right.position(predicate);
            from_side = Side::Right;
        }
        let index: usize = index.ok_or(RunError::NotFound)?;

        let item = match from_side {
            Side::Left => left.remove(index),
            Side::Right => right.remove(index),
        };
        VecOp::Push(item).run(match from_side {
            Side::Left => right,
            Side::Right => left,
        })
    }

    fn for_each_mut(
        left: VecMutAccessor<'_, T>,
        right: VecMutAccessor<'_, T>,
        operation: &dyn Fn(&mut T),
    ) {
        match left {
            VecMutAccessor::ExclRef(vec) => {
                for item in &mut vec.iter_mut() {
                    operation(item);
                }
            }
            VecMutAccessor::ArcMutex(armu) => {
                for item in armu.lock_ignore_poisoned().iter_mut() {
                    operation(item);
                }
            }
        }

        match right {
            VecMutAccessor::ExclRef(vec) => {
                for item in &mut vec.iter_mut() {
                    operation(item);
                }
            }
            VecMutAccessor::ArcMutex(armu) => {
                for item in armu.lock_ignore_poisoned().iter_mut() {
                    operation(item);
                }
            }
        }
    }

    fn move_up(
        mut left: VecMutAccessor<'_, T>,
        mut right: VecMutAccessor<'_, T>,
        predicate: &crate::traits::MoverPredicate<'a, T>,
    ) -> RunResult {
        match left.move_match_up(Box::new(predicate)) {
            Ok(()) => Ok(()),
            Err(VecMoveError::NoMatch) => {
                right.move_match_up(Box::new(predicate)).map_err(Into::into)
            }
            Err(err) => Err(err.into()),
        }
    }

    fn move_down(
        mut left: VecMutAccessor<'_, T>,
        mut right: VecMutAccessor<'_, T>,
        predicate: &crate::traits::MoverPredicate<'a, T>,
    ) -> RunResult {
        match left.move_match_down(Box::new(predicate)) {
            Ok(()) => Ok(()),
            Err(VecMoveError::NoMatch) => right
                .move_match_down(Box::new(predicate))
                .map_err(Into::into),
            Err(err) => Err(err.into()),
        }
    }
}
