// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Deserialize, serde::Serialize,
)]
pub enum MenuOptions {
    CreateNote,
    ViewNote,
    ListNotes,
    UpdateNote,
    ViewTags,
    DeleteNote,
    Exit,
}

impl MenuOptions {
    pub fn all() -> &'static [Self] {
        &[
            Self::CreateNote,
            Self::ViewNote,
            Self::ListNotes,
            Self::UpdateNote,
            Self::ViewTags,
            Self::DeleteNote,
            Self::Exit,
        ]
    }
}

impl From<MenuOptions> for usize {
    fn from(opt: MenuOptions) -> Self {
        match opt {
            MenuOptions::CreateNote => 0,
            MenuOptions::ViewNote => 1,
            MenuOptions::ListNotes => 2,
            MenuOptions::UpdateNote => 3,
            MenuOptions::ViewTags => 4,
            MenuOptions::DeleteNote => 5,
            MenuOptions::Exit => 6,
        }
    }
}

impl From<usize> for MenuOptions {
    fn from(i: usize) -> Self {
        match i {
            0 => MenuOptions::CreateNote,
            1 => MenuOptions::ViewNote,
            2 => MenuOptions::ListNotes,
            3 => MenuOptions::UpdateNote,
            4 => MenuOptions::ViewTags,
            5 => MenuOptions::DeleteNote,
            _ => MenuOptions::Exit,
        }
    }
}

impl AsRef<str> for MenuOptions {
    fn as_ref(&self) -> &str {
        match self {
            MenuOptions::CreateNote => "Create Note",
            MenuOptions::ViewNote => "View Note",
            MenuOptions::ListNotes => "List Notes",
            MenuOptions::UpdateNote => "Update Note",
            MenuOptions::ViewTags => "View Tags",
            MenuOptions::DeleteNote => "Delete Note",
            MenuOptions::Exit => "Exit",
        }
    }
}

impl std::fmt::Display for MenuOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

pub fn execute(backend: super::Backend) -> crate::Result<MenuOptions> {
    backend.select("Operations:", MenuOptions::all())
}

pub struct MenuComponent;

impl super::Component for MenuComponent {
    type Output = MenuOptions;

    fn execute_with(
        db: &mut crate::db::Database,
        backend: super::Backend,
        _options: super::NoOptions,
    ) -> crate::Result<Self::Output> {
        self::execute(backend)
    }
}
