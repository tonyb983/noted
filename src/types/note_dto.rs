// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod create {
    use serde::{Deserialize, Serialize};

    use crate::types::Reminder;
    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct CreateNote {
        pub title: Option<String>,
        pub content: Option<String>,
        pub tags: Vec<String>,
        pub reminders: Vec<Reminder>,
    }

    impl CreateNote {
        #[must_use]
        pub fn empty() -> Self {
            Self::default()
        }

        #[must_use]
        pub fn new(
            title: Option<String>,
            content: Option<String>,
            tags: Vec<String>,
            reminders: Vec<Reminder>,
        ) -> Self {
            Self {
                title,
                content,
                tags,
                reminders,
            }
        }

        #[must_use]
        pub fn with_title(self, title: Option<String>) -> Self {
            Self { title, ..self }
        }

        #[must_use]
        pub fn with_content(self, content: Option<String>) -> Self {
            Self { content, ..self }
        }

        #[must_use]
        pub fn with_tags(self, tags: Vec<String>) -> Self {
            Self { tags, ..self }
        }

        #[must_use]
        pub fn with_reminders(self, reminders: Vec<Reminder>) -> Self {
            Self { reminders, ..self }
        }

        #[must_use]
        pub fn with_tag(self, tag: String) -> Self {
            let mut next = Self {
                tags: self.tags,
                ..self
            };
            next.tags.push(tag);
            next
        }

        #[must_use]
        pub fn title(&self) -> Option<&str> {
            self.title.as_deref()
        }

        #[must_use]
        pub fn content(&self) -> Option<&str> {
            self.content.as_deref()
        }

        #[must_use]
        pub fn tags(&self) -> &[String] {
            &self.tags
        }

        #[must_use]
        pub fn into_parts(self) -> (Option<String>, Option<String>, Vec<String>, Vec<Reminder>) {
            (self.title, self.content, self.tags, self.reminders)
        }
    }

    impl From<(String, String, Vec<String>)> for CreateNote {
        fn from(parts: (String, String, Vec<String>)) -> Self {
            Self::new(Some(parts.0), Some(parts.1), parts.2, vec![])
        }
    }

    impl From<(&str, &str, Vec<&str>)> for CreateNote {
        fn from(parts: (&str, &str, Vec<&str>)) -> Self {
            Self::new(
                Some(parts.0.to_string()),
                Some(parts.1.to_string()),
                parts
                    .2
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect(),
                vec![],
            )
        }
    }

    impl From<(&str, &str, Vec<&str>, Vec<Reminder>)> for CreateNote {
        fn from(parts: (&str, &str, Vec<&str>, Vec<Reminder>)) -> Self {
            Self::new(
                Some(parts.0.to_string()),
                Some(parts.1.to_string()),
                parts
                    .2
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect(),
                parts.3,
            )
        }
    }

    impl From<(&str, &str)> for CreateNote {
        fn from(parts: (&str, &str)) -> Self {
            Self::new(
                Some(parts.0.to_string()),
                Some(parts.1.to_string()),
                Vec::new(),
                Vec::new(),
            )
        }
    }

    impl From<(Option<String>, Option<String>, Vec<String>)> for CreateNote {
        fn from(parts: (Option<String>, Option<String>, Vec<String>)) -> Self {
            Self::new(parts.0, parts.1, parts.2, Vec::new())
        }
    }

    impl From<(Option<String>, String, Vec<String>)> for CreateNote {
        fn from(parts: (Option<String>, String, Vec<String>)) -> Self {
            Self::new(parts.0, Some(parts.1), parts.2, Vec::new())
        }
    }

    impl From<(String, Option<String>, Vec<String>)> for CreateNote {
        fn from(parts: (String, Option<String>, Vec<String>)) -> Self {
            Self::new(Some(parts.0), parts.1, parts.2, Vec::new())
        }
    }

    impl From<(String, String)> for CreateNote {
        fn from(parts: (String, String)) -> Self {
            Self::new(Some(parts.0), Some(parts.1), Vec::new(), Vec::new())
        }
    }

    impl From<(Option<String>, Option<String>)> for CreateNote {
        fn from(parts: (Option<String>, Option<String>)) -> Self {
            Self::new(parts.0, parts.1, Vec::new(), Vec::new())
        }
    }
}

