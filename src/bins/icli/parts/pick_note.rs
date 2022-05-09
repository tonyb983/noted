// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use tinyid::TinyId;

use crate::types::Note;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NoteField {
    Id,
    Title,
    Content,
    Tags,
    Created,
    Updated,
}

impl NoteField {
    pub fn get_from(self, note: &Note) -> String {
        match self {
            NoteField::Id => note.id().to_string(),
            NoteField::Title => note.title().to_string(),
            NoteField::Content => note.content().to_string(),
            NoteField::Tags => note.tags().join(", "),
            NoteField::Created => note.created().to_string(),
            NoteField::Updated => note.updated().to_string(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NoteDisplay {
    Field(NoteField),
    Space,
    Tab,
    Char(char),
    Newline,
}

impl NoteDisplay {
    pub fn get_from(self, note: &Note) -> String {
        match self {
            NoteDisplay::Field(field) => field.get_from(note),
            NoteDisplay::Space => " ".to_string(),
            NoteDisplay::Tab => "\t".to_string(),
            NoteDisplay::Char(ch) => ch.to_string(),
            NoteDisplay::Newline => "\n".to_string(),
        }
    }
}

/// This contains a list of [`NoteDisplay`] items which determine how a list
/// of notes should be displayed. For example, the default value is:
/// ```ignore
/// vec![
///     NoteDisplay::Field(NoteField::Id),
///     NoteDisplay::Space,
///     NoteDisplay::Char('|'),
///     NoteDisplay::Space,
///     NoteDisplay::Field(NoteField::Title),
/// ]
/// ```
/// Which would display `NOTE_ID | NOTE_TITLE`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NoteVisibility(pub Vec<NoteDisplay>);

impl NoteVisibility {
    pub fn get_from(&self, note: &Note) -> String {
        self.0.iter().map(|d| d.get_from(note)).collect()
    }
}

impl Default for NoteVisibility {
    fn default() -> Self {
        Self(vec![
            NoteDisplay::Field(NoteField::Id),
            NoteDisplay::Space,
            NoteDisplay::Char('|'),
            NoteDisplay::Space,
            NoteDisplay::Field(NoteField::Title),
        ])
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PickNoteOptions {
    pub page_size: usize,
    pub field_visibility: NoteVisibility,
    pub multiline: bool,
}

impl Default for PickNoteOptions {
    fn default() -> Self {
        Self {
            page_size: 10,
            field_visibility: NoteVisibility::default(),
            multiline: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct PartialNote {
    id: TinyId,
    text: String,
}

impl PartialNote {
    pub fn new(id: TinyId, text: String) -> Self {
        Self { id, text }
    }

    pub fn from_note(note: &Note, visibility: &NoteVisibility) -> Self {
        let mut text = visibility.get_from(note);

        Self {
            id: note.id(),
            text,
        }
    }
}

impl std::fmt::Display for PartialNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

mod with_d {
    use dialoguer::{theme::ColorfulTheme, Select};

    pub fn execute(
        db: &mut crate::db::Database,
        options: &super::PickNoteOptions,
    ) -> crate::Result<Option<super::Note>> {
        let all_notes = db.get_all().to_vec();
        if all_notes.is_empty() {
            println!("There are no notes to display!");
            return Ok(None);
        }
        let compact = all_notes
            .iter()
            .map(|n| super::PartialNote::from_note(n, &options.field_visibility))
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

    pub fn execute(
        db: &mut crate::db::Database,
        options: &super::PickNoteOptions,
    ) -> crate::Result<Option<super::Note>> {
        let all_notes = db.get_all().to_vec();
        if all_notes.is_empty() {
            println!("There are no notes to display!");
            return Ok(None);
        }
        let compact = all_notes
            .iter()
            .map(|n| super::PartialNote::from_note(n, &options.field_visibility))
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
    execute_with(db, backend, PickNoteOptions::default())
}

#[allow(clippy::needless_pass_by_value)]
pub fn execute_with(
    db: &mut crate::db::Database,
    backend: super::Backend,
    options: PickNoteOptions,
) -> crate::Result<Option<Note>> {
    match backend {
        super::Backend::Dialoguer => with_d::execute(db, &options),
        super::Backend::Inquire => with_i::execute(db, &options),
    }
}

pub struct PickNoteComponent;

impl super::Component for PickNoteComponent {
    type Output = Option<Note>;

    type Options = PickNoteOptions;

    fn execute_with(
        db: &mut crate::db::Database,
        backend: super::Backend,
        options: Self::Options,
    ) -> crate::Result<Self::Output> {
        execute_with(db, backend, options)
    }

    fn execute(
        db: &mut crate::db::Database,
        backend: super::Backend,
    ) -> crate::Result<Self::Output> {
        execute(db, backend)
    }
}
