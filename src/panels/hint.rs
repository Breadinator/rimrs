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
    pub fn get_hint(&self) -> Option<String> {
        self.hint.lock().ok().map(|s| s.clone())
    }

    /// Returns a clone of the contents in hint.
    /// Calls [`Mutex::TryLock`], so will fail if lock already taken.
    pub fn try_get_hint(&self) -> Option<String> {
        self.hint.try_lock().ok().map(|s| s.clone())
    }

    /// Returns true if it could get the lock.
    /// This calls [`Mutex::Lock`] so should only be false if there's a [`std::sync::PoisonError`].
    pub fn set_hint(&self, new_value: String) -> bool {
        self.hint.lock().map(|mut hint| *hint = new_value).is_ok()
    }

    /// Returns true if it could get the lock.
    /// This calls [`Mutex::TryLock`], so will fail if the lock is already taken.
    pub fn try_set_hint(&self, new_value: String) -> bool {
        self.hint.try_lock().map(|mut hint| *hint = new_value).is_ok()
    }
}

impl Widget for &HintPanel {
    fn ui(self, ui: &mut Ui) -> Response {
        let text = self.try_get_hint().unwrap_or_default();
        ui.label(&text)
    }
}

