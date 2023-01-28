use std::{
    sync::{
        Arc,
        Mutex,
        TryLockError,
    },
    collections::HashMap,
};
use crate::{
    ModMetaData,
    widgets::PathLabel,
};
use eframe::egui::{
    Widget,
    Ui,
    Response,
};

#[derive(Debug, Default, Clone)]
pub struct ModInfo {
    mmd: Arc<Mutex<HashMap<String, ModMetaData>>>,
    selected: Arc<Mutex<Option<String>>>,
    last_selected: Option<String>,
    path_lab: Option<PathLabel>,
}

impl ModInfo {
    #[must_use]
    pub fn new(mmd: Arc<Mutex<HashMap<String, ModMetaData>>>, selected: Arc<Mutex<Option<String>>>) -> Self {
        Self {
            mmd,
            selected,
            last_selected: None,
            path_lab: None,
        }
    }

    fn render(ui: &mut Ui, selected: &String, mmd: &ModMetaData, last_selected: &mut Option<String>, path_lab: &mut Option<PathLabel>) -> Response {
        if last_selected.as_ref() != Some(selected) {
            *last_selected = Some(selected.clone());
            *path_lab = Some(PathLabel(mmd.path.clone().unwrap_or_default()));
        }
        if path_lab.is_none() {
            *path_lab = Some(PathLabel(mmd.path.clone().unwrap_or_default()));
        }

        let name = mmd.name.clone().unwrap_or_default();
        let description = mmd.description.clone().unwrap_or_default();

        let mut authors: Vec<String> = Vec::new();
        if let Some(author) = mmd.author.as_ref() {
            authors.push(author.clone());
        }
        if let Some(auths) = mmd.authors.as_ref() {
            authors.append(&mut auths.clone());
        }
        let authors = authors.join(", ");

        // TODO: put these in nicely lol
        ui.scope(|ui| {
            ui.label(name);
            ui.label(authors);
            ui.add(path_lab.as_ref().unwrap());
            ui.label(description);
        }).response
    }
}

impl Widget for &mut ModInfo {
    fn ui(self, ui: &mut Ui) -> Response {
        let sel = self.selected.try_lock();
        if let Ok(Some(sel)) = sel.as_deref() {
            let map = self.mmd.try_lock();
            match map.as_ref().map(|map| map.get(sel)) {
                Ok(Some(mmd)) => return ModInfo::render(ui, sel, mmd, &mut self.last_selected, &mut self.path_lab),
                Ok(None) => log::warn!("No ModMetaData found for {sel}"),
                Err(TryLockError::Poisoned(_)) => log::error!("Couldn't get lock for ModMetaData map: mutex poisoned"),
                Err(TryLockError::WouldBlock) => log::warn!("Couldn't get lock for ModMetaData map: already taken."),
            }
        } else if let Err(TryLockError::Poisoned(_)) = sel.as_deref() {
            log::error!("Couldn't get lock for Selected mod: mutex poisoned");
        }

        ui.scope(|_|{}).response
    }
}

