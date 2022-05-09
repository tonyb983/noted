// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod with_i {
    use inquire::Text;

    use crate::types::CreateNote;

    pub fn execute(db: &mut crate::db::Database) -> crate::Result<CreateNote> {
        let title = Text::new("Note Title")
            .with_help_message("The title for the new note.")
            .prompt()?;
        let content = Text::new("Note Title")
            .with_help_message("The title for the new note.")
            .prompt()?;

        let tags = {
            let mut ts = Vec::new();
            while let Some(tag) = Text::new("Note Tag(s)")
                .with_help_message("The tags for the new note.")
                .prompt_skippable()?
            {
                if tag.is_empty() {
                    break;
                }
                ts.push(tag);
            }
            ts
        };

        Ok((title, content, tags).into())
    }

    pub fn confirm(msg: &str) -> crate::Result<bool> {
        let opt = inquire::Confirm::new(msg).prompt()?;
        Ok(opt)
    }
}

pub mod with_d {
    use dialoguer::{theme::ColorfulTheme, Input};

    use crate::types::CreateNote;

    pub fn execute(db: &mut crate::db::Database) -> crate::Result<CreateNote> {
        let theme = ColorfulTheme::default();
        let title: String = Input::with_theme(&theme)
            .with_prompt("Note Title")
            .interact_text()?;

        let content: String = Input::with_theme(&theme)
            .with_prompt("Note Content")
            .interact_text()?;

        let tags = {
            let mut ts: Vec<String> = Vec::new();
            while let Ok(tag) = Input::<String>::with_theme(&theme)
                .with_prompt("Note Tag(s)")
                .interact_text()
            {
                if tag.is_empty() {
                    break;
                }
                ts.push(tag);
            }

            ts
        };

        Ok((title, content, tags).into())
    }

    pub fn confirm(msg: &str) -> crate::Result<bool> {
        let opt = dialoguer::Confirm::new().with_prompt(msg).interact()?;
        Ok(opt)
    }
}

/// TODO: Turn this into a trait that limits access to [`Database`] methods.
pub fn execute(db: &mut crate::db::Database, backend: super::Backend) -> crate::Result {
    let mut dto = match backend {
        super::Backend::Dialoguer => with_d::execute(db)?,
        super::Backend::Inquire => with_i::execute(db)?,
    };

    println!(
        "New Note:\n\tTitle: {}\n\tContent: {}\n\tTags: [{}]",
        dto.title().unwrap_or_default(),
        dto.content().unwrap_or_default(),
        dto.tags().join(", ")
    );
    if match backend {
        super::Backend::Dialoguer => with_d::confirm("Are you sure you want to create this note?"),
        super::Backend::Inquire => with_i::confirm("Are you sure you want to create this note?"),
    }? {
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
}