mod update {
    use serde::{Deserialize, Serialize};
    use tinyid::TinyId;

    use crate::types::Reminder;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UpdateNote {
        pub id: TinyId,
        pub title: Option<String>,
        pub content: Option<String>,
        pub tags: Option<Vec<String>>,
        pub reminders: Option<Vec<Reminder>>,
    }

    impl UpdateNote {
        #[must_use]
        pub fn empty(id: TinyId) -> Self {
            Self {
                id,
                title: None,
                content: None,
                tags: None,
                reminders: None,
            }
        }

        #[must_use]
        pub fn new(
            id: TinyId,
            title: Option<String>,
            content: Option<String>,
            tags: Option<Vec<String>>,
            reminders: Option<Vec<Reminder>>,
        ) -> Self {
            Self {
                id,
                title,
                content,
                tags,
                reminders,
            }
        }

        #[must_use]
        pub fn with_title(self, title: Option<String>) -> Self {
            Self { title, ..self }
        }

        #[must_use]
        pub fn with_content(self, content: Option<String>) -> Self {
            Self { content, ..self }
        }

        #[must_use]
        pub fn with_tags(self, tags: Option<Vec<String>>) -> Self {
            Self { tags, ..self }
        }

        #[must_use]
        pub fn with_tag(self, tag: String) -> Self {
            let mut next = Self {
                tags: self.tags,
                ..self
            };
            match next.tags {
                Some(ref mut tags) => tags.push(tag),
                None => {}
            };
            next
        }

        #[must_use]
        pub fn id(&self) -> &TinyId {
            &self.id
        }

        #[must_use]
        pub fn title(&self) -> Option<&str> {
            self.title.as_deref()
        }

        #[must_use]
        pub fn content(&self) -> Option<&str> {
            self.content.as_deref()
        }

        #[must_use]
        pub fn tags(&self) -> Option<&[String]> {
            self.tags.as_deref()
        }

        #[allow(
            clippy::type_complexity,
            reason = "we are breaking this type into it's parts, I think the meaning is clear"
        )]
        #[must_use]
        pub fn into_parts(
            self,
        ) -> (
            TinyId,
            Option<String>,
            Option<String>,
            Option<Vec<String>>,
            Option<Vec<Reminder>>,
        ) {
            (self.id, self.title, self.content, self.tags, self.reminders)
        }
    }

    impl From<(TinyId, String, String, Vec<String>)> for UpdateNote {
        fn from(parts: (TinyId, String, String, Vec<String>)) -> Self {
            Self::new(parts.0, Some(parts.1), Some(parts.2), Some(parts.3), None)
        }
    }

    impl From<(TinyId, Vec<Reminder>)> for UpdateNote {
        fn from(parts: (TinyId, Vec<Reminder>)) -> Self {
            Self::new(parts.0, None, None, None, Some(parts.1))
        }
    }

    impl From<(TinyId, Reminder)> for UpdateNote {
        fn from(parts: (TinyId, Reminder)) -> Self {
            Self::new(parts.0, None, None, None, Some(vec![parts.1]))
        }
    }

    impl From<(TinyId, String, String, Vec<String>, Vec<Reminder>)> for UpdateNote {
        fn from(parts: (TinyId, String, String, Vec<String>, Vec<Reminder>)) -> Self {
            Self::new(
                parts.0,
                Some(parts.1),
                Some(parts.2),
                Some(parts.3),
                Some(parts.4),
            )
        }
    }

    impl From<(TinyId, Option<String>, Option<String>, Vec<String>)> for UpdateNote {
        fn from(parts: (TinyId, Option<String>, Option<String>, Vec<String>)) -> Self {
            Self::new(parts.0, parts.1, parts.2, Some(parts.3), None)
        }
    }

    impl From<(TinyId, Option<String>, Option<String>, Option<Vec<String>>)> for UpdateNote {
        fn from(parts: (TinyId, Option<String>, Option<String>, Option<Vec<String>>)) -> Self {
            Self::new(parts.0, parts.1, parts.2, parts.3, None)
        }
    }

    impl From<(TinyId, Option<String>, String, Vec<String>)> for UpdateNote {
        fn from(parts: (TinyId, Option<String>, String, Vec<String>)) -> Self {
            Self::new(parts.0, parts.1, Some(parts.2), Some(parts.3), None)
        }
    }

    impl From<(TinyId, String, Option<String>, Vec<String>)> for UpdateNote {
        fn from(parts: (TinyId, String, Option<String>, Vec<String>)) -> Self {
            Self::new(parts.0, Some(parts.1), parts.2, Some(parts.3), None)
        }
    }

    impl From<(TinyId, String, String)> for UpdateNote {
        fn from(parts: (TinyId, String, String)) -> Self {
            Self::new(parts.0, Some(parts.1), Some(parts.2), None, None)
        }
    }

    impl From<(TinyId, Option<String>, Option<String>)> for UpdateNote {
        fn from(parts: (TinyId, Option<String>, Option<String>)) -> Self {
            Self::new(parts.0, parts.1, parts.2, None, None)
        }
    }
}

