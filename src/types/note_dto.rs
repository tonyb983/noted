// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod create {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct CreateNote {
        pub title: Option<String>,
        pub content: Option<String>,
        pub tags: Vec<String>,
    }

    impl CreateNote {
        #[must_use]
        pub fn empty() -> Self {
            Self::default()
        }

        #[must_use]
        pub fn new(title: Option<String>, content: Option<String>, tags: Vec<String>) -> Self {
            Self {
                title,
                content,
                tags,
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
        pub fn into_parts(self) -> (Option<String>, Option<String>, Vec<String>) {
            (self.title, self.content, self.tags)
        }
    }

    impl From<(String, String, Vec<String>)> for CreateNote {
        fn from(parts: (String, String, Vec<String>)) -> Self {
            Self::new(Some(parts.0), Some(parts.1), parts.2)
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
            )
        }
    }

    impl From<(&str, &str)> for CreateNote {
        fn from(parts: (&str, &str)) -> Self {
            Self::new(
                Some(parts.0.to_string()),
                Some(parts.1.to_string()),
                Vec::new(),
            )
        }
    }

    impl From<(Option<String>, Option<String>, Vec<String>)> for CreateNote {
        fn from(parts: (Option<String>, Option<String>, Vec<String>)) -> Self {
            Self::new(parts.0, parts.1, parts.2)
        }
    }

    impl From<(Option<String>, String, Vec<String>)> for CreateNote {
        fn from(parts: (Option<String>, String, Vec<String>)) -> Self {
            Self::new(parts.0, Some(parts.1), parts.2)
        }
    }

    impl From<(String, Option<String>, Vec<String>)> for CreateNote {
        fn from(parts: (String, Option<String>, Vec<String>)) -> Self {
            Self::new(Some(parts.0), parts.1, parts.2)
        }
    }

    impl From<(String, String)> for CreateNote {
        fn from(parts: (String, String)) -> Self {
            Self::new(Some(parts.0), Some(parts.1), Vec::new())
        }
    }

    impl From<(Option<String>, Option<String>)> for CreateNote {
        fn from(parts: (Option<String>, Option<String>)) -> Self {
            Self::new(parts.0, parts.1, Vec::new())
        }
    }
}

mod update {
    use serde::{Deserialize, Serialize};

    use crate::ShortId;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UpdateNote {
        pub id: ShortId,
        pub title: Option<String>,
        pub content: Option<String>,
        pub tags: Option<Vec<String>>,
    }

    impl UpdateNote {
        #[must_use]
        pub fn empty(id: ShortId) -> Self {
            Self {
                id,
                title: None,
                content: None,
                tags: None,
            }
        }

        #[must_use]
        pub fn new(
            id: ShortId,
            title: Option<String>,
            content: Option<String>,
            tags: Option<Vec<String>>,
        ) -> Self {
            Self {
                id,
                title,
                content,
                tags,
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
        pub fn id(&self) -> &ShortId {
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

        #[must_use]
        pub fn into_parts(self) -> (ShortId, Option<String>, Option<String>, Option<Vec<String>>) {
            (self.id, self.title, self.content, self.tags)
        }
    }

    impl From<(ShortId, String, String, Vec<String>)> for UpdateNote {
        fn from(parts: (ShortId, String, String, Vec<String>)) -> Self {
            Self::new(parts.0, Some(parts.1), Some(parts.2), Some(parts.3))
        }
    }

    impl From<(ShortId, Option<String>, Option<String>, Vec<String>)> for UpdateNote {
        fn from(parts: (ShortId, Option<String>, Option<String>, Vec<String>)) -> Self {
            Self::new(parts.0, parts.1, parts.2, Some(parts.3))
        }
    }

    impl From<(ShortId, Option<String>, Option<String>, Option<Vec<String>>)> for UpdateNote {
        fn from(parts: (ShortId, Option<String>, Option<String>, Option<Vec<String>>)) -> Self {
            Self::new(parts.0, parts.1, parts.2, parts.3)
        }
    }

    impl From<(ShortId, Option<String>, String, Vec<String>)> for UpdateNote {
        fn from(parts: (ShortId, Option<String>, String, Vec<String>)) -> Self {
            Self::new(parts.0, parts.1, Some(parts.2), Some(parts.3))
        }
    }

    impl From<(ShortId, String, Option<String>, Vec<String>)> for UpdateNote {
        fn from(parts: (ShortId, String, Option<String>, Vec<String>)) -> Self {
            Self::new(parts.0, Some(parts.1), parts.2, Some(parts.3))
        }
    }

    impl From<(ShortId, String, String)> for UpdateNote {
        fn from(parts: (ShortId, String, String)) -> Self {
            Self::new(parts.0, Some(parts.1), Some(parts.2), None)
        }
    }

    impl From<(ShortId, Option<String>, Option<String>)> for UpdateNote {
        fn from(parts: (ShortId, Option<String>, Option<String>)) -> Self {
            Self::new(parts.0, parts.1, parts.2, None)
        }
    }
}

mod delete {
    use serde::{Deserialize, Serialize};

    use crate::ShortId;

    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub struct DeleteNote {
        pub id: ShortId,
    }

    impl DeleteNote {
        #[must_use]
        pub fn new(id: ShortId) -> Self {
            Self { id }
        }

        #[must_use]
        pub fn id(&self) -> &ShortId {
            &self.id
        }
    }

    impl From<ShortId> for DeleteNote {
        fn from(id: ShortId) -> Self {
            Self::new(id)
        }
    }

    impl From<&ShortId> for DeleteNote {
        fn from(id: &ShortId) -> Self {
            Self::new(*id)
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

pub use create::*;
pub use delete::*;
pub use dto::*;
pub use update::*;
