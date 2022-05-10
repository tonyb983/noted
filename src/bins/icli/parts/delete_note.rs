// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub fn execute(db: &mut crate::db::Database, backend: super::Backend) -> crate::Result<()> {
    execute_with(db, backend, super::NoOptions::default())
}

pub fn execute_with(
    db: &mut crate::db::Database,
    backend: super::Backend,
    _options: super::NoOptions,
) -> crate::Result<()> {
    let choice = super::pick_note(db, backend)?;
    println!("Are you sure you want to delete this note?");
    super::view_note_with(db, backend, choice.clone())?;
    if let Some(note) = choice && backend.confirm("Delete note?")? {
        db.apply_delete(note)?;
        println!("Note deleted!");
    }
    Ok(())
}
