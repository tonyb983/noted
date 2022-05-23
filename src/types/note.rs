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
    types::{CreateNote, DeleteNote, UpdateNote},
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Note {
    id: TinyId,
    title: String,
    content: String,
    tags: Vec<String>,
    created: OffsetDateTime,
    updated: OffsetDateTime,
    #[serde(skip)]
    dirty: bool,
    #[serde(skip)]
    pending_delete: bool,
}

impl Note {
    #[must_use]
    pub fn existing(
        id: TinyId,
        title: String,
        content: String,
        tags: Vec<String>,
        created: OffsetDateTime,
        updated: OffsetDateTime,
    ) -> Self {
        crate::profile_guard!("existing", "types::Note");
        Note {
            id,
            title,
            content,
            tags,
            created,
            updated,
            dirty: false,
            pending_delete: false,
        }
    }

    #[must_use]
    pub fn create(dto: impl Into<CreateNote>) -> Self {
        crate::profile_guard!("create", "types::Note");
        let (title, content, tags) = dto.into().into_parts();
        Self {
            id: TinyId::random(),
            title: title.unwrap_or_default(),
            content: content.unwrap_or_default(),
            tags,
            created: OffsetDateTime::now_utc(),
            updated: OffsetDateTime::now_utc(),
            dirty: true,
            pending_delete: false,
        }
    }

    #[must_use]
    pub fn create_for(db: &crate::db::Database, dto: impl Into<CreateNote>) -> Self {
        crate::profile_guard!("create_for", "types::Note");
        let (title, content, tags) = dto.into().into_parts();
        Self {
            id: db.create_id(),
            title: title.unwrap_or_default(),
            content: content.unwrap_or_default(),
            tags,
            created: OffsetDateTime::now_utc(),
            updated: OffsetDateTime::now_utc(),
            dirty: true,
            pending_delete: false,
        }
    }

    pub fn update(&mut self, dto: impl Into<UpdateNote>) -> bool {
        crate::profile_guard!("update", "types::Note");
        let (id, title, content, tags) = dto.into().into_parts();

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
        if self.dirty {
            self.set_updated_now();
        }
        self.dirty
    }

    pub fn update_from(&mut self, other: &Note) {
        crate::profile_guard!("update_from", "types::Note");
        if self.id != other.id {
            return;
        }

        self.title = other.title.clone();
        self.content = other.content.clone();
        self.tags = other.tags.clone();
        self.created = other.created;
        self.updated = other.updated;
        self.dirty = false;
        self.pending_delete = false;
    }

    pub fn delete(&mut self, dto: impl Into<DeleteNote>) -> bool {
        crate::profile_guard!("delete", "types::Note");
        let id = *dto.into().id();
        if self.id == id {
            self.dirty = true;
            self.pending_delete = true;
        }

        self.pending_delete
    }

    #[must_use]
    pub fn id(&self) -> TinyId {
        crate::profile_guard!("id", "types::Note");
        self.id
    }

    #[must_use]
    pub fn title(&self) -> &str {
        crate::profile_guard!("title", "types::Note");
        &self.title
    }

    pub fn set_title(&mut self, title: &str) {
        crate::profile_guard!("set_title", "types::Note");
        if self.title != title {
            self.title = title.to_string();
            self.set_updated_now();
            self.dirty = true;
        }
    }

    pub fn update_title(&mut self, f: impl FnOnce(&str) -> String) {
        crate::profile_guard!("update_title", "types::Note");
        let new = f(&self.title);
        if new != self.title {
            self.title = new;
            self.set_updated_now();
            self.dirty = true;
        }
    }

    #[must_use]
    pub fn content(&self) -> &str {
        crate::profile_guard!("content", "types::Note");
        &self.content
    }

    pub fn set_content(&mut self, content: &str) {
        crate::profile_guard!("set_content", "types::Note");
        if self.content != content {
            self.content = content.to_string();
            self.set_updated_now();
            self.dirty = true;
        }
    }

    pub fn update_content(&mut self, f: impl FnOnce(&str) -> String) {
        crate::profile_guard!("update_content", "types::Note");
        let new = f(&self.content);
        if new != self.content {
            self.content = new;
            self.set_updated_now();
            self.dirty = true;
        }
    }

    pub fn append_content(&mut self, content: &str) {
        crate::profile_guard!("append_content", "types::Note");
        if !content.is_empty() {
            if !self.content().ends_with(' ') && !content.starts_with(' ') {
                self.content.push(' ');
            }
            self.content.push_str(content);
            self.set_updated_now();
            self.dirty = true;
        }
    }

    #[must_use]
    pub fn tags(&self) -> &[String] {
        crate::profile_guard!("tags", "types::Note");
        &self.tags
    }

    pub fn set_tags(&mut self, mut tags: Vec<String>) {
        crate::profile_guard!("set_tags", "types::Note");
        // tags.sort_unstable();
        // tags.dedup();
        if self.tags != tags {
            self.tags = tags;
            self.set_updated_now();
            self.dirty = true;
        }
    }

    pub fn update_tags(&mut self, f: impl FnOnce(&[String]) -> Vec<String>) {
        crate::profile_guard!("update_tags", "types::Note");
        let new = f(&self.tags);
        self.set_tags(new);
    }

