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
    pub(crate) fn comparison(self) -> Comparison {
        match self.field {
            OrderBy::Title => match self.direction {
                OrderDirection::Ascending => box |a, b| a.title().cmp(b.title()),
                OrderDirection::Descending => box |a, b| b.title().cmp(a.title()),
            },
            OrderBy::Content => match self.direction {
                OrderDirection::Ascending => box |a, b| a.content().cmp(b.content()),
                OrderDirection::Descending => box |a, b| b.content().cmp(a.content()),
            },
            OrderBy::Tags => match self.direction {
                OrderDirection::Ascending => box |a, b| a.tags().cmp(b.tags()),
                OrderDirection::Descending => box |a, b| b.tags().cmp(a.tags()),
            },
            OrderBy::Created => match self.direction {
                OrderDirection::Ascending => box |a, b| a.created().cmp(b.created()),
                OrderDirection::Descending => box |a, b| b.created().cmp(a.created()),
            },
            OrderBy::Updated => match self.direction {
                OrderDirection::Ascending => box |a, b| a.updated().cmp(b.updated()),
                OrderDirection::Descending => box |a, b| b.updated().cmp(a.updated()),
            },
        }
    }
}
