// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod add_note;
pub mod list_notes;
pub mod menu;
pub mod view_note;

pub trait ICliComponent {
    type Output = ();
    fn execute(db: &mut crate::db::Database, backend: Backend) -> crate::Result<Self::Output>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Backend {
    Dialoguer,
    Inquire,
}

pub use add_note::execute as add_note;
pub use list_notes::execute as list_notes;
pub use menu::execute as menu;
pub use view_note::execute as view_note;
