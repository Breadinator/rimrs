use crate::{
    helpers::{config::get_mod_list_path, paths::path_to_str},
    traits::{LogIfErr, PushChained},
    widgets::{Button, ModListing},
    writer_thread, ModMetaData, ModsConfig, CHANGED_ACTIVE_MODS,
};
use std::{
    cell::RefCell,
    collections::HashMap,
    path::PathBuf,
    process::Command,
    rc::Rc,
    sync::{
        mpsc::{Sender, SyncSender},
        Arc, Mutex,
    },
};

impl<'a> Button<'a> {
    /// Generates the [`Button`] that clears the active mod list.
    #[must_use]
    pub fn clear(hint_tx: SyncSender<String>, change_mod_list_tx: Sender<Vec<String>>) -> Self {
        let action = Box::new(move || {
            change_mod_list_tx
                .send(vec![
                    String::from("ludeon.rimworld"),
                    String::from("ludeon.rimworld.royalty"),
                    String::from("ludeon.rimworld.ideology"),
                    String::from("ludeon.rimworld.biotech"),
                ])
                .log_if_err();
        }) as Box<dyn Fn() + 'a>;
        let hint = "Remove all mods, except Core and DLCs";

        Self::builder("Clear")
            .action(action)
            .hint(hint, hint_tx)
            .build()
    }

    /// Generates the [`Button`] that auto-sorts the active mod list.
    #[must_use]
    pub fn sort(
        hint_tx: SyncSender<String>,
        change_mod_list_tx: Sender<Vec<String>>,
        active_mod_listing_ref: Rc<RefCell<ModListing<'a>>>,
        mod_meta_data: Arc<Mutex<HashMap<String, ModMetaData>>>,
    ) -> Self {
        let action = Box::new(move || {
            let active_mods = Vec::from(&active_mod_listing_ref.borrow().clone());
            if let Some(sorted) = crate::sort(&active_mods, &mod_meta_data).log_if_err() {
                change_mod_list_tx.send(sorted).log_if_err();
            }
        }) as Box<dyn Fn() + 'a>;
        let hint = "Auto-sort mods";

        Self::builder("Sort")
            .action(action)
            .hint(hint, hint_tx)
            .build()
    }

    /// Generates the [`Button`] that saves the active mod list to disk.
    #[must_use]
    pub fn save(
        hint_tx: SyncSender<String>,
        writer_thread_tx: SyncSender<writer_thread::Message>,
        active_mod_listing_ref: Rc<RefCell<ModListing<'a>>>,
    ) -> Self {
        let action = Box::new(move || {
            let active_mods = Vec::from(&active_mod_listing_ref.borrow().clone());
            writer_thread_tx
                .try_send(writer_thread::Message::SetActiveMods(active_mods))
                .and_then(|_| writer_thread_tx.try_send(writer_thread::Message::Save))
                .log_if_err();
        }) as Box<dyn Fn() + 'a>;
        let hint = "Save the mod list to ModsConfig.xml file (applies changes to game mod list)";
        let is_enabled = Box::new(|| CHANGED_ACTIVE_MODS.check()) as Box<dyn Fn() -> bool + 'a>;

        Self::builder("Save")
            .action(action)
            .hint(hint, hint_tx)
            .is_enabled_fn(is_enabled)
            .build()
    }

    /// Generates the [`Button`] that launches the game.
    #[must_use]
    pub fn run(hint_tx: SyncSender<String>, exe_path: PathBuf, args: Option<String>) -> Self {
        let action = Box::new(move || {
            let mut cmd = Command::new(&exe_path);
            if let Some(args) = args.as_ref() {
                cmd.arg(args); // idk if this'll work with more complex args than I use, TODO check
            }
            cmd.spawn().log_if_err();
        }) as Box<dyn Fn() + 'a>;
        let hint = "Run the game";
        let is_enabled = Box::new(|| !CHANGED_ACTIVE_MODS.check()) as Box<dyn Fn() -> bool>;

        Self::builder("Run")
            .action(action)
            .hint(hint, hint_tx)
            .is_enabled_fn(is_enabled)
            .build()
    }

    #[must_use]
    pub fn import_list(
        hint_tx: SyncSender<String>,
        change_mod_list_tx: Sender<Vec<String>>,
    ) -> Self {
        let hint = "Imports mod list from mod list file";
        let action = Box::new(move || {
            let path = get_mod_list_path().log_if_err().map(|p| p.push_chained("")); // need to push empty so it opens in the dir rather than in its parent with the dir name as the input
            let path = path.as_ref().and_then(path_to_str).unwrap_or_default();
            if let Some(parsed) =
                tinyfiledialogs::open_file_dialog("Select mod list", path, Some((&["*.xml"], "")))
                    .and_then(|p| ModsConfig::try_from(PathBuf::from(p).as_path()).log_if_err())
            {
                change_mod_list_tx.send(parsed.activeMods).log_if_err();
            }
        }) as Box<dyn Fn() + 'a>;

        Self::builder("Import list")
            .hint(hint, hint_tx)
            .action(action)
            .build()
    }

    #[must_use]
    pub fn export_list(
        hint_tx: SyncSender<String>,
        writer_thread_tx: SyncSender<writer_thread::Message>,
        active_mod_listing_ref: Rc<RefCell<ModListing<'a>>>,
    ) -> Self {
        let hint = "Exports mod list to file";
        let action = Box::new(move || {
            let mods = active_mod_listing_ref
                .borrow()
                .items
                .iter()
                .map(|item| item.package_id.clone())
                .collect();

            let mod_list_path = get_mod_list_path().log_if_err().map(|p| p.push_chained(""));
            let mod_list_path = mod_list_path
                .as_ref()
                .and_then(path_to_str)
                .unwrap_or_default();

            tinyfiledialogs::save_file_dialog_with_filter(
                "Save file list",
                mod_list_path,
                &["*.xml"],
                "",
            )
            .map(|save_path| {
                writer_thread_tx.try_send(writer_thread::Message::WriteTo(
                    PathBuf::from(save_path),
                    mods,
                ))
            })
            .map(LogIfErr::log_if_err);
        }) as Box<dyn Fn() + 'a>;

        Self::builder("Export list")
            .hint(hint, hint_tx)
            .action(action)
            .build()
    }
}
