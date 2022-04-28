// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use rayon::iter::IntoParallelRefIterator;
use serde::{Deserialize, Serialize};
use sqlx::{
    sqlite::{SqliteColumn, SqliteRow},
    Row,
};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    types::{CreateNote, DeleteNote, UpdateNote},
    ShortId,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Note {
    id: ShortId,
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
        id: ShortId,
        title: String,
        content: String,
        tags: Vec<String>,
        created: OffsetDateTime,
        updated: OffsetDateTime,
    ) -> Self {
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
        let (title, content, tags) = dto.into().into_parts();
        Self {
            id: ShortId::random(),
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
        let (title, content, tags) = dto.into().into_parts();
        Self {
            id: db.create_id_force(),
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
        let id = *dto.into().id();
        if self.id == id {
            self.dirty = true;
            self.pending_delete = true;
        }

        self.pending_delete
    }

    #[must_use]
    pub fn id(&self) -> ShortId {
        self.id
    }

    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn set_title(&mut self, title: &str) {
        if self.title != title {
            self.title = title.to_string();
            self.set_updated_now();
            self.dirty = true;
        }
    }

    pub fn update_title(&mut self, f: impl FnOnce(&str) -> String) {
        let new = f(&self.title);
        if new != self.title {
            self.title = new;
            self.set_updated_now();
            self.dirty = true;
        }
    }

    #[must_use]
    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn set_content(&mut self, content: &str) {
        if self.content != content {
            self.content = content.to_string();
            self.set_updated_now();
            self.dirty = true;
        }
    }

    pub fn update_content(&mut self, f: impl FnOnce(&str) -> String) {
        let new = f(&self.content);
        if new != self.content {
            self.content = new;
            self.set_updated_now();
            self.dirty = true;
        }
    }

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

    #[must_use]
    pub fn tags(&self) -> &[String] {
        &self.tags
    }

    pub fn set_tags(&mut self, tags: Vec<String>) {
        if self.tags != tags {
            self.tags = tags;
            self.set_updated_now();
            self.dirty = true;
        }
    }

    pub fn update_tags(&mut self, f: impl FnOnce(&[String]) -> Vec<String>) {
        let new = f(&self.tags);
        self.set_tags(new);
    }

    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.set_updated_now();
            self.dirty = true;
        }
    }

    pub fn remove_tag(&mut self, tag: &str) {
        if let Some(index) = self.tags.iter().position(|t| t == tag) {
            self.tags.remove(index);
            self.set_updated_now();
            self.dirty = true;
        }
    }

    #[must_use]
    pub fn created(&self) -> &OffsetDateTime {
        &self.created
    }

    #[must_use]
    pub fn updated(&self) -> &OffsetDateTime {
        &self.updated
    }

    #[must_use]
    pub fn dirty(&self) -> bool {
        self.dirty
    }

    pub fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    #[must_use]
    pub fn pending_delete(&self) -> bool {
        self.pending_delete
    }

    pub fn set_pending_delete(&mut self, pending_delete: bool) {
        self.pending_delete = pending_delete;
    }

    #[must_use]
    pub fn title_contains(&self, text: &str) -> bool {
        self.title.contains(text)
    }

    #[must_use]
    pub fn title_matches(&self, text: &str) -> bool {
        self.title == text
    }

    #[must_use]
    pub fn content_contains(&self, text: &str) -> bool {
        self.content.contains(text)
    }

    #[must_use]
    pub fn content_matches(&self, text: &str) -> bool {
        self.content == text
    }

    #[must_use]
    pub fn tag_contains(&self, text: &str) -> bool {
        self.tags.iter().any(|tag| tag.contains(text))
    }

    #[must_use]
    pub fn tag_matches(&self, text: &str) -> bool {
        self.tags.iter().any(|tag| tag == text)
    }

    #[must_use]
    pub fn tag_contains_mt(&self, text: &str) -> bool {
        use rayon::iter::ParallelIterator;
        self.tags.par_iter().any(|tag| tag.contains(text))
    }

    #[must_use]
    pub fn full_text_search(&self, text: &str) -> bool {
        self.title_contains(text) || self.content_contains(text) || self.tag_contains(text)
    }

    pub(crate) fn clear_flags(&mut self) {
        self.dirty = false;
        self.pending_delete = false;
    }

    pub(crate) fn make_invalid(&mut self) {
        self.id = ShortId::null();
        self.dirty = false;
        self.pending_delete = false;
        self.title = String::new();
        self.content = String::new();
        self.tags = Vec::new();
        self.created = OffsetDateTime::UNIX_EPOCH;
        self.updated = OffsetDateTime::UNIX_EPOCH;
    }

    fn set_updated_now(&mut self) {
        self.updated = OffsetDateTime::now_utc();
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

    fn big_tag_list_note() -> Note {
        let mut note = Note::create(("Title", "This is some content."));
        for i in 0..100 {
            note.add_tag(format!("tag{}", i));
        }
        note
    }

    #[allow(clippy::similar_names)]
    #[test]
    fn tag_contains_compare() {
        use std::time::{Duration, Instant};
        let total_now = Instant::now();

        let note = big_tag_list_note();

        let now = Instant::now();
        let st_has_10 = note.tag_contains("tag10");
        let st_early_elapsed = now.elapsed();

        let now = Instant::now();
        let st_has_99 = note.tag_contains("tag99");
        let st_last_elapsed = now.elapsed();

        let now = Instant::now();
        let st_has_100 = note.tag_contains("tag100");
        let st_none_elapsed = now.elapsed();

        let now = Instant::now();
        let mt_has_10 = note.tag_contains_mt("tag10");
        let mt_early_elapsed = now.elapsed();

        let now = Instant::now();
        let mt_has_99 = note.tag_contains_mt("tag99");
        let mt_last_elapsed = now.elapsed();

        let now = Instant::now();
        let mt_has_100 = note.tag_contains_mt("tag100");
        let mt_none_elapsed = now.elapsed();

        let total_elapsed = total_now.elapsed();
        println!("Completed in {:?} total.", total_elapsed);
        println!("Results comparing tag_contains and tag_contains_mt.");
        println!("Single Threaded:");
        println!(
            "\tHas tag10  is {}. Took {:?}.",
            st_has_10, st_early_elapsed
        );
        println!("\tHas tag99  is {}. Took {:?}.", st_has_99, st_last_elapsed);
        println!(
            "\tHas tag100 is {}. Took {:?}.",
            st_has_100, st_none_elapsed
        );
        println!();

        println!("Multi Threaded:");
        println!(
            "\tHas tag10  is {}. Took {:?}.",
            mt_has_10, mt_early_elapsed
        );
        println!("\tHas tag99  is {}. Took {:?}.", mt_has_99, mt_last_elapsed);
        println!(
            "\tHas tag100 is {}. Took {:?}.",
            mt_has_100, mt_none_elapsed
        );
    }
}
