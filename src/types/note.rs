// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tinyid::TinyId;
use uuid::Uuid;

use crate::{
    flame_guard,
    types::{CreateNote, DeleteNote, HasId, Reminder, UpdateNote},
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Note {
    id: TinyId,
    title: String,
    content: String,
    tags: Vec<String>,
    reminders: Vec<Reminder>,
    created: OffsetDateTime,
    updated: OffsetDateTime,
    #[serde(skip)]
    dirty: bool,
    #[serde(skip)]
    pending_delete: bool,
}

impl Note {
    #[tracing::instrument(level = "trace")]
    #[must_use]
    pub fn existing(
        id: TinyId,
        title: String,
        content: String,
        tags: Vec<String>,
        reminders: Vec<Reminder>,
        created: OffsetDateTime,
        updated: OffsetDateTime,
    ) -> Self {
        Note {
            id,
            title,
            content,
            tags,
            reminders,
            created,
            updated,
            dirty: false,
            pending_delete: false,
        }
    }

    #[tracing::instrument(skip(dto), fields(dto.title, dto.content, dto.tags, dto.reminders))]
    #[must_use]
    pub fn create(dto: impl Into<CreateNote>) -> Self {
        let (title, content, tags, reminders) = dto.into().into_parts();
        #[cfg(feature = "trace")]
        {
            tracing::Span::current().record("dto.title", &format!("{:?}", &title).as_str());
            tracing::Span::current().record("dto.content", &format!("{:?}", &content).as_str());
            tracing::Span::current().record("dto.tags", &format!("{:?}", &tags).as_str());
            tracing::Span::current().record("dto.reminders", &format!("{:?}", &reminders).as_str());
        }
        Self {
            id: TinyId::random(),
            title: title.unwrap_or_default(),
            content: content.unwrap_or_default(),
            tags,
            reminders,
            created: OffsetDateTime::now_utc(),
            updated: OffsetDateTime::now_utc(),
            dirty: true,
            pending_delete: false,
        }
    }

    #[tracing::instrument(skip_all, fields(dto.title, dto.content, dto.tags, dto.reminders))]
    #[must_use]
    pub fn create_for(db: &crate::db::Database, dto: impl Into<CreateNote>) -> Self {
        let (title, content, tags, reminders) = dto.into().into_parts();
        #[cfg(feature = "trace")]
        {
            tracing::Span::current().record("dto.title", &format!("{:?}", &title).as_str());
            tracing::Span::current().record("dto.content", &format!("{:?}", &content).as_str());
            tracing::Span::current().record("dto.tags", &format!("{:?}", &tags).as_str());
            tracing::Span::current().record("dto.reminders", &format!("{:?}", &reminders).as_str());
        }
        Self {
            id: db.create_id(),
            title: title.unwrap_or_default(),
            content: content.unwrap_or_default(),
            tags,
            reminders,
            created: OffsetDateTime::now_utc(),
            updated: OffsetDateTime::now_utc(),
            dirty: true,
            pending_delete: false,
        }
    }

    #[tracing::instrument(skip(dto), fields(dto.id, dto.title, dto.content, dto.tags, dto.reminders))]
    pub fn update(&mut self, dto: impl Into<UpdateNote>) -> bool {
        let (id, title, content, tags, reminders) = dto.into().into_parts();

        #[cfg(feature = "trace")]
        {
            tracing::Span::current().record("dto.id", &id.to_string().as_str());
            tracing::Span::current().record("dto.title", &format!("{:?}", &title).as_str());
            tracing::Span::current().record("dto.content", &format!("{:?}", &content).as_str());
            tracing::Span::current().record("dto.tags", &format!("{:?}", &tags).as_str());
            tracing::Span::current().record("dto.reminders", &format!("{:?}", &reminders).as_str());
        }

        if id != self.id {
            return false;
        }

        if let Some(title) = title {
            if self.title != title {
                self.title = title;
                self.dirty = true;
            }
        }
        if let Some(content) = content {
            if self.content != content {
                self.content = content;
                self.dirty = true;
            }
        }
        if let Some(tags) = tags {
            if self.tags != tags {
                self.tags = tags;
                self.dirty = true;
            }
        }
        if let Some(reminders) = reminders {
            if self.reminders != reminders {
                self.reminders = reminders;
                self.dirty = true;
            }
        }
        if self.dirty {
            self.set_updated_now();
        }
        self.dirty
    }

    /// Updates this note to match the given note, **if the IDs match**.
    ///
    /// ### Note that this does **NOT** set the `dirty` flag. This method is intended to be used as an alternative to an `update` DTO.
    #[tracing::instrument(level = "trace")]
    pub(crate) fn update_from(&mut self, other: &Note) {
        if self.id != other.id {
            return;
        }

        self.title = other.title.clone();
        self.content = other.content.clone();
        self.tags = other.tags.clone();
        self.reminders = other.reminders.clone();
        self.created = other.created;
        self.updated = other.updated;
        self.dirty = false;
        self.pending_delete = false;
    }

    #[tracing::instrument(level = "trace", skip(dto), fields(dto))]
    pub fn delete(&mut self, dto: impl Into<DeleteNote>) -> bool {
        let id = *dto.into().id();
        if self.id == id {
            self.dirty = true;
            self.pending_delete = true;
        }

        #[cfg(feature = "trace")]
        {
            tracing::Span::current().record("dto.id", &id.to_string().as_str());
        }

        self.pending_delete
    }

    #[tracing::instrument(level = "trace")]
    #[must_use]
    pub fn id(&self) -> TinyId {
        self.id
    }

    #[tracing::instrument(level = "trace")]
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    #[tracing::instrument(level = "trace")]
    pub fn set_title(&mut self, title: &str) {
        if self.title != title {
            self.title = title.to_string();
            self.set_updated_now();
            self.dirty = true;
        }
    }

    #[tracing::instrument(level = "trace", skip(f))]
    pub fn update_title(&mut self, f: impl FnOnce(&str) -> String) {
        let new = f(&self.title);
        if new != self.title {
            self.title = new;
            self.set_updated_now();
            self.dirty = true;
        }
    }

    #[tracing::instrument(level = "trace")]
    #[must_use]
    pub fn content(&self) -> &str {
        &self.content
    }

    #[tracing::instrument(level = "trace")]
    pub fn set_content(&mut self, content: &str) {
        if self.content != content {
            self.content = content.to_string();
            self.set_updated_now();
            self.dirty = true;
        }
    }

    #[tracing::instrument(level = "trace", skip(f))]
    pub fn update_content(&mut self, f: impl FnOnce(&str) -> String) {
        let new = f(&self.content);
        if new != self.content {
            self.content = new;
            self.set_updated_now();
            self.dirty = true;
        }
    }

    #[tracing::instrument(level = "trace")]
    pub fn append_content(&mut self, content: &str) {
        if !content.is_empty() {
            if !self.content().ends_with(' ') && !content.starts_with(' ') {
                self.content.push(' ');
            }
            self.content.push_str(content);
            self.set_updated_now();
            self.dirty = true;
        }
    }

    #[tracing::instrument(level = "trace")]
    #[must_use]
    pub fn reminders(&self) -> &[Reminder] {
        &self.reminders
    }

    #[tracing::instrument(level = "trace")]
    pub fn set_reminders(&mut self, reminders: Vec<Reminder>) {
        self.reminders = reminders;
        self.set_updated_now();
        self.dirty = true;
    }

    #[tracing::instrument(level = "trace", skip(f))]
    pub fn update_reminders(&mut self, f: impl FnOnce(&[Reminder]) -> Vec<Reminder>) {
        let new = f(&self.reminders);
        self.set_reminders(new);
    }

    #[tracing::instrument(level = "trace")]
    pub fn add_reminder(&mut self, reminder: Reminder) {
        if !self.reminders.contains(&reminder) {
            self.reminders.push(reminder);
            self.set_updated_now();
            self.dirty = true;
        }
    }

    #[tracing::instrument(level = "trace")]
    pub fn remove_reminder(&mut self, reminder: &Reminder) {
        self.remove_reminder_with_id(reminder.id());
    }

    #[tracing::instrument(level = "trace")]
    pub fn remove_reminder_with_id(&mut self, id: TinyId) {
        if let Some(index) = self.reminders.iter().position(|r| r.id() == id) {
            self.reminders.remove(index);
            self.set_updated_now();
            self.dirty = true;
        }
    }

    #[tracing::instrument(level = "trace")]
    #[must_use]
    pub fn tags(&self) -> &[String] {
        &self.tags
    }

    #[tracing::instrument(level = "trace")]
    pub fn set_tags(&mut self, mut tags: Vec<String>) {
        // tags.sort_unstable();
        // tags.dedup();
        if self.tags != tags {
            self.tags = tags;
            self.set_updated_now();
            self.dirty = true;
        }
    }

    #[tracing::instrument(level = "trace", skip(f))]
    pub fn update_tags(&mut self, f: impl FnOnce(&[String]) -> Vec<String>) {
        let new = f(&self.tags);
        self.set_tags(new);
    }

    #[tracing::instrument(level = "trace")]
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.set_updated_now();
            self.dirty = true;
        }
    }

    #[tracing::instrument(level = "trace")]
    pub fn remove_tag(&mut self, tag: &str) {
        if let Some(index) = self.tags.iter().position(|t| t == tag) {
            self.tags.remove(index);
            self.set_updated_now();
            self.dirty = true;
        }
    }

    #[tracing::instrument(level = "trace")]
    #[must_use]
    pub fn tag_len(&self) -> usize {
        self.tags.len()
    }

    #[tracing::instrument(level = "trace")]
    #[must_use]
    pub fn created(&self) -> &OffsetDateTime {
        &self.created
    }

    #[tracing::instrument(level = "trace")]
    #[must_use]
    pub fn created_humanized(&self) -> impl std::fmt::Display {
        crate::util::dtf::humanize_timespan_to_now(self.created)
    }

    #[tracing::instrument(level = "trace")]
    #[must_use]
    pub fn updated(&self) -> &OffsetDateTime {
        &self.updated
    }

    #[tracing::instrument(level = "trace")]
    #[must_use]
    pub fn updated_humanized(&self) -> impl std::fmt::Display {
        crate::util::dtf::humanize_timespan_to_now(self.updated)
    }

    #[tracing::instrument(level = "trace")]
    #[must_use]
    pub fn dirty(&self) -> bool {
        self.dirty
    }

    #[tracing::instrument(level = "trace")]
    pub fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    #[tracing::instrument(level = "trace")]
    #[must_use]
    pub fn pending_delete(&self) -> bool {
        self.pending_delete
    }

    #[tracing::instrument(level = "trace")]
    pub fn set_pending_delete(&mut self, pending_delete: bool) {
        self.pending_delete = pending_delete;
    }

    #[tracing::instrument(level = "trace")]
    #[must_use]
    pub fn title_contains(&self, text: &str) -> bool {
        self.title.contains(text)
    }

    #[tracing::instrument(level = "trace")]
    #[must_use]
    pub fn title_matches(&self, text: &str) -> bool {
        self.title == text
    }

    #[tracing::instrument(level = "trace")]
    #[must_use]
    pub fn content_contains(&self, text: &str) -> bool {
        self.content.contains(text)
    }

    #[tracing::instrument(level = "trace")]
    #[must_use]
    pub fn content_matches(&self, text: &str) -> bool {
        self.content == text
    }

    #[tracing::instrument(level = "trace")]
    #[must_use]
    pub fn tag_contains(&self, text: &str) -> bool {
        self.tags.iter().any(|tag| tag.contains(text))
    }

    #[tracing::instrument(level = "trace")]
    #[must_use]
    pub fn tag_matches(&self, text: &str) -> bool {
        self.tags.iter().any(|tag| tag == text)
    }

    #[tracing::instrument(level = "trace")]
    #[must_use]
    pub fn full_text_search(&self, text: &str) -> bool {
        self.title_contains(text) || self.content_contains(text) || self.tag_contains(text)
    }

    #[tracing::instrument(level = "trace")]
    pub fn remove_empty_tags(&mut self) {
        let before = self.tags.len();
        self.tags.retain(|r| !r.is_empty());
        if before != self.tags.len() {
            self.set_updated_now();
            self.dirty = true;
        }
    }

    #[tracing::instrument(level = "trace")]
    pub fn remove_cleared_reminders(&mut self) {
        let before = self.reminders.len();
        self.reminders.retain(|r| !r.is_null());
        if before != self.reminders.len() {
            self.set_updated_now();
            self.dirty = true;
        }
    }

    #[must_use]
    pub fn ids_used(&self) -> Vec<TinyId> {
        let mut ids = if self.id.is_null() {
            vec![]
        } else {
            vec![self.id]
        };

        for reminder in &self.reminders {
            let id = reminder.id();
            if !id.is_null() {
                ids.push(id);
            }
        }

        ids
    }

    #[tracing::instrument(level = "trace")]
    pub(crate) fn clear_flags(&mut self) {
        self.dirty = false;
        self.pending_delete = false;
    }

    #[tracing::instrument(level = "trace")]
    pub(crate) fn make_invalid(&mut self) {
        self.id = TinyId::null();
        self.dirty = false;
        self.pending_delete = false;
        self.title = String::new();
        self.content = String::new();
        self.tags = Vec::new();
        self.reminders = Vec::new();
        self.created = OffsetDateTime::UNIX_EPOCH;
        self.updated = OffsetDateTime::UNIX_EPOCH;
    }

    #[tracing::instrument(level = "trace")]
    pub fn touch(&mut self) {
        self.set_updated_now();
        self.set_dirty(true);
    }

    fn set_updated_now(&mut self) {
        self.updated = OffsetDateTime::now_utc();
    }
}

