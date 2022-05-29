// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod api;
mod error;
mod note;
mod note_dto;
mod reminder;
mod taglist;
mod time;
mod traits;

pub use error::*;
pub use note::Note;
pub use note_dto::{CreateNote, DeleteNote, NoteDto, UpdateNote};
pub use reminder::Reminder;
pub use taglist::TagList;
pub use traits::HasId;

pub type Action<T> = Box<dyn Fn(&T)>;
pub type Mapping<T, R> = Box<dyn Fn(&T) -> R>;
pub type Predicate<T> = Box<dyn Fn(&T) -> bool>;
pub type Mutation<T> = Box<dyn FnMut(&mut T)>;