mod delete {
    use serde::{Deserialize, Serialize};
    use tinyid::TinyId;

    use crate::types::Note;

    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub struct DeleteNote {
        pub id: TinyId,
    }

    impl DeleteNote {
        #[must_use]
        pub fn new(id: TinyId) -> Self {
            Self { id }
        }

        #[must_use]
        pub fn id(&self) -> &TinyId {
            &self.id
        }
    }

    impl From<TinyId> for DeleteNote {
        fn from(id: TinyId) -> Self {
            Self::new(id)
        }
    }

    impl From<&TinyId> for DeleteNote {
        fn from(id: &TinyId) -> Self {
            Self::new(*id)
        }
    }

    impl From<Note> for DeleteNote {
        fn from(note: Note) -> Self {
            Self::new(note.id())
        }
    }

    impl From<&Note> for DeleteNote {
        fn from(note: &Note) -> Self {
            Self::new(note.id())
        }
    }
}

mod dto {
    use super::{CreateNote, DeleteNote, UpdateNote};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum NoteDto {
        Create(CreateNote),
        Update(UpdateNote),
        Delete(DeleteNote),
    }

    impl From<CreateNote> for NoteDto {
        fn from(note: CreateNote) -> Self {
            NoteDto::Create(note)
        }
    }

    impl From<UpdateNote> for NoteDto {
        fn from(note: UpdateNote) -> Self {
            NoteDto::Update(note)
        }
    }

    impl From<DeleteNote> for NoteDto {
        fn from(note: DeleteNote) -> Self {
            NoteDto::Delete(note)
        }
    }
}

mod ops {
    use serde::{Deserialize, Serialize};

    use crate::types::{Mutation, Predicate};

