use std::{
    path::PathBuf,
    ffi::OsString,
    sync::{
        Mutex,
        MutexGuard,
    },
};
use thiserror::Error;

pub trait LogIfErr {
    type OkValue;
    fn log_if_err(self) -> Option<Self::OkValue>;
}

impl<T, E: std::fmt::Debug> LogIfErr for Result<T, E> {
    type OkValue = T;
    fn log_if_err(self) -> Option<T> {
        match self {
            Ok(val) => Some(val),
            Err(e) => {
                log::error!("{e:?}");
                None
            }
        }
    }
}

pub trait ToStringOrEmpty {
    fn to_string_or_empty(&self) -> String;
}

impl ToStringOrEmpty for Option<PathBuf> {
    fn to_string_or_empty(&self) -> String {
        self.clone()
            .map(PathBuf::into_os_string)
            .map_or(Ok(String::new()), OsString::into_string)
            .unwrap_or(String::new())
    }
}

pub trait TableRower {
    fn table_row(self, row: egui_extras::TableRow);
}

/* mover traits */

pub type MoverPredicate<'a, T> = Box<dyn Fn(&'_ T) -> bool + 'a>;

#[allow(clippy::missing_errors_doc)]
pub trait Mover {
    type Error;
    fn move_up(self, i: usize) -> Result<(), Self::Error>;
    fn move_down(self, i: usize) -> Result<(), Self::Error>;
    fn move_up_n(self, i: usize, n: usize) -> Result<(), Self::Error>;
    fn move_down_n(self, i: usize, n: usize) -> Result<(), Self::Error>;
}

#[allow(clippy::missing_errors_doc)]
pub trait MoverMatcher : Mover {
    type Item;
    fn move_match_up(self, predicate: MoverPredicate<'_, Self::Item>) -> Result<(), Self::Error>;
    fn move_match_down(self, predicate: MoverPredicate<'_, Self::Item>) -> Result<(), Self::Error>;
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum VecMoveError {
    #[error("index out of bounds")]
    IndexOutOfBounds,
    #[error("no match")]
    NoMatch,
}

impl<T> Mover for &mut Vec<T> {
    type Error = VecMoveError;

    /// Wrapper around [`Vec::swap`] to swap `i` and `i-1`.
    /// Returns [`VecMoveError::IndexOutOfBounds`] instead of panicking.
    fn move_up(self, i: usize) -> Result<(), Self::Error> {
        if i == 0 || i >= self.len() {
            return Err(Self::Error::IndexOutOfBounds);
        }
        self.swap(i, i-1);
        Ok(())
    }

    /// Wrapper around [`Vec::swap`] to swap `i` and `i+1`.
    /// Returns [`VecMoveError::IndexOutOfBounds`] instead of panicking.
    fn move_down(self, i: usize) -> Result<(), Self::Error> {
        if i > self.len() - 2 {
            return Err(Self::Error::IndexOutOfBounds);
        }
        self.swap(i, i+1);
        Ok(())
    }

    /// Performs [`Self::move_up`] `n` times, starting from `i`.
    /// Should perform no operation if it would move out of bounds.
    #[allow(unused_must_use)]
    fn move_up_n(self, i: usize, n: usize) -> Result<(), Self::Error> {
        if i >= self.len() || n > i {
            return Err(Self::Error::IndexOutOfBounds);
        }
        for j in 0..n {
            self.move_up(i-j); // shouldn't fail due to the above check
        }
        Ok(())
    }

    /// Performs [`Self::move_down`] `n` times, starting from `i`.
    /// Should perform no operation if it would move out of bounds.
    #[allow(unused_must_use)]
    fn move_down_n(self, i: usize, n: usize) -> Result<(), Self::Error> {
        if i + n >= self.len() {
            return Err(Self::Error::IndexOutOfBounds);
        }
        for j in 0..n {
            self.move_down(i+j); // shouldn't fail due to the above check
        }
        Ok(())
    }
}

impl<'a, T> MoverMatcher for &'a mut Vec<T>
where
    &'a mut Vec<T>: Mover<Error = VecMoveError>,
{
    type Item = T;

    fn move_match_up(self, predicate: MoverPredicate<'_, Self::Item>) -> Result<(), Self::Error> {
        let i = self.iter().position(predicate).ok_or(Self::Error::NoMatch)?;
        self.move_up(i)
    }

    fn move_match_down(self, predicate: MoverPredicate<'_, Self::Item>) -> Result<(), Self::Error> {
        let i = self.iter().position(predicate).ok_or(Self::Error::NoMatch)?;
        self.move_down(i)
    }
}

#[derive(Error, Debug)]
#[error("TryLockIgnorePoisonedError: WouldBlock")]
pub struct TryLockIgnorePoisonedError;

pub trait LockIgnorePoisoned<T> {
    fn lock_ignore_poisoned(&self) -> MutexGuard<'_, T>;
    fn try_lock_ignore_poisoned(&self) -> Result<MutexGuard<'_, T>, TryLockIgnorePoisonedError>;
}

impl<T> LockIgnorePoisoned<T> for Mutex<T> {
    fn lock_ignore_poisoned(&self) -> MutexGuard<'_, T> {
        self.lock().unwrap_or_else(|psn| psn.into_inner())
    }

    fn try_lock_ignore_poisoned(&self) -> Result<MutexGuard<'_, T>, TryLockIgnorePoisonedError> {
        match self.try_lock() {
            Ok(guard) => Ok(guard),
            Err(std::sync::TryLockError::Poisoned(psn)) => Ok(psn.into_inner()),
            Err(std::sync::TryLockError::WouldBlock) => Err(TryLockIgnorePoisonedError),
        }
    }
}