impl std::fmt::Display for Note {
    #[tracing::instrument(level = "trace", skip(f))]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let fd = time::macros::format_description!("[weekday], [month repr:short] [day], [year] [hour repr:12]:[minute]:[second][period case:lower]");
        writeln!(f, "ID: {}", self.id)?;
        writeln!(f, "Title: {}", self.title)?;
        writeln!(f, "Content: {}", self.content)?;
        writeln!(f, "Tags: {:?}", self.tags)?;
        writeln!(f, "Created: {}", self.created_humanized())?;
        writeln!(f, "Updated: {}", self.updated_humanized())?;
        Ok(())
    }
}

impl PartialEq<Self> for Note {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialEq<&Self> for Note {
    fn eq(&self, other: &&Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd<Self> for Note {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id().partial_cmp(&other.id())
    }
}

impl PartialOrd<&Self> for Note {
    fn partial_cmp(&self, other: &&Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id())
    }
}

impl std::hash::Hash for Note {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl HasId for Note {
    fn id(&self) -> TinyId {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[no_coverage]
    fn make_one_note() -> Note {
        Note::create(("title", "content", vec!["tag1", "tag2", "tag3"]))
    }

    #[no_coverage]
    fn make_four_notes() -> Vec<Note> {
        vec![
            Note::create(("title1", "content1", vec!["tag1", "tag2", "something"])),
            Note::create((
                "title2",
                "content2",
                vec![
                    "tag1",
                    "tag2",
                    "tag3",
                    "tag4",
                    "tag5",
                    "another",
                    "something",
                ],
            )),
            Note::create((
                "title3",
                "content3",
                vec!["tag10", "tag20", "tag3", "tag40", "tag500", "ass", "hole"],
            )),
            Note::create((
                "title4",
                "content4",
                vec!["tag", "tags", "tagz", "tagzz", "tag3", "yes", "sir"],
            )),
        ]
    }

    #[no_coverage]
    fn big_tag_list_note() -> Note {
        let mut note = Note::create(("Title", "This is some content."));
        for i in 0..100 {
            note.add_tag(format!("tag{}", i));
        }
        note
    }

    #[test]
    #[no_coverage]
    fn mutations() {
        let mut note = make_one_note();
        note.clear_flags();
        assert_eq!(note.title, "title");
        assert_eq!(note.content, "content");
        assert_eq!(
            note.tags,
            vec!["tag1".to_string(), "tag2".to_string(), "tag3".to_string()]
        );
        assert!(!note.dirty());

        let updated = *note.updated();
        note.set_title("new title");
        assert_eq!(note.title, "new title");
        assert!(note.dirty());
        note.clear_flags();
        assert!(!note.dirty());
        assert!(updated < *note.updated());

        let updated = *note.updated();
        note.update_title(str::to_uppercase);
        assert_eq!(note.title, "NEW TITLE");
        assert!(note.dirty());
        note.clear_flags();
        assert!(!note.dirty());
        assert!(updated < *note.updated());

        let updated = *note.updated();
        note.set_content("new content");
        assert_eq!(note.content, "new content");
        assert!(note.dirty());
        note.clear_flags();
        assert!(!note.dirty());
        assert!(updated < *note.updated());

        let updated = *note.updated();
        note.update_content(str::to_uppercase);
        assert_eq!(note.content, "NEW CONTENT");
        assert!(note.dirty());
        note.clear_flags();
        assert!(!note.dirty());
        assert!(updated < *note.updated());
    }
}
