// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::path::{Path, PathBuf};

use crossbeam_channel::{Receiver, Sender};
use parking_lot::Once;
use tracing::{debug, error, info};

use super::{ToBackend, ToFrontend};

static LOG_CHANNEL_CLOSED: Once = Once::new();

pub struct Backend {
    db: Option<crate::db::Database>,
    db_path: Option<PathBuf>,
    back_tx: Sender<ToFrontend>,
    front_rx: Receiver<ToBackend>,
    egui_context: egui::Context,
}

impl Backend {
    pub fn new(
        back_tx: Sender<ToFrontend>,
        front_rx: Receiver<ToBackend>,
        egui_context: egui::Context,
    ) -> Self {
        Self {
            db: None,
            db_path: None,
            back_tx,
            front_rx,
            egui_context,
        }
    }

    pub fn init(&mut self) {
        info!("Initializing backend");

        let rt = tokio::runtime::Runtime::new().unwrap();
        debug!("Runtime created");

        rt.block_on(async {
            loop {
                match self.front_rx.recv() {
                    Ok(mut msg) => {
                        match msg {
                            ToBackend::UpdateNote { ref mut note } => {
                                if let Some(db) = &mut self.db {
                                    db.ensure_sync_v2(note);
                                    self.back_tx.send(ToFrontend::RefreshNoteList { notes: db.get_all().to_vec() }).expect("Unable to send message to frontend");
                                } else {
                                    error!("UpdateNote received but no database is open");
                                    self.back_tx
                                        .send(ToFrontend::Error {
                                            error_msg: format!(
                                                "Update Note requested but no database is open. Note: {:?}", note
                                            ),
                                        })
                                        .expect("Unable to send message to frontend");
                                }
                            },
                            ToBackend::CreateNote { ref dto } => {
                                if let Some(db) = &mut self.db {
                                    match db.apply_create(dto.clone()) {
                                        Ok(created) => {
                                            self.back_tx
                                                .send(ToFrontend::NoteCreated { note: created })
                                                .expect("Unable to send message to frontend");
                                            self.back_tx.send(ToFrontend::RefreshNoteList { notes: db.get_all().to_vec() }).expect("Unable to send message to frontend");
                                        },
                                        Err(error) => error!(%error, ?dto, "Error while creating note from dto:"),
                                    };
                                } else {
                                    error!("CreateNote received but no database is open");
                                }
                            },
                            ToBackend::DeleteNote { id } => {
                                if let Some(db) = &mut self.db {
                                    match db.apply_delete(id) {
                                        Ok(deleted) => if deleted {
                                            self.back_tx.send(ToFrontend::RefreshNoteList { notes: db.get_all().to_vec() }).expect("Unable to send message to frontend");
                                        } else {
                                            self.back_tx.send(ToFrontend::Error { error_msg: "Unable to find and delete the given note.".to_string()}).expect("Unable to send message to frontend");
                                        },
                                        Err(err) => self.back_tx.send(ToFrontend::Error { error_msg: format!("Error deleting note with ID {}: {}", id, err)}).expect("Unable to send message to frontend"),
                                    }
                                } else {
                                    error!(%id, "DeleteNote received but no database is open");
                                    self.back_tx.send(ToFrontend::Error { error_msg: "Delete requested but no database is open!".to_string()}).expect("Unable to send message to frontend");
                                }
                            },
                            ToBackend::SaveData => self.save_data(),
                            ToBackend::Startup => {
                                info!("Backend starting up");
                            },
                            ToBackend::Shutdown => {
                                self.save_data();
                                info!("Backend shutting down");
                            },
                            ToBackend::CreateDatabase { path } => self.create_db(path),
                            ToBackend::OpenDatabase { path } => self.open_db(path),
                            ToBackend::CloseDatabase => self.close_db(),
                        }
                        self.egui_context.request_repaint();
                    }
                    Err(error) => {
                         // As the only reason this will error out is if the channel is closed (sender is dropped) a one time log of the error is enough
                       LOG_CHANNEL_CLOSED.call_once(|| {
                           error!(%error, "There was an error when receiving a message from the frontend:");
                       });
                    }
                }
            }
        });
    }

    fn save_data(&mut self) {
        if let Some(path) = &self.db_path {
            if let Some(db) = &mut self.db {
                db.save(path).unwrap();
            }
        }
    }

    fn create_db<P: AsRef<Path>>(&mut self, path: P) {
        let path = path.as_ref();
        self.close_db();
        let db = crate::db::Database::empty();
        info!(?path, "Database opened at path");
        let notes = Vec::new();
        self.db = Some(db);
        self.db_path = Some(path.to_path_buf());
        self.save_data();
        // self.back_tx
        //     .send(ToFrontend::RefreshNoteList { notes })
        //     .expect("Unable to send message to frontend");
        self.back_tx
            .send(ToFrontend::DatabaseLoaded { notes })
            .expect("Unable to send message to frontend");
    }

    fn open_db<P: AsRef<Path>>(&mut self, path: P) {
        let path = path.as_ref();
        self.close_db();
        match crate::db::Database::load(path) {
            Ok(db) => {
                info!(?path, "Database opened at path");
                let notes = db.get_all().to_vec();
                self.db = Some(db);
                self.db_path = Some(path.to_path_buf());
                // self.back_tx
                //     .send(ToFrontend::RefreshNoteList { notes })
                //     .expect("Unable to send message to frontend");
                self.back_tx
                    .send(ToFrontend::DatabaseLoaded { notes })
                    .expect("Unable to send message to frontend");
            }
            Err(error) => {
                error!(%error, ?path, "Error while opening database:");
                self.back_tx
                    .send(ToFrontend::Error {
                        error_msg: format!(
                            "Error loading database at path '{}': {}",
                            path.display(),
                            error
                        ),
                    })
                    .expect("Unable to send message to frontend");
            }
        }
    }

    fn close_db(&mut self) {
        if self.db.is_some() {
            self.save_data();
            self.db = None;
            self.db_path = None;
            // self.back_tx
            //     .send(ToFrontend::RefreshNoteList { notes: Vec::new() })
            //     .expect("Unable to send message to frontend");
            self.back_tx
                .send(ToFrontend::DatabaseClosed)
                .expect("Unable to send message to frontend");
        }
    }
}
