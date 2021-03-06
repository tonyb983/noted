// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::types::Note;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OrderDirection {
    Ascending,
    Descending,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OrderBy {
    Title,
    Content,
    Tags,
    Created,
    Updated,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ordering {
    field: OrderBy,
    direction: OrderDirection,
}

impl Ordering {
    #[must_use]
    pub fn new(field: OrderBy, direction: OrderDirection) -> Self {
        Self { field, direction }
    }

    #[must_use]
    pub fn ascending(field: OrderBy) -> Self {
        Self::new(field, OrderDirection::Ascending)
    }

    #[must_use]
    pub fn descending(field: OrderBy) -> Self {
        Self::new(field, OrderDirection::Descending)
    }

    #[must_use]
    pub fn field(&self) -> OrderBy {
        self.field
    }

    #[must_use]
    pub fn direction(&self) -> OrderDirection {
        self.direction
    }

    pub fn reverse(&mut self) {
        self.direction = match self.direction {
            OrderDirection::Ascending => OrderDirection::Descending,
            OrderDirection::Descending => OrderDirection::Ascending,
        }
    }
}

impl Default for Ordering {
    fn default() -> Self {
        Self {
            field: OrderBy::Updated,
            direction: OrderDirection::Ascending,
        }
    }
}

pub type Comparison = Box<dyn Fn(&Note, &Note) -> std::cmp::Ordering>;

impl Ordering {
    #[must_use]
    pub fn comparison(self) -> Comparison {
        crate::flame_guard!("types", "api", "order", "Ordering", "comparison");
        match self.field {
            OrderBy::Title => match self.direction {
                OrderDirection::Ascending => box |a: &Note, b: &Note| a.title().cmp(b.title()),
                OrderDirection::Descending => box |a: &Note, b: &Note| b.title().cmp(a.title()),
            },
            OrderBy::Content => match self.direction {
                OrderDirection::Ascending => box |a: &Note, b: &Note| a.content().cmp(b.content()),
                OrderDirection::Descending => box |a: &Note, b: &Note| b.content().cmp(a.content()),
            },
            OrderBy::Tags => match self.direction {
                OrderDirection::Ascending => box |a: &Note, b: &Note| a.tags().cmp(b.tags()),
                OrderDirection::Descending => box |a: &Note, b: &Note| b.tags().cmp(a.tags()),
            },
            OrderBy::Created => match self.direction {
                OrderDirection::Ascending => box |a: &Note, b: &Note| a.created().cmp(b.created()),
                OrderDirection::Descending => box |a: &Note, b: &Note| b.created().cmp(a.created()),
            },
            OrderBy::Updated => match self.direction {
                OrderDirection::Ascending => box |a: &Note, b: &Note| a.updated().cmp(b.updated()),
                OrderDirection::Descending => box |a: &Note, b: &Note| b.updated().cmp(a.updated()),
            },
        }
    }
}
