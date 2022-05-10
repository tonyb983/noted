// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::types::Note;

mod with_i {}

mod with_d {
    use dialoguer::{theme::ColorfulTheme, Editor, Input};

    pub fn execute(
        db: &mut crate::db::Database,
        backend: super::super::Backend,
        target: &Option<super::Note>,
    ) -> crate::Result<()> {
        let mut note = match target {
            Some(n) => n.clone(),
            None => {
                eprintln!("No note selected!");
                return Ok(());
            }
        };

        let theme = ColorfulTheme::default();
        let title: String = Input::with_theme(&theme)
            .with_initial_text(note.title())
            .with_prompt("Title:")
            .interact()?;

        let content = match Editor::new()
            .require_save(true)
            .trim_newlines(true)
            .edit(note.content())?
        {
            Some(new) => new,
            None => {
                eprintln!("Error editing content.");
                return Ok(());
            }
        };

        note.set_title(title.as_str());
        note.set_content(content.as_str());
        db.ensure_sync(&mut [note]);

        Ok(())
    }
}

pub fn execute(db: &mut crate::db::Database, backend: super::Backend) -> crate::Result<()> {
    execute_with(db, backend, &None)
}

pub fn execute_with(
    db: &mut crate::db::Database,
    backend: super::Backend,
    option: &Option<Note>,
) -> crate::Result<()> {
    match backend {
        super::Backend::Dialoguer => with_d::execute(db, backend, option),
        super::Backend::Inquire => unimplemented!(
            "Inquire backend lacks an `Editor` type, please use Dialoguer backend instead"
        ),
    }
}
