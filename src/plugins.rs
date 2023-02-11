use std::{
    path::Path,
    fs,
};
use libloading::{
    Library,
    Symbol,
    os::windows
};
use crate::{
    widgets::Button,
    traits::LogIfErr,
};
use once_cell::sync::Lazy;

pub static LOADED_PLUGINS: Lazy<LoadedPlugins> = Lazy::new(|| LoadedPlugins::load(std::env::current_dir().unwrap()).unwrap());
type PostButtonGen = unsafe extern "C" fn(*mut [Option<Button>; 10]);

#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct LoadedPlugins(Vec<Plugin>);

#[derive(Debug)]
pub struct Plugin {
    #[allow(dead_code)]
    lib: Library,
    post_button_gen_symbol: Option<windows::Symbol<PostButtonGen>>,
}

impl From<Library> for Plugin {
    fn from(lib: Library) -> Self {
        unsafe {
            let post_button_gen_symbol: Option<Symbol<PostButtonGen>> = lib.get(b"post_button_gen").log_if_err();
            let post_button_gen_symbol = post_button_gen_symbol.map(|sym| sym.into_raw());

            Self {
                lib,
                post_button_gen_symbol,
            }
        }
    }
}

impl LoadedPlugins {
    /// Loads plugins from dynamic libraries in the given directory.
    /// Filters out
    ///
    /// # Errors
    /// Returns an [`Err`] if [`fs::read_dir`] fails.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self(fs::read_dir(path)?
            .filter(|f| f.as_ref().map_or(false, |f| f.path().extension().map_or(false, |s| s.to_string_lossy().ends_with("dll"))))
            .filter_map(|f| f.ok().map(|f| f.path()))
            .map(|path| unsafe{ Library::new(path) })
            .filter_map(Result::ok)
            .map(Into::into)
            .collect()))
    }

    pub fn post_button_gen(&self, buttons: &mut [Option<Button>; 10]) {
        for f in self.0.iter().filter_map(|pl| pl.post_button_gen_symbol.as_ref()) {
            unsafe { f(buttons); }
        }
    }
}

impl Plugin {
    /// Runs the `post_button_gen` function in the plugin.
    /// Returns `Err` if the plugin doesn't have that function.
    pub fn post_button_gen(&self, buttons: &mut [Option<Button>; 10]) {
        if let Some(post_button_gen_symbol) = &self.post_button_gen_symbol {
            unsafe { post_button_gen_symbol(buttons); }
        }
    }
}

