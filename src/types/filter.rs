// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! TODO: Clean up this file, maybe separate or at least better organize the types.

use time::OffsetDateTime;

use crate::types::Note;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NoteStringField {
    Title,
    Content,
    Tags,
}

impl NoteStringField {
    /// TODO: Make this return reference not value
    #[must_use]
    pub fn get_value(&self, note: &Note) -> String {
        match self {
            NoteStringField::Title => note.title().to_string(),
            NoteStringField::Content => note.content().to_string(),
            NoteStringField::Tags => note.tags().join(","),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NoteDateField {
    Created,
    Updated,
}

impl NoteDateField {
    #[must_use]
    pub fn get_value<'s, 'n>(&'s self, note: &'n Note) -> &'n OffsetDateTime {
        match self {
            NoteDateField::Created => note.created(),
            NoteDateField::Updated => note.updated(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StringSearchArgs {
    text: String,
    invert: bool,
}

impl StringSearchArgs {
    #[must_use]
    pub fn new(text: String, invert: bool) -> StringSearchArgs {
        StringSearchArgs { text, invert }
    }

    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    #[must_use]
    pub fn invert(&self) -> bool {
        self.invert
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StringSearch {
    Contains(StringSearchArgs),
    Matches(StringSearchArgs),
    StartsWith(StringSearchArgs),
    EndsWith(StringSearchArgs),
}

impl StringSearch {
    #[must_use]
    pub fn args(&self) -> &StringSearchArgs {
        match self {
            StringSearch::Matches(args)
            | StringSearch::EndsWith(args)
            | StringSearch::StartsWith(args)
            | StringSearch::Contains(args) => args,
        }
    }

    #[must_use]
    pub fn invert(&self) -> bool {
        match self {
            StringSearch::Matches(args)
            | StringSearch::EndsWith(args)
            | StringSearch::StartsWith(args)
            | StringSearch::Contains(args) => args.invert(),
        }
    }

    #[must_use]
    pub fn text(&self) -> &str {
        match self {
            StringSearch::Matches(args)
            | StringSearch::EndsWith(args)
            | StringSearch::StartsWith(args)
            | StringSearch::Contains(args) => args.text(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DateSearch {
    Before(OffsetDateTime),
    After(OffsetDateTime),
    On(OffsetDateTime),
    Between(OffsetDateTime, OffsetDateTime),
    NotBetween(OffsetDateTime, OffsetDateTime),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NoteFilter {
    String(NoteStringField, StringSearch),
    Date(NoteDateField, DateSearch),
}

pub type Predicate = Box<dyn Fn(&&Note) -> bool + Send + Sync>;

impl NoteFilter {
    #[must_use]
    pub fn predicate(&self) -> Predicate {
        match self {
            NoteFilter::String(field, ss) => {
                let field = *field;
                match ss.clone() {
                    StringSearch::Contains(args) => box move |&note| {
                        let value = field.get_value(note);
                        if args.invert() {
                            !value.contains(args.text())
                        } else {
                            value.contains(args.text())
                        }
                    },
                    StringSearch::Matches(args) => box move |&note| {
                        let value = field.get_value(note);
                        if args.invert() {
                            value != args.text()
                        } else {
                            value == args.text()
                        }
                    },
                    StringSearch::StartsWith(args) => box move |&note| {
                        let value = field.get_value(note);
                        if args.invert() {
                            !value.starts_with(args.text())
                        } else {
                            value.starts_with(args.text())
                        }
                    },
                    StringSearch::EndsWith(args) => box move |&note| {
                        let value = field.get_value(note);
                        if args.invert() {
                            !value.ends_with(args.text())
                        } else {
                            value.ends_with(args.text())
                        }
                    },
                }
            }
            NoteFilter::Date(field, ds) => {
                let field = *field;
                match *ds {
                    DateSearch::Before(other) => box move |note| {
                        let value = *field.get_value(note);
                        value < other
                    },
                    DateSearch::After(other) => box move |note| {
                        let value = *field.get_value(note);
                        value > other
                    },
                    DateSearch::On(other) => box move |note| {
                        let value = *field.get_value(note);
                        value == other
                    },
                    DateSearch::Between(a, b) => box move |note| {
                        let value = *field.get_value(note);
                        value >= a && value <= b
                    },
                    DateSearch::NotBetween(a, b) => box move |note| {
                        let value = *field.get_value(note);
                        value < a || value > b
                    },
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Filter {
    filters: Vec<NoteFilter>,
}

impl Filter {
    #[must_use]
    pub fn empty() -> Filter {
        Self {
            filters: Vec::new(),
        }
    }

    #[must_use]
    pub fn single(filter: NoteFilter) -> Filter {
        Filter {
            filters: vec![filter],
        }
    }

    #[must_use]
    pub fn multiple(filters: Vec<NoteFilter>) -> Filter {
        Filter { filters }
    }

    #[must_use]
    pub fn add_string(mut self, field: NoteStringField, search: StringSearch) -> Filter {
        self.filters.push(NoteFilter::String(field, search));
        self
    }

    #[must_use]
    pub fn add_date(mut self, field: NoteDateField, search: DateSearch) -> Filter {
        self.filters.push(NoteFilter::Date(field, search));
        self
    }

    #[must_use]
    pub fn filters(&self) -> &[NoteFilter] {
        &self.filters
    }

    #[must_use]
    pub fn predicate(&self) -> Predicate {
        if self.filters.is_empty() {
            return box |_| true;
        }

        let mut predicates = Vec::new();
        for filter in &self.filters {
            predicates.push(filter.predicate());
        }

        box move |note| predicates.iter().all(|pred| pred(note))
    }
}

impl Default for Filter {
    fn default() -> Self {
        Self::empty()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Count {
    All,
    Some(usize),
    One,
}

impl Count {
    #[must_use]
    pub fn to_usize(&self) -> usize {
        match self {
            Count::All => usize::MAX,
            Count::Some(n) => *n,
            Count::One => 1,
        }
    }
}

impl From<usize> for Count {
    fn from(count: usize) -> Self {
        match count {
            0 => Count::All,
            1 => Count::One,
            _ => Count::Some(count),
        }
    }
}

impl Default for Count {
    fn default() -> Self {
        Count::All
    }
}
