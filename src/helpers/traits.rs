use std::{
    path::PathBuf,
    ffi::OsString,
};

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

