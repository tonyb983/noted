// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod add_note;
pub mod delete_note;
pub mod list_notes;
pub mod list_tags;
pub mod menu;
pub mod pick_note;
pub mod update_note;
pub mod view_note;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NoOptions;

pub trait Component {
    type Output = ();
    type Options: Default = NoOptions;
    fn execute(db: &mut crate::db::Database, backend: Backend) -> crate::Result<Self::Output> {
        Self::execute_with(db, backend, Self::Options::default())
    }

    fn execute_with(
        db: &mut crate::db::Database,
        backend: Backend,
        options: Self::Options,
    ) -> crate::Result<Self::Output>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Backend {
    Dialoguer,
    Inquire,
}

pub use add_note::execute as add_note;
pub use delete_note::execute as delete_note;
pub use list_notes::execute as list_notes;
pub use list_tags::execute as list_tags;
pub use menu::execute as menu;
pub use pick_note::{execute as pick_note, execute_with as pick_note_with};
pub use update_note::execute_with as update_note_with;
pub use view_note::execute_with as view_note_with;
