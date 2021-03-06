// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use egui_toast::ToastKind;

use crate::types::Note;

pub enum ToApp {
    CreateNewNote,
    SetActiveNote(Note),
    DeleteNote(Note),
    DeleteActiveNote,
    SaveRequested,
    Toast(ToastKind, String),
    Error(String),
    Debug(String),
}