    /// Operations that can be applied to a [`String`].
    ///
    /// Unfortunately, my mind is blanking on the best way to have both the "clone and modify"
    /// and "modify in place" operations in the same implementation so currently `apply_to` and
    /// `apply_to_in_place` must be kept in sync manually.
    #[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum StringOperation {
        /// Add text to the end of a string
        Append(String),
        /// Add text to the beginning of a string
        Prepend(String),
        /// Insert text at the given index
        InsertAt(usize, String),
        /// Remove whitespace from the beginning and end of a string
        TrimWhitespace,
        /// Remove the given pattern if it appears at the beginning of the string
        TrimFront(Vec<char>),
        /// Remove the given pattern if it appears at the end of the string
        TrimBack(Vec<char>),
        /// Remove the given pattern if it appears at the beginning **or** end of the string
        Trim(Vec<char>),
        /// Keeps the first n characters of the string
        KeepFront(usize),
        /// Keeps the last n characters of the string
        KeepBack(usize),
        /// Replaces the first instance of the given pattern with the given replacement
        ReplaceWord(String, String),
        /// Replaces N instances of the given pattern with the given replacement
        ReplaceWordsN(usize, String, String),
        /// Replaces all instances of the given pattern with the given replacement
        ReplaceWordsAll(String, String),
        /// For each pair given, replace all instances of the pattern
        ReplaceManyWords(Vec<(String, String)>),
        /// Clear / empty the string
        Clear,
    }

    impl StringOperation {
        /// Applies this [`StringOperation`] to the given string, returning a newly allocated string and
        /// leaving the original unchanged
        #[tracing::instrument(skip(string), fields(input = %string, output))]
        pub fn apply_to(&self, string: &str) -> String {
            let result = match self {
                Self::Append(text) => {
                    format!("{}{}", string, text)
                }
                Self::Prepend(text) => {
                    let mut s = string.to_string();
                    s.insert_str(0, text);
                    s
                }
                Self::InsertAt(i, text) => {
                    let mut s = string.to_string();
                    s.insert_str(*i, text);
                    s
                }
                Self::TrimWhitespace => string.trim().to_string(),
                Self::TrimFront(chars) => string.trim_start_matches(chars.as_slice()).to_string(),
                Self::TrimBack(chars) => string.trim_end_matches(chars.as_slice()).to_string(),
                Self::Trim(chars) => string.trim_matches(chars.as_slice()).to_string(),
                Self::KeepFront(n) => {
                    if string.len() > *n {
                        string.chars().take(*n).collect()
                    } else {
                        string.to_string()
                    }
                }
                Self::KeepBack(n) => {
                    if string.len() > *n {
                        let skip = string.len() - *n;
                        string.chars().skip(skip).collect()
                    } else {
                        string.to_string()
                    }
                }
                Self::ReplaceWord(old, new) => string.replacen(old, new, 1),
                Self::ReplaceWordsN(n, old, new) => string.replacen(old, new, *n),
                Self::ReplaceWordsAll(old, new) => string.replace(old, new),
                Self::ReplaceManyWords(pairs) => {
                    // TODO: Check the performance of this. Perhaps only resort to using `aho-corasick` if the number
                    // of pairs is larger than some threshold?

                    use aho_corasick::AhoCorasick;

                    let (patterns, replacements): (Vec<&String>, Vec<&String>) =
                        pairs.iter().map(|&(ref a, ref b)| (a, b)).unzip();

                    let mut result = vec![];

                    let ac = AhoCorasick::new(patterns);

                    ac.stream_replace_all(string.as_bytes(), &mut result, replacements.as_slice())
                        .expect("stream_replace_all failed");

                    match String::from_utf8(result) {
                        Ok(s) => s,
                        Err(err) => {
                            tracing::error!(%err, "StringOperation::ReplaceManyWords - String::from_utf8 failed.");
                            string.to_string()
                        }
                    }
                }
                Self::Clear => String::new(),
            };

            #[cfg(feature = "trace")]
            {
                tracing::Span::current().record("result", &result);
            }

            result
        }

        /// Applies this [`StringOperation`] by mutating the given `string`.
        #[tracing::instrument(skip(string), fields(input = %string, output))]
        pub fn apply_in_place(&self, string: &mut String) {
            match self {
                Self::Append(text) => string.push_str(text),
                Self::Prepend(text) => string.insert_str(0, text),
                Self::InsertAt(i, text) => string.insert_str(*i, text),
                Self::TrimWhitespace => {
                    *string = string.trim().to_string();
                }
                Self::TrimFront(chars) => {
                    *string = string.trim_start_matches(chars.as_slice()).to_string();
                }
                Self::TrimBack(chars) => {
                    *string = string.trim_end_matches(chars.as_slice()).to_string();
                }
                Self::Trim(chars) => {
                    *string = string.trim_matches(chars.as_slice()).to_string();
                }
                Self::KeepFront(n) => {
                    if string.len() > *n {
                        *string = string.chars().take(*n).collect();
                    }
                }
                Self::KeepBack(n) => {
                    if string.len() > *n {
                        let skip = string.len() - *n;
                        *string = string.chars().skip(skip).collect();
                    }
                }
                Self::ReplaceWord(old, new) => {
                    *string = string.replacen(old, new, 1);
                }
                Self::ReplaceWordsN(n, old, new) => {
                    *string = string.replacen(old, new, *n);
                }
                Self::ReplaceWordsAll(old, new) => {
                    *string = string.replace(old, new);
                }
                Self::ReplaceManyWords(pairs) => {
                    // TODO: Check the performance of this. Perhaps only resort to using `aho-corasick` if the number
                    // of pairs is larger than some threshold?

                    use aho_corasick::AhoCorasick;

                    let (patterns, replacements): (Vec<&String>, Vec<&String>) =
                        pairs.iter().map(|&(ref a, ref b)| (a, b)).unzip();

                    // let patterns = pairs.iter().map(|(o, _)| o).collect::<Vec<_>>();
                    // let replacements = pairs.iter().map(|(_, n)| n).collect::<Vec<_>>();

                    let mut result = vec![];

                    let ac = AhoCorasick::new(patterns);

                    ac.stream_replace_all(string.as_bytes(), &mut result, replacements.as_slice())
                        .expect("stream_replace_all failed");

                    match String::from_utf8(result) {
                        Ok(s) => {
                            *string = s;
                        }
                        Err(err) => {
                            tracing::error!(%err, "StringOperation::ReplaceManyWords - String::from_utf8 failed.");
                        }
                    }
                }
                Self::Clear => {
                    string.clear();
                }
            }

            #[cfg(feature = "trace")]
            {
                tracing::Span::current().record("output", string);
            }
        }
    }

    pub enum CollectionOperation<T> {
        /// Add a single item to the collection
        Add(T),
        /// Add many items to the collection
        AddMany(Vec<T>),
        /// Insert an item at the specified index
        Insert(usize, T),
        /// Insert many items at the specified indices
        InsertMany(Vec<(usize, T)>),
        /// Replaces the first item that matches the given predicate with the given item
        Replace(Predicate<T>, T),
        /// Remove an item
        Remove(T),
        /// Remove the item at the given index
        RemoveAt(usize),
        /// Remove many items
        RemoveMany(Vec<T>),
        /// Filter out any items matching the given predicate
        Filter(Predicate<T>),
        /// Retain any items matching the given predicate
        Retain(Predicate<T>),
    }

    impl<T: PartialEq> CollectionOperation<T> {
        /// Applies this [`CollectionOperation`] to the given `collection`.
        #[tracing::instrument(skip_all)]
        pub fn into_apply_in_place(self, collection: &mut Vec<T>) {
            match self {
                Self::Add(item) => collection.push(item),
                Self::AddMany(items) => collection.extend(items.into_iter()),
                Self::Insert(i, item) => collection.insert(i, item),
                Self::InsertMany(items) => {
                    for (i, item) in items {
                        collection.insert(i, item);
                    }
                }
                Self::Replace(predicate, item) => {
                    for member in collection.iter_mut() {
                        if predicate(member) {
                            *member = item;
                            break;
                        }
                    }
                }
                Self::Remove(item) => collection.retain(|i| i != &item),
                Self::RemoveAt(i) => {
                    collection.remove(i);
                }
                Self::RemoveMany(items) => collection.retain(|i| !items.contains(i)),
                Self::Filter(predicate) => collection.retain(|item| !predicate(item)),
                Self::Retain(predicate) => collection.retain(predicate),
            }
        }
    }

    impl<T: PartialEq + Clone> CollectionOperation<T> {
        /// Applies this [`CollectionOperation`] to the given `collection`.
        #[tracing::instrument(skip_all)]
        pub fn apply_in_place(&self, collection: &mut Vec<T>) {
            match self {
                Self::Add(item) => collection.push(item.clone()),
                Self::AddMany(items) => {
                    collection.extend(items.iter().cloned());
                }
                Self::Insert(i, item) => collection.insert(*i, item.clone()),
                Self::InsertMany(items) => {
                    for (i, item) in items {
                        collection.insert(*i, item.clone());
                    }
                }
                Self::Replace(predicate, item) => {
                    for member in collection.iter_mut() {
                        if predicate(member) {
                            *member = item.clone();
                            break;
                        }
                    }
                }
                Self::Remove(item) => collection.retain(|i| i != item),
                Self::RemoveAt(i) => {
                    collection.remove(*i);
                }
                Self::RemoveMany(items) => collection.retain(|i| !items.contains(i)),
                Self::Filter(predicate) => collection.retain(|item| !predicate(item)),
                Self::Retain(predicate) => collection.retain(predicate),
            }
        }
    }
}

pub use create::*;
pub use delete::*;
pub use dto::*;
pub use update::*;