    pub fn add_tag(&mut self, tag: String) {
        crate::profile_guard!("add_tag", "types::Note");
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.set_updated_now();
            self.dirty = true;
        }
    }

    pub fn remove_tag(&mut self, tag: &str) {
        crate::profile_guard!("remove_tag", "types::Note");
        if let Some(index) = self.tags.iter().position(|t| t == tag) {
            self.tags.remove(index);
            self.set_updated_now();
            self.dirty = true;
        }
    }

    #[must_use]
    pub fn tag_len(&self) -> usize {
        crate::profile_guard!("tag_len", "types::Note");
        self.tags.len()
    }

    #[must_use]
    pub fn created(&self) -> &OffsetDateTime {
        crate::profile_guard!("created", "types::Note");
        &self.created
    }

    #[must_use]
    pub fn created_humanized(&self) -> impl std::fmt::Display {
        crate::profile_guard!("created_humanized", "types::Note");
        crate::util::dtf::humanize_timespan_to_now(self.created)
    }

    #[must_use]
    pub fn updated(&self) -> &OffsetDateTime {
        crate::profile_guard!("updated", "types::Note");
        &self.updated
    }

    #[must_use]
    pub fn updated_humanized(&self) -> impl std::fmt::Display {
        crate::profile_guard!("updated_humanized", "types::Note");
        crate::util::dtf::humanize_timespan_to_now(self.updated)
    }

    #[must_use]
    pub fn dirty(&self) -> bool {
        crate::profile_guard!("dirty", "types::Note");
        self.dirty
    }

    pub fn set_dirty(&mut self, dirty: bool) {
        crate::profile_guard!("set_dirty", "types::Note");
        self.dirty = dirty;
    }

    #[must_use]
    pub fn pending_delete(&self) -> bool {
        crate::profile_guard!("pending_delete", "types::Note");
        self.pending_delete
    }

    pub fn set_pending_delete(&mut self, pending_delete: bool) {
        crate::profile_guard!("set_pending_delete", "types::Note");
        self.pending_delete = pending_delete;
    }

    #[must_use]
    pub fn title_contains(&self, text: &str) -> bool {
        crate::profile_guard!("title_contains", "types::Note");
        self.title.contains(text)
    }

    #[must_use]
    pub fn title_matches(&self, text: &str) -> bool {
        crate::profile_guard!("title_matches", "types::Note");
        self.title == text
    }

    #[must_use]
    pub fn content_contains(&self, text: &str) -> bool {
        crate::profile_guard!("content_contains", "types::Note");
        self.content.contains(text)
    }

    #[must_use]
    pub fn content_matches(&self, text: &str) -> bool {
        crate::profile_guard!("content_matches", "types::Note");
        self.content == text
    }

    #[must_use]
    pub fn tag_contains(&self, text: &str) -> bool {
        crate::profile_guard!("tag_contains", "types::Note");
        self.tags.iter().any(|tag| tag.contains(text))
    }

    #[must_use]
    pub fn tag_matches(&self, text: &str) -> bool {
        crate::profile_guard!("tag_matches", "types::Note");
        self.tags.iter().any(|tag| tag == text)
    }

    #[must_use]
    pub fn full_text_search(&self, text: &str) -> bool {
        crate::profile_guard!("full_text_search", "types::Note");
        self.title_contains(text) || self.content_contains(text) || self.tag_contains(text)
    }

    pub(crate) fn clear_flags(&mut self) {
        crate::profile_guard!("clear_flags", "types::Note");
        self.dirty = false;
        self.pending_delete = false;
    }

    pub(crate) fn make_invalid(&mut self) {
        crate::profile_guard!("make_invalid", "types::Note");
        self.id = TinyId::null();
        self.dirty = false;
        self.pending_delete = false;
        self.title = String::new();
        self.content = String::new();
        self.tags = Vec::new();
        self.created = OffsetDateTime::UNIX_EPOCH;
        self.updated = OffsetDateTime::UNIX_EPOCH;
    }

    pub fn touch(&mut self) {
        crate::profile_guard!("touch", "types::Note");
        self.set_updated_now();
        self.set_dirty(true);
    }

    fn set_updated_now(&mut self) {
        crate::profile_guard!("set_updated_now", "types::Note");
        self.updated = OffsetDateTime::now_utc();
    }
}

impl std::fmt::Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let fd = time::macros::format_description!("[weekday], [month repr:short] [day], [year] [hour repr:12]:[minute]:[second][period case:lower]");
        writeln!(f, "ID: {}", self.id)?;
        writeln!(f, "Title: {}", self.title)?;
        writeln!(f, "Content: {}", self.content)?;
        writeln!(f, "Tags: {:?}", self.tags)?;
        writeln!(
            f,
            "Created: {}",
            crate::util::dtf::humanize_timespan_to_now(self.created)
        )?;
        writeln!(
            f,
            "Updated: {}",
            crate::util::dtf::humanize_timespan_to_now(self.updated)
        )?;
        Ok(())
    }
}

impl PartialEq<Note> for Note {
    fn eq(&self, other: &Note) -> bool {
        self.id == other.id
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
