// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod list;
mod msg;
mod note_editor;
mod panels;
mod screens;
mod simple_prompt;
mod toaster;

pub use list::NoteList;
pub use msg::ToApp;
pub use note_editor::NoteEditor;
pub use simple_prompt::SimplePrompt;
pub use toaster::{Toast, ToastKind, Toaster};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WidgetState {
    Enabled,
    Disabled,
}

impl WidgetState {
    pub fn is_active(self) -> bool {
        self == WidgetState::Enabled
    }

    pub fn is_enabled(self) -> bool {
        self == WidgetState::Enabled
    }
    pub fn is_inactive(self) -> bool {
        self == WidgetState::Disabled
    }

    pub fn is_disabled(self) -> bool {
        self == WidgetState::Disabled
    }
}

impl From<bool> for WidgetState {
    fn from(b: bool) -> Self {
        if b {
            WidgetState::Enabled
        } else {
            WidgetState::Disabled
        }
    }
}
