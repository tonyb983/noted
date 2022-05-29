// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tinyid::TinyId;

use crate::types::HasId;

pub type ReminderDate = time::Date;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Reminder {
    id: TinyId,
    text: String,
    due: OffsetDateTime,
    #[serde(skip, default)]
    due_display: OnceCell<String>,
    has_fired: bool,
}

impl Default for Reminder {
    fn default() -> Self {
        Self {
            id: TinyId::random(),
            text: String::from("New Reminder"),
            due: OffsetDateTime::now_local()
                .expect("unable to get now_local")
                .checked_add(time::Duration::days(1))
                .expect("unable to add one day to now_local"),
            due_display: OnceCell::new(),
            has_fired: false,
        }
    }
}

impl PartialEq<Self> for Reminder {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd<Self> for Reminder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.id.cmp(&other.id))
    }
}

impl std::hash::Hash for Reminder {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Reminder {
    #[must_use]
    pub fn new<T: ToString>(text: &T, due: OffsetDateTime) -> Self {
        let has_fired = OffsetDateTime::now_utc() > due;

        Self {
            id: TinyId::random(),
            text: text.to_string(),
            due,
            due_display: OnceCell::new(),
            has_fired,
        }
    }

    #[must_use]
    pub fn existing(id: TinyId, text: String, due: OffsetDateTime, has_fired: bool) -> Self {
        Self {
            id,
            text,
            due,
            due_display: OnceCell::new(),
            has_fired,
        }
    }

    #[must_use]
    pub fn id(&self) -> TinyId {
        self.id
    }

    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    #[must_use]
    pub fn due(&self) -> &OffsetDateTime {
        &self.due
    }

    #[must_use]
    pub fn due_display(&self) -> &str {
        use crate::util::dtf;
        self.due_display
            .get_or_init(|| dtf::short_datetime(&self.due))
    }

    pub fn set_due(&mut self, due: OffsetDateTime) {
        self.due = due;
    }

    #[must_use]
    pub fn has_fired(&self) -> bool {
        self.has_fired
    }

    #[must_use]
    pub fn is_due(&mut self) -> bool {
        let now = OffsetDateTime::now_utc();

        if now > self.due {
            self.has_fired = true;
        }

        self.has_fired
    }

    pub fn fire(&mut self) {
        self.has_fired = true;
    }

    pub fn clear(&mut self) {
        self.due = OffsetDateTime::UNIX_EPOCH;
        self.has_fired = true;
        self.text.clear();
        self.id.make_null();
    }

    #[must_use]
    pub fn is_null(&self) -> bool {
        self.id.is_null()
    }
}

impl HasId for Reminder {
    fn id(&self) -> TinyId {
        self.id
    }
}
