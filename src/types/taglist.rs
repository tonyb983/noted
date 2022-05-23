// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[derive(Clone, Debug, Default, Hash, serde::Deserialize, serde::Serialize)]
pub struct TagList {
    data: Vec<String>,
    dirty: bool,
}

impl TagList {
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn new(data: Vec<String>) -> Self {
        Self { data, dirty: false }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn add(&mut self, tag: String) -> bool {
        if self.data.contains(&tag) {
            false
        } else {
            self.data.push(tag);
            self.dirty = true;
            true
        }
    }

    #[must_use]
    pub fn matches(&self, tag: &str) -> bool {
        self.data.iter().any(|t| t == tag)
    }

    #[must_use]
    pub fn contains(&self, text: &str) -> bool {
        self.data.iter().any(|t| t.contains(text))
    }

    pub fn remove(&mut self, tag: &str) -> bool {
        if let Some(index) = self.data.iter().position(|t| t == tag) {
            self.data.remove(index);
            self.dirty = true;
            true
        } else {
            false
        }
    }
}

impl From<Vec<String>> for TagList {
    fn from(tags: Vec<String>) -> Self {
        Self::new(tags)
    }
}

impl From<Vec<&String>> for TagList {
    fn from(tags: Vec<&String>) -> Self {
        Self::new(tags.into_iter().cloned().collect())
    }
}

impl From<&[String]> for TagList {
    fn from(tags: &[String]) -> Self {
        Self::new(tags.to_vec())
    }
}

impl From<&[&String]> for TagList {
    fn from(tags: &[&String]) -> Self {
        Self::new(tags.iter().map(|&s| s.clone()).collect())
    }
}

impl From<&Vec<&String>> for TagList {
    fn from(tags: &Vec<&String>) -> Self {
        Self::new(tags.iter().map(|&s| s.clone()).collect())
    }
}

impl<T: ToString> FromIterator<T> for TagList {
    fn from_iter<TIter: IntoIterator<Item = T>>(iter: TIter) -> Self {
        Self::new(iter.into_iter().map(|t| t.to_string()).collect())
    }
}
