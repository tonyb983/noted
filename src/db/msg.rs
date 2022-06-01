// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Database Messages
//!
//! This is planned to be the foundation of the undo/redo system with regards to the database. Each change in database state
//! should be representable as a `DatabaseMessage`. From there we can code implementations that are able to perform and reverse
//! each change.

use serde::{Deserialize, Serialize};

use crate::types::{CreateNote, DeleteNote, Note, UpdateNote};

#[derive(Clone, Debug, PartialEq, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DatabaseMessage {
    DataSaved { path: String },
    DataLoaded { path: String },
    NoteCreated { dto: CreateNote, created: Note },
    NoteUpdated { before: Note, after: Note },
    NoteDeleted { deleted: Note },
    Error { msg: String },
}
