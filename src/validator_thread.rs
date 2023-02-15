use std::{
    thread::{
        self,
        JoinHandle,
    },
    collections::HashMap,
    sync::{
        Arc,
        Mutex,
        mpsc::{
            Receiver,
            SyncSender,
        },
    },
};
use crate::{
    ModMetaData,
    traits::LogIfErr,
    helpers::{
        ModListValidator,
        ModListValidationResult,
    },
};

#[derive(Debug, Clone)]
pub enum Message {
    Stop,
    NewModMetaData(Arc<Mutex<HashMap<String, ModMetaData>>>),
    Validate(Vec<String>, SyncSender<ModListValidationResult>)
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
fn validate(mmd: &Arc<Mutex<HashMap<String, ModMetaData>>>, mod_list: &[String], tx: SyncSender<ModListValidationResult>) {
    tx.send(ModListValidator::new(mmd).validate(mod_list))
        .log_if_err();
}

