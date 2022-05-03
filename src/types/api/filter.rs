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

    pub fn set_title(&mut self, title: String) {
        self.text = title;
    }

    pub fn set_invert(&mut self, invert: bool) {
        self.invert = invert;
    }

    pub fn toggle_invert(&mut self) {
        self.invert = !self.invert;
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StringSearch {
    Contains(StringSearchArgs),
    Matches(StringSearchArgs),
    StartsWith(StringSearchArgs),
    EndsWith(StringSearchArgs),
}

/// Constructors
impl StringSearch {
    #[must_use]
    pub fn contains(text: String, invert: bool) -> Self {
        Self::Contains(StringSearchArgs::new(text, invert))
    }

    #[must_use]
    pub fn matches(text: String, invert: bool) -> Self {
        Self::Matches(StringSearchArgs::new(text, invert))
    }

    #[must_use]
    pub fn starts_with(text: String, invert: bool) -> Self {
        Self::StartsWith(StringSearchArgs::new(text, invert))
    }

    #[must_use]
    pub fn ends_with(text: String, invert: bool) -> Self {
        Self::EndsWith(StringSearchArgs::new(text, invert))
    }
}

/// Member Functions
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

    pub fn args_mut(&mut self) -> &mut StringSearchArgs {
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

/// Constructors
impl DateSearch {
    #[must_use]
    pub fn before(date: OffsetDateTime) -> Self {
        Self::Before(date)
    }

    #[must_use]
    pub fn after(date: OffsetDateTime) -> Self {
        Self::After(date)
    }

    #[must_use]
    pub fn on(date: OffsetDateTime) -> Self {
        Self::On(date)
    }

    #[must_use]
    pub fn between(start: OffsetDateTime, end: OffsetDateTime) -> Self {
        Self::Between(start, end)
    }

    #[must_use]
    pub fn not_between(start: OffsetDateTime, end: OffsetDateTime) -> Self {
        Self::NotBetween(start, end)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NoteFilter {
    String(NoteStringField, StringSearch),
    Date(NoteDateField, DateSearch),
}

pub type Predicate = Box<dyn Fn(&&Note) -> bool + Send + Sync>;
pub type GenericPredicate<T> = Box<dyn Fn(&&T) -> bool + Send + Sync>;

pub trait GenericFilter {
    type Target;
    fn predicate(&self) -> GenericPredicate<Self::Target>;
}

pub trait DataFilter {
    type Target;
    type Filter: GenericFilter<Target = Self::Target>;

    fn add_filter(&mut self, filter: Self::Filter);
    fn get_filters(&self) -> Vec<Self::Filter>;
    fn clear_filters(&mut self);

    fn get_predicate(&self) -> GenericPredicate<Self::Target>;
}

/// Constructors
impl NoteFilter {
    #[must_use]
    pub fn title(search: StringSearch) -> Self {
        NoteFilter::String(NoteStringField::Title, search)
    }

    #[must_use]
    pub fn content(search: StringSearch) -> Self {
        NoteFilter::String(NoteStringField::Content, search)
    }

    #[must_use]
    pub fn tag(search: StringSearch) -> Self {
        NoteFilter::String(NoteStringField::Tags, search)
    }

    #[must_use]
    pub fn created(search: DateSearch) -> Self {
        NoteFilter::Date(NoteDateField::Created, search)
    }

    #[must_use]
    pub fn updated(search: DateSearch) -> Self {
        NoteFilter::Date(NoteDateField::Updated, search)
    }
}

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

    pub fn set_string_search(&mut self, field: NoteStringField, ss: StringSearch) {
        *self = NoteFilter::String(field, ss);
    }

    pub fn set_date_search(&mut self, field: NoteDateField, ds: DateSearch) {
        *self = NoteFilter::Date(field, ds);
    }

    #[must_use]
    pub fn string_search(&self) -> Option<&StringSearch> {
        match self {
            NoteFilter::String(_, ss) => Some(ss),
            NoteFilter::Date(_, _) => None,
        }
    }

    #[must_use]
    pub fn date_search(&self) -> Option<&DateSearch> {
        match self {
            NoteFilter::Date(_, ds) => Some(ds),
            NoteFilter::String(_, _) => None,
        }
    }

    #[must_use]
    pub fn string_field(&self) -> Option<&NoteStringField> {
        match self {
            NoteFilter::String(f, _) => Some(f),
            NoteFilter::Date(_, _) => None,
        }
    }

    #[must_use]
    pub fn date_field(&self) -> Option<&NoteDateField> {
        match self {
            NoteFilter::Date(f, _) => Some(f),
            NoteFilter::String(_, _) => None,
        }
    }
    /////////////////////////////////////////

    #[must_use]
    pub fn string_search_mut(&mut self) -> Option<&mut StringSearch> {
        match self {
            NoteFilter::String(_, ss) => Some(ss),
            NoteFilter::Date(_, _) => None,
        }
    }

    #[must_use]
    pub fn date_search_mut(&mut self) -> Option<&mut DateSearch> {
        match self {
            NoteFilter::Date(_, ds) => Some(ds),
            NoteFilter::String(_, _) => None,
        }
    }

    #[must_use]
    pub fn string_field_mut(&mut self) -> Option<&mut NoteStringField> {
        match self {
            NoteFilter::String(f, _) => Some(f),
            NoteFilter::Date(_, _) => None,
        }
    }

    #[must_use]
    pub fn date_field_mut(&mut self) -> Option<&mut NoteDateField> {
        match self {
            NoteFilter::Date(f, _) => Some(f),
            NoteFilter::String(_, _) => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Filter {
    filters: Vec<NoteFilter>,
}

/// Constructors
impl Filter {
    #[must_use]
    pub fn empty() -> Self {
        Self {
            filters: Vec::new(),
        }
    }

    #[must_use]
    pub fn single(filter: NoteFilter) -> Self {
        Self {
            filters: vec![filter],
        }
    }

    #[must_use]
    pub fn multiple(filters: Vec<NoteFilter>) -> Self {
        Self { filters }
    }
}

/// Member Functions
impl Filter {
    pub fn add_filter(&mut self, filter: NoteFilter) {
        self.filters.push(filter);
    }

    #[must_use]
    pub fn with_filter(mut self, filter: NoteFilter) -> Self {
        self.add_filter(filter);
        self
    }

    #[must_use]
    pub fn with_string_filter(mut self, field: NoteStringField, search: StringSearch) -> Self {
        self.filters.push(NoteFilter::String(field, search));
        self
    }

    #[must_use]
    pub fn with_date_filter(mut self, field: NoteDateField, search: DateSearch) -> Self {
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
            0 | usize::MAX => Count::All,
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

#[cfg(test)]
mod tests {
    use crate::types::api::Ordering;
    use pretty_assertions::{assert_eq, assert_ne, assert_str_eq};

    use super::*;

    fn create_notes() -> Vec<Note> {
        vec![
            Note::create((
                "A Title",
                "The content for the note goes here.",
                vec!["tag1", "tag2"],
            )),
            Note::create((
                "Some Title",
                "Here is content for the note the for content is Here.",
                vec!["1tag", "2tag"],
            )),
            Note::create((
                "This is Title",
                "Whoa, how about this note content.",
                vec!["tag1", "tag2"],
            )),
            Note::create((
                "Gooooo Title",
                "My my, how amazing this note is!",
                vec!["whoa", "dude"],
            )),
            Note::create((
                "Last Title",
                "Once upon a time, there was a note. It was beautiful.",
                vec!["tag1", "tag2"],
            )),
            Note::create((
                "Title Goes Here",
                "This note has a title that does not end in the word Title, what a fkin rebel this guy is.",
                vec!["what", "a", "rebel"],
            )),
        ]
    }

    fn fake_repo_get(notes: &[Note], filter: &Filter, order: Ordering, count: Count) -> Vec<Note> {
        let mut notes = notes.to_vec();
        notes.sort_unstable_by(order.comparison());
        notes
            .iter()
            .filter(filter.predicate())
            .take(count.to_usize())
            .cloned()
            .collect()
    }

    fn apply_filter(notes: &[Note], filter: &NoteFilter) -> Vec<Note> {
        notes.iter().filter(filter.predicate()).cloned().collect()
    }

    fn apply_filters(notes: &[Note], filter: &Filter) -> Vec<Note> {
        notes.iter().filter(filter.predicate()).cloned().collect()
    }

    fn apply_order(notes: &[Note], order: Ordering) -> Vec<Note> {
        let mut notes = notes.to_vec();
        notes.sort_unstable_by(order.comparison());
        notes
    }

    fn apply_count(notes: &[Note], count: Count) -> Vec<Note> {
        notes.iter().take(count.to_usize()).cloned().collect()
    }

    fn print_titles(notes: &[Note], linebreak: bool) {
        let titles = notes.iter().map(Note::title).collect::<Vec<_>>();
        if linebreak {
            println!("{}", titles.join("\n"));
        } else {
            println!("{}", titles.join(" "));
        }
    }

    #[test]
    fn filter_misc() {
        let notes = create_notes();
        assert_eq!(notes.len(), 6, "create_notes should create 6 notes");

        let filtered = apply_filters(&notes, &Filter::default());
        assert_eq!(filtered.len(), 6, "apply_filters should return all notes");

        let filter = Filter::empty();
        let count = Count::All;
    }

    #[test]
    fn filter_title() {
        let notes = create_notes();
        assert_eq!(notes.len(), 6, "create_notes should create 6 notes");
        // println!("Original Notes:\n{:#?}", notes);
        let mut title_contains_some =
            NoteFilter::title(StringSearch::contains("Some".to_string(), false));
        let mut title_matches_some_title =
            NoteFilter::title(StringSearch::matches("Some Title".to_string(), false));
        let mut title_starts_with_last =
            NoteFilter::title(StringSearch::starts_with("Last".to_string(), false));
        let mut title_ends_with_title =
            NoteFilter::title(StringSearch::ends_with("Title".to_string(), false));
        let multiple_filters = Filter::empty()
            .with_string_filter(
                NoteStringField::Title,
                StringSearch::Contains(StringSearchArgs::new("Some".to_string(), false)),
            )
            .with_filter(title_ends_with_title.clone());
        let filtered = apply_filters(&notes, &multiple_filters);
        assert_eq!(filtered.len(), 1);
        let first = filtered
            .first()
            .expect("Unable to get first element of filtered");
        // println!("Filtered Note:\n{:#?}", filtered);
        assert_eq!(first.title(), "Some Title");
        assert_eq!(
            first.content(),
            "Here is content for the note the for content is Here."
        );
        let filtered = apply_filter(&notes, &title_ends_with_title);
        assert_eq!(filtered.len(), 5);
        title_ends_with_title
            .string_search_mut()
            .unwrap()
            .args_mut()
            .toggle_invert();
        let filtered = apply_filter(&notes, &title_ends_with_title);
        assert_eq!(filtered.len(), 1);

        let filtered = apply_filter(&notes, &title_matches_some_title);
        assert_eq!(filtered.len(), 1);
        title_matches_some_title
            .string_search_mut()
            .unwrap()
            .args_mut()
            .toggle_invert();
        let filtered = apply_filter(&notes, &title_matches_some_title);
        assert_eq!(filtered.len(), 5);

        let filtered = apply_filter(&notes, &title_starts_with_last);
        assert_eq!(filtered.len(), 1);
        title_starts_with_last
            .string_search_mut()
            .unwrap()
            .args_mut()
            .toggle_invert();
        let filtered = apply_filter(&notes, &title_starts_with_last);
        assert_eq!(filtered.len(), 5);
    }

    #[test]
    fn filter_content() {
        let notes = create_notes();
        assert_eq!(notes.len(), 6, "create_notes should create 6 notes");

        let mut content_contains_the =
            NoteFilter::content(StringSearch::contains("the".to_string(), false));

        let filtered = apply_filter(&notes, &content_contains_the);
        assert_eq!(filtered.len(), 4);

        content_contains_the
            .string_search_mut()
            .unwrap()
            .args_mut()
            .toggle_invert();

        let filtered = apply_filter(&notes, &content_contains_the);
        assert_eq!(filtered.len(), 2);

        let mut content_contains_zzz =
            NoteFilter::content(StringSearch::contains("zzz".to_string(), false));
        let filtered = apply_filter(&notes, &content_contains_zzz);
        assert_eq!(filtered.len(), 0);
        assert!(filtered.is_empty());

        content_contains_zzz
            .string_search_mut()
            .unwrap()
            .args_mut()
            .toggle_invert();
        let filtered = apply_filter(&notes, &content_contains_zzz);
        assert_eq!(filtered.len(), 6);

        // "Whoa, how about this note content."
        let mut content_matches = NoteFilter::content(StringSearch::matches(
            "Whoa, how about this note content.".to_string(),
            false,
        ));
        let filtered = apply_filter(&notes, &content_matches);
        assert_eq!(filtered.len(), 1);
        content_matches
            .string_search_mut()
            .unwrap()
            .args_mut()
            .toggle_invert();
        let filtered = apply_filter(&notes, &content_matches);
        assert_eq!(filtered.len(), 5);
    }

    #[test]
    fn filter_created() {
        let notes = create_notes();
        assert_eq!(notes.len(), 6, "create_notes should create 6 notes");

        let first_created = *notes
            .first()
            .expect("unable to get notes.first()")
            .created();
        let last_created = *notes.last().expect("unable to get notes.last()").created();

        let mut between = NoteFilter::created(DateSearch::between(first_created, last_created));
        let filtered = apply_filter(&notes, &between);
        assert_eq!(filtered.len(), 6);

        let mut not_between =
            NoteFilter::created(DateSearch::not_between(first_created, last_created));
        let filtered = apply_filter(&notes, &not_between);
        assert_eq!(filtered.len(), 0);

        let now = OffsetDateTime::now_utc();
        let mut before = NoteFilter::created(DateSearch::before(now));
        let filtered = apply_filter(&notes, &before);
        assert_eq!(filtered.len(), 6);

        let mut after = NoteFilter::created(DateSearch::after(now));
        let filtered = apply_filter(&notes, &after);
        assert_eq!(filtered.len(), 0);

        let mut on_first = NoteFilter::created(DateSearch::on(first_created));
        let filtered = apply_filter(&notes, &on_first);
        assert_eq!(filtered.len(), 1);

        let mut on_now = NoteFilter::created(DateSearch::on(now));
        let filtered = apply_filter(&notes, &on_now);
        assert_eq!(filtered.len(), 0);
    }

    #[test]
    fn filter_updated() {
        let mut notes = create_notes();
        assert_eq!(notes.len(), 6, "create_notes should create 6 notes");

        let first = *notes
            .first()
            .expect("unable to get notes.first()")
            .updated();
        let last = *notes.last().expect("unable to get notes.last()").updated();

        let mut between = NoteFilter::updated(DateSearch::between(first, last));
        let filtered = apply_filter(&notes, &between);
        assert_eq!(filtered.len(), 6);

        let mut not_between = NoteFilter::updated(DateSearch::not_between(first, last));
        let filtered = apply_filter(&notes, &not_between);
        assert_eq!(filtered.len(), 0);

        let now = OffsetDateTime::now_utc();
        let mut before = NoteFilter::updated(DateSearch::before(now));
        let filtered = apply_filter(&notes, &before);
        assert_eq!(filtered.len(), 6);

        let mut after = NoteFilter::updated(DateSearch::after(now));
        let filtered = apply_filter(&notes, &after);
        assert_eq!(filtered.len(), 0);

        let mut on_first = NoteFilter::updated(DateSearch::on(first));
        let filtered = apply_filter(&notes, &on_first);
        assert_eq!(filtered.len(), 1);

        let mut on_now = NoteFilter::updated(DateSearch::on(now));
        let filtered = apply_filter(&notes, &on_now);
        assert_eq!(filtered.len(), 0);

        assert_eq!(*notes.first().unwrap().updated(), first);
        notes
            .first_mut()
            .unwrap()
            .append_content("Here is more content!");
        assert_ne!(*notes.first().unwrap().updated(), first);

        let mut after = NoteFilter::updated(DateSearch::after(now));
        let filtered = apply_filter(&notes, &after);
        assert_eq!(filtered.len(), 1);
    }

    #[test]
    fn count() {
        let notes = create_notes();
        assert_eq!(notes.len(), 6, "create_notes should create 6 notes");

        let one = Count::One;
        let two = Count::Some(2);
        let all = Count::All;

        assert_eq!(all, Count::default(), "count should default to All");

        let counted = apply_count(&notes, one);
        assert_eq!(counted.len(), 1);

        let counted = apply_count(&notes, two);
        assert_eq!(counted.len(), 2);

        let counted = apply_count(&notes, all);
        assert_eq!(counted.len(), 6);

        let counted = apply_count(&notes, 1.into());
        assert_eq!(counted.len(), 1);
    }

    #[test]
    fn ordering() {
        use super::super::order::{OrderBy, OrderDirection, Ordering};

        let notes = create_notes();
        assert_eq!(notes.len(), 6, "create_notes should create 6 notes");

        let title_asc = Ordering::ascending(OrderBy::Title);
        let content_desc = Ordering::descending(OrderBy::Content);
        let ordered_ta = apply_order(&notes, title_asc);
        let ordered_cd = apply_order(&notes, content_desc);

        assert_eq!(ordered_ta[0].title(), "A Title");
        assert_eq!(ordered_ta[1].title(), "Gooooo Title");
        assert_eq!(ordered_ta[2].title(), "Last Title");
        assert_eq!(ordered_ta[3].title(), "Some Title");
        assert_eq!(ordered_ta[4].title(), "This is Title");
        assert_eq!(ordered_ta[5].title(), "Title Goes Here");

        assert_eq!(notes[0].content(), "The content for the note goes here.");
        assert_eq!(
            ordered_cd[0].content(),
            "Whoa, how about this note content."
        );
        // Whoa, how about this note content.
        assert_eq!(notes[2].content(), "Whoa, how about this note content.");
        assert_eq!(
            ordered_cd[2].content(),
            "The content for the note goes here."
        );
        assert_eq!(notes[5].content(), "This note has a title that does not end in the word Title, what a fkin rebel this guy is.");
        assert_eq!(
            ordered_cd[5].content(),
            "Here is content for the note the for content is Here."
        );

        let created_desc = Ordering::descending(OrderBy::Created);
        let ordered = apply_order(&notes, created_desc);
        assert_eq!(ordered[0].title(), notes[5].title());
        assert_eq!(ordered[1].title(), notes[4].title());
        assert_eq!(ordered[2].title(), notes[3].title());
        assert_eq!(ordered[3].title(), notes[2].title());
        assert_eq!(ordered[4].title(), notes[1].title());
        assert_eq!(ordered[5].title(), notes[0].title());

        let updated_asc = Ordering::ascending(OrderBy::Updated);
        let ordered = apply_order(&notes, updated_asc);
        for i in 0..6 {
            assert_eq!(ordered[i].title(), notes[i].title());
            assert_eq!(ordered[i].content(), notes[i].content());
        }
    }
}
