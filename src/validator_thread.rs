use crate::{
    mods::{ModListValidationResult, ModListValidator},
    traits::LogIfErr,
    ModMetaData,
};
use std::{
    collections::HashMap,
    sync::{
        mpsc::{Receiver, SyncSender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

#[derive(Debug, Clone)]
pub enum Message {
    Stop,
    NewModMetaData(Arc<Mutex<HashMap<String, ModMetaData>>>),
    Validate(Vec<String>, SyncSender<ModListValidationResult>),
}

#[must_use]
pub fn spawn(rx: Receiver<Message>) -> JoinHandle<()> {
    thread::spawn(move || validator_thread_fn(rx))
}

#[allow(clippy::needless_pass_by_value)]
fn validator_thread_fn(rx: Receiver<Message>) {
    let mut mmd: Option<Arc<Mutex<HashMap<String, ModMetaData>>>> = None;

    loop {
        match rx.recv() {
            Ok(Message::Validate(mod_list, tx)) => validate(mmd.as_ref().unwrap(), &mod_list, tx),
            Ok(Message::NewModMetaData(new_mmd)) => mmd = Some(new_mmd),
            Ok(Message::Stop) => break,
            Err(err) => panic!("{err}"),
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn validate(
    mmd: &Arc<Mutex<HashMap<String, ModMetaData>>>,
    mod_list: &[String],
    tx: SyncSender<ModListValidationResult>,
) {
    tx.send(ModListValidator::new(mmd).validate(mod_list))
        .log_if_err();
}
