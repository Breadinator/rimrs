use eframe::egui::{
    Widget,
    Ui,
    Response,
};
use std::sync::Mutex;

#[derive(Debug, Default)]
pub struct HintPanel {
    hint: Mutex<String>,
}

impl HintPanel {
    #[must_use]
    pub fn new(hint: String) -> Self {
        Self {
            hint: Mutex::new(hint),
        }
    }

    /// Returns a clone of the contents in hint.
    /// Calls [`Mutex::Lock`], so will block until acquired.
    #[must_use]
    pub fn get_hint_cloned(&self) -> Option<String> {
        self.hint.lock().ok().map(|s| s.clone())
    }

    /// Returns a clone of the contents in hint.
    /// Calls [`Mutex::TryLock`], so will fail if lock already taken.
    #[must_use]
    pub fn try_get_hint_cloned(&self) -> Option<String> {
        self.hint.try_lock().ok().map(|s| s.clone())
    }

    /// Returns true if it could get the lock.
    /// This calls [`Mutex::Lock`] so should only be false if there's a [`std::sync::PoisonError`].
    ///
    /// # Errors
    /// If it the [`Mutex`] is poisoned, it'll return [`Err`] wrapping the given string.
    pub fn set_hint(&self, new_value: String) -> Result<(), String> {
        match self.hint.lock() {
            Ok(mut guard) => *guard = new_value,
            Err(_) => return Err(new_value),
        }
        Ok(())
    }

    /// Returns true if it could get the lock.
    /// This calls [`Mutex::TryLock`], so will fail if the lock is already taken.
    ///
    /// # Errors
    /// If the [`Mutex`] is poisoned or already held,
    /// it'll return [`Err`] wrapping the given string.
    pub fn try_set_hint(&self, new_value: String) -> Result<(), String> {
        match self.hint.try_lock() {
            Ok(mut guard) => *guard = new_value,
            Err(_) => return Err(new_value),
        }
        Ok(())
    }
}

impl Widget for &HintPanel {
    fn ui(self, ui: &mut Ui) -> Response {
        let text = self.try_get_hint_cloned().unwrap_or_default();
        ui.label(&text)
    }
}

