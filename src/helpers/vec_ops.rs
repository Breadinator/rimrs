use super::Side;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum RunError {
    #[error("given index out of bounds")]
    IndexOutOfBounds,
    #[error("couldn't grow Vec: {0}")]
    TryReserveError(#[from] std::collections::TryReserveError),
    #[error("predicate didn't match any items")]
    NotFound,
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
}

impl<T: std::fmt::Debug> std::fmt::Debug for VecOp<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("VecOps::{}({})", match self {
            Self::Swap(_, _) => "Swap",
            Self::Push(_) => "Push",
            Self::Remove(_) => "Remove",
            Self::ForEachMut(_) => "ForEachMut",
        }, match self {
            Self::Swap(a, b) => format!("{a}, {b}"),
            Self::Push(item) => format!("{item:?}"),
            Self::Remove(index) => index.to_string(),
            Self::ForEachMut(_) => String::from("Fn(&mut T)"),
        }))
    }
}

impl<'a, T> VecOp<'a, T> {
    /// Runs the operation.
    ///
    /// # Errors
    /// See [`VecOps`] variant documentation.
    pub fn run(self, vec: &mut Vec<T>) -> RunResult {
        match self {
            Self::Swap(a, b) => Self::swap(vec, a, b),
            Self::Push(item) => Self::push(vec, item),
            Self::Remove(index) => Self::remove(vec, index),
            Self::ForEachMut(operation) => { Self::for_each_mut(vec, &operation); Ok(()) },
        }
    }

    fn swap(vec: &mut Vec<T>, a: usize, b: usize) -> RunResult {
        if a >= vec.len() || b >= vec.len() {
            return Err(RunError::IndexOutOfBounds)
        }

        vec.swap(a, b);
        Ok(())
    }

    fn push(vec: &mut Vec<T>, item: T) -> RunResult {
        vec.try_reserve(1).map_err(RunError::TryReserveError)?;
        vec.push(item);
        Ok(())
    }

    fn remove(vec: &mut Vec<T>, index: usize) -> RunResult {
         if index >= vec.len() {
            return Err(RunError::IndexOutOfBounds)
        }

        vec.remove(index);
        Ok(())
    }

    fn for_each_mut(vec: &mut [T], operation: &dyn Fn(&mut T)) {
        for item in vec {
            operation(item);
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
}

impl<'a, T> MultiVecOp<'a, T> {
    /// Runs the operation.
    ///
    /// # Errors
    /// See [`MultiVecOp`] variant documentation.
    pub fn run(self, left: &mut Vec<T>, right: &mut Vec<T>) -> RunResult {
        match self {
            Self::SingleOp(Side::Left, op) => op.run(left),
            Self::SingleOp(Side::Right, op) => op.run(right),
            Self::MoveFrom(Side::Left, predicate) => Self::move_from(left, right, predicate),
            Self::MoveFrom(Side::Right, predicate) => Self::move_from(right, left, predicate),
            Self::Swap(predicate) => Self::swap(left, right, predicate),
            Self::ForEachMut(operation) => { Self::for_each_mut(left, right, &operation); Ok(()) },
        }
    }

    fn move_from(from: &mut Vec<T>, to: &mut Vec<T>, predicate: Box<dyn Fn(&T) -> bool + 'a>) -> RunResult {
        let index = from.iter().position(predicate).ok_or(RunError::NotFound)?;
        let item = from.remove(index);

        VecOp::Push(item).run(to)
    }

    fn swap(left: &mut Vec<T>, right: &mut Vec<T>, predicate: Box<dyn Fn(&T) -> bool + 'a>) -> RunResult {
        let mut index: Option<usize> = left.iter().position(&predicate);
        let mut from_side = Side::Left;
        if index.is_none() {
            index = right.iter().position(predicate);
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

    fn for_each_mut(left: &mut [T], right: &mut [T], operation: &dyn Fn(&mut T)) {
        for item in left {
            operation(item);
        }
        for item in right {
            operation(item);
        }
    }
}

