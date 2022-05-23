// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::path::PathBuf;

use tinyid::TinyId;

use crate::types::{CreateNote, Note};

use super::BackendError;

pub enum ToFrontend {
    RefreshNoteList { notes: Vec<Note> },
    Error { error_msg: String },
    NoteCreated { note: Note },
    DatabaseLoaded { notes: Vec<Note> },
    DatabaseClosed,
}

pub enum ToBackend {
    UpdateNote { note: Note },
    CreateNote { dto: CreateNote },
    DeleteNote { id: TinyId },
    SaveData,
    Startup,
    Shutdown,
    CreateDatabase { path: PathBuf },
    OpenDatabase { path: PathBuf },
    CloseDatabase,
}
