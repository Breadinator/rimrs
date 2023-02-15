use std::{
    sync::{
        Arc,
        Mutex,
        TryLockError,
        atomic::{
            AtomicUsize,
            Ordering,
        },
    },
    collections::HashMap,
    rc::Rc,
    cell::RefCell,
};
use crate::{
    ModMetaData,
    widgets::PathLabel,
    helpers::fetch_inc_id,
};
use eframe::egui::{
    Widget,
    Ui,
    Response,
    widgets::Label,
    ScrollArea,
};
use egui_extras::{
    TableBuilder,
    Column,
};

/// The info panel to the left of the mods lists that shows more details on a selected mod.
#[derive(Debug)]
pub struct ModInfo {
    mmd: Arc<Mutex<HashMap<String, ModMetaData>>>,
    selected: Rc<RefCell<Option<String>>>,
    last_selected: Option<String>,
    path_lab: Option<PathLabel>,
    id: AtomicUsize,
}

impl ModInfo {
    #[must_use]
    pub fn new(mmd: Arc<Mutex<HashMap<String, ModMetaData>>>, selected: Rc<RefCell<Option<String>>>) -> Self {
        Self {
            mmd,
            selected,
            last_selected: None,
            path_lab: None,
            id: AtomicUsize::new(fetch_inc_id()),
        }
    }

    fn render(
        ui: &mut Ui,
        selected: &str,
        mmd: &ModMetaData,
        last_selected: &mut Option<String>,
        path_lab: &mut Option<PathLabel>,
        id: &AtomicUsize,
    ) -> Response {
        // get data
        *last_selected = Some(String::from(selected));
        *path_lab = Some(PathLabel::new(mmd.path.clone().unwrap_or_default()));
        id.store(fetch_inc_id(), Ordering::Release);

        if path_lab.is_none() {
            *path_lab = Some(PathLabel::new(mmd.path.clone().unwrap_or_default()));
        }

        let name = mmd.name.clone().unwrap_or_default();
        let description = mmd.description.clone();

        let mut authors: Vec<String> = Vec::new();
        if let Some(author) = mmd.author.as_ref() {
            authors.push(author.clone());
        }
        if let Some(auths) = mmd.authors.as_ref() {
            authors.append(&mut auths.clone());
        }
        let authors = authors.join(", ");

        // make widgets
        let name_widget = Label::new(format!("Name: {name}"));
        let authors_widget = Label::new(format!("Authors: {authors}"));
        let path_widget = path_lab.as_ref().unwrap();
        let description_widget = description.map(Label::new);

        // add widgets to ui
        ui.push_id(id.load(Ordering::Acquire), |ui| {
            let w = ui.available_width() / 2.0;
            let desc_height = ui.available_height() - 100.0;

            // name + authors
            TableBuilder::new(ui)
                .column(Column::exact(w))
                .column(Column::remainder())
                .body(|mut body| {
                    body.row(f32::NAN, |mut row| {
                        row.col(|ui| { ui.add(name_widget); });
                        row.col(|ui| { ui.add(authors_widget); });
                    });
                });

            // path
            ui.add(path_widget);

            // desc
            if let Some(description_widget) = description_widget {
                ui.group(|ui| {
                    ScrollArea::vertical()
                        .max_height(desc_height)
                        .show(ui, |ui| ui.add(description_widget));
                });
            }
        }).response
    }
}

impl Widget for &mut ModInfo {
    fn ui(self, ui: &mut Ui) -> Response {
        let sel = self.selected.borrow_mut();
        if let Some(sel) = sel.as_deref() {
            let map = self.mmd.try_lock();
            match map.as_ref().map(|map| map.get(sel)) {
                Ok(Some(mmd)) => return ModInfo::render(ui, sel, mmd, &mut self.last_selected, &mut self.path_lab, &self.id),
                Ok(None) => log::warn!("No ModMetaData found for {sel}"),
                Err(TryLockError::Poisoned(_)) => log::error!("Couldn't get lock for ModMetaData map: mutex poisoned"),
                Err(TryLockError::WouldBlock) => log::warn!("Couldn't get lock for ModMetaData map: already taken."),
            }
        }
        ui.scope(|_|{}).response
    }
}

