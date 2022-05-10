// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::types::Note;

pub fn execute(db: &mut crate::db::Database, backend: super::Backend) -> crate::Result {
    execute_with(db, backend, None)
}

pub fn execute_with(
    db: &mut crate::db::Database,
    backend: super::Backend,
    options: Option<Note>,
) -> crate::Result {
    let mut note = match options {
        Some(n) => n,
        None => {
            eprintln!("No note to display!");
            return Ok(());
        }
    };
    // note.append_content("\n**Let's test out the Markdown rendering!**\n\n```rust\nfn main() {\n\tprintln!(\"Hello, world!\");\n}\n```\n\n*Italic text too!*");

    let text = format!(
        r#"# {}

---

{}

---
*Tags: {}*
Created: {} | Updated: {}"#,
        note.title(),
        note.content(),
        note.tags().join(", "),
        note.created(),
        note.updated_humanized(),
    );

    let formatted = termimad::term_text(&text);

    println!("{}", formatted);

    db.ensure_sync_v2(&mut note);

    Ok(())
}

pub struct ViewNoteComponent;

impl super::Component for ViewNoteComponent {
    type Output = ();
    type Options = Option<Note>;

    fn execute_with(
        db: &mut crate::db::Database,
        backend: super::Backend,
        options: Self::Options,
    ) -> crate::Result<Self::Output> {
        execute_with(db, backend, options)
    }
}
