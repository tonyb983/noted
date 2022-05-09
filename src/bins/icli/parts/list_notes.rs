// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use tinyid::TinyId;

use crate::types::Note;

struct CompactNote<'s> {
    pub id: TinyId,
    pub title: &'s str,
}

impl<'s> CompactNote<'s> {
    pub fn new(note: &'s Note) -> Self {
        CompactNote {
            id: note.id(),
            title: note.title(),
        }
    }

    pub fn to_string_single(&self) -> impl std::fmt::Display {
        format!("ID: {} | Title: {}", self.id, self.title)
    }

    pub fn to_string_multiple(&self) -> impl std::fmt::Display {
        format!("ID: {}\nTitle: {}", self.id, self.title)
    }
}

impl std::fmt::Display for CompactNote<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string_single())
    }
}

mod with_d {
    use dialoguer::{theme::ColorfulTheme, Select};

    pub fn execute(db: &mut crate::db::Database) -> crate::Result<Option<super::Note>> {
        let all_notes = db.get_all().to_vec();
        if all_notes.is_empty() {
            println!("There are no notes to display!");
            return Ok(None);
        }
        let compact = all_notes
            .iter()
            .map(super::CompactNote::new)
            .collect::<Vec<_>>();
        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Notes:")
            .max_length(10)
            .items(&compact)
            .interact_opt()?;

        Ok(choice.and_then(|i| all_notes.iter().find(|n| n.id() == compact[i].id).cloned()))
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
        let compact = all_notes
            .iter()
            .map(super::CompactNote::new)
            .collect::<Vec<_>>();
        let mut select = Select::new("Notes:", compact).with_page_size(10);
        let choice = select.prompt_skippable()?;
        Ok(choice.and_then(|cn| all_notes.iter().find(|n| n.id() == cn.id).cloned()))
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

pub struct ListNoteComponent;

impl super::Component for ListNoteComponent {
    type Output = Option<Note>;

    fn execute_with(
        db: &mut crate::db::Database,
        backend: super::Backend,
        _options: super::NoOptions,
    ) -> crate::Result<Self::Output> {
        self::execute(db, backend)
    }
}
