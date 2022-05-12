// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::types::CreateNote;

/// TODO: Turn this into a trait that limits access to [`Database`] methods.
pub fn execute(db: &mut crate::db::Database, backend: super::Backend) -> crate::Result {
    crate::flame_guard!("bins", "icli", "parts", "add_note", "execute");

    let title = backend.text("Title:", None)?;
    let content = backend.multiline_text("Content:", None)?;
    let tags = backend.text_array("Tags:")?;

    println!(
        "New Note:\n\tTitle: {}\n\tContent: {}\n\tTags: [{}]",
        title,
        content,
        tags.join(", ")
    );

    let mut dto: CreateNote = (title, content, tags).into();

    if backend.confirm("Are you sure you want to create this note?")? {
        let note = db.apply_create(dto)?;
        println!("Created note:\n{}", note);
    }

    Ok(())
}

struct AddNoteComponent;

impl super::Component for AddNoteComponent {
    fn execute_with(
        db: &mut crate::db::Database,
        backend: super::Backend,
        _options: super::NoOptions,
    ) -> crate::Result<Self::Output> {
        self::execute(db, backend)
    }

    type Output = ();

    type Options = super::NoOptions;
}
