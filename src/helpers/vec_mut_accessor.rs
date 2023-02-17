use crate::traits::{LockIgnorePoisoned, Mover, MoverMatcher, MoverPredicate, VecMoveError};
use std::sync::{Arc, Mutex};

pub enum VecMutAccessor<'v, T> {
    ExclRef(&'v mut Vec<T>),
    ArcMutex(Arc<Mutex<Vec<T>>>),
}

impl<'v, T> From<&'v mut Vec<T>> for VecMutAccessor<'v, T> {
    fn from(vec: &'v mut Vec<T>) -> Self {
        Self::ExclRef(vec)
    }
}

impl<'v, T> From<Arc<Mutex<Vec<T>>>> for VecMutAccessor<'v, T> {
    fn from(vec: Arc<Mutex<Vec<T>>>) -> Self {
        Self::ArcMutex(vec)
    }
}

impl<'v, T> VecMutAccessor<'v, T> {
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Self::ExclRef(vec) => vec.len(),
            Self::ArcMutex(armu) => armu.lock_ignore_poisoned().len(),
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn remove(&mut self, index: usize) -> T {
        match self {
            Self::ExclRef(vec) => vec.remove(index),
            Self::ArcMutex(armu) => armu.lock_ignore_poisoned().remove(index),
        }
    }

    #[must_use]
    pub fn position<P: FnMut(&T) -> bool>(&self, predicate: P) -> Option<usize> {
        match self {
            Self::ExclRef(vec) => vec.iter().position(predicate),
            Self::ArcMutex(armu) => armu.lock_ignore_poisoned().iter().position(predicate),
        }
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        match self {
            Self::ExclRef(vec) => vec.swap(a, b),
            Self::ArcMutex(armu) => armu.lock_ignore_poisoned().swap(a, b),
        }
    }

    /// See [`Vec::try_reserve`]
    #[allow(clippy::missing_errors_doc)]
    pub fn try_reserve(
        &mut self,
        additional: usize,
    ) -> Result<(), std::collections::TryReserveError> {
        match self {
            Self::ExclRef(vec) => vec.try_reserve(additional),
            Self::ArcMutex(armu) => armu.lock_ignore_poisoned().try_reserve(additional),
        }
    }

    pub fn push(&mut self, item: T) {
        match self {
            Self::ExclRef(vec) => vec.push(item),
            Self::ArcMutex(armu) => armu.lock_ignore_poisoned().push(item),
        }
    }
}

impl<'v, T> Mover for &mut VecMutAccessor<'v, T> {
    type Error = VecMoveError;

    fn move_up(self, i: usize) -> Result<(), Self::Error> {
        if i == 0 || i >= self.len() {
            return Err(Self::Error::IndexOutOfBounds);
        }
        match self {
            VecMutAccessor::ExclRef(vec) => vec.swap(i, i - 1),
            VecMutAccessor::ArcMutex(armu) => armu.lock_ignore_poisoned().swap(i, i - 1),
        }
        Ok(())
    }

    fn move_down(self, i: usize) -> Result<(), Self::Error> {
        // acquires lock twice, once for len call and tthen in the match below; OOB check could be invalidated
        if i > self.len() - 2 {
            return Err(Self::Error::IndexOutOfBounds);
        }
        match self {
            VecMutAccessor::ExclRef(vec) => vec.swap(i, i + 1),
            VecMutAccessor::ArcMutex(armu) => armu.lock_ignore_poisoned().swap(i, i + 1),
        }
        Ok(())
    }

    fn move_up_n(self, i: usize, n: usize) -> Result<(), Self::Error> {
        if i + n >= self.len() {
            return Err(Self::Error::IndexOutOfBounds);
        }
        for j in 0..n {
            self.move_down(i + j).ok(); // shouldn't fail due to the above check
        }
        Ok(())
    }

    fn move_down_n(self, i: usize, n: usize) -> Result<(), Self::Error> {
        if i + n >= self.len() {
            return Err(Self::Error::IndexOutOfBounds);
        }
        for j in 0..n {
            self.move_down(i + j).ok(); // shouldn't fail due to the above check
        }
        Ok(())
    }
}

impl<'v, 'a, T> MoverMatcher for &'a mut VecMutAccessor<'v, T>
where
    &'a mut VecMutAccessor<'v, T>: Mover<Error = VecMoveError>,
{
    type Item = T;

    fn move_match_up(self, predicate: MoverPredicate<'_, Self::Item>) -> Result<(), Self::Error> {
        let i = self.position(predicate).ok_or(Self::Error::NoMatch)?;
        self.move_up(i)
    }

    fn move_match_down(self, predicate: MoverPredicate<'_, Self::Item>) -> Result<(), Self::Error> {
        let i = self.position(predicate).ok_or(Self::Error::NoMatch)?;
        self.move_down(i)
    }
}
