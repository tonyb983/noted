// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::types::Note;

mod with_d {
    use dialoguer::{theme::ColorfulTheme, Select};

    pub fn execute(db: &mut crate::db::Database) -> crate::Result<Option<super::Note>> {
        let all_notes = db.get_all().to_vec();
        if all_notes.is_empty() {
            println!("There are no notes to display!");
            return Ok(None);
        }
        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Notes:")
            .max_length(10)
            .items(&all_notes)
            .interact_opt()?;

        Ok(choice.and_then(|i| all_notes.get(i).cloned()))
    }
}

mod with_i {
    use inquire::Select;

    use crate::types::Note;

    pub fn execute(db: &mut crate::db::Database) -> crate::Result<Option<super::Note>> {
        let all_notes = db.get_all().to_vec();
        if all_notes.is_empty() {
            println!("There are no notes to display!");
            return Ok(None);
        }
        let mut select = Select::new("Notes:", all_notes).with_page_size(10);
        let choice = select.prompt_skippable()?;
        Ok(choice)
    }
}

pub fn execute(
    db: &mut crate::db::Database,
    backend: super::Backend,
) -> crate::Result<Option<Note>> {
    match backend {
        super::Backend::Dialoguer => with_d::execute(db),
        super::Backend::Inquire => with_i::execute(db),
    }
}
