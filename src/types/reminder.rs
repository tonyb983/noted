// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{cell::RefCell, rc::Weak as WeakRc, sync::Weak as WeakSync};

use chrono::Datelike;
use once_cell::sync::{Lazy, OnceCell};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tinyid::TinyId;

use crate::{
    types::{
        time::{Hour, Hour12, Minute, ReminderTime, SimpleTime, TimePeriod},
        HasId,
    },
    util::dtf,
};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
struct StoredReminder {
    pub id: TinyId,
    pub text: String,
    pub due: ReminderTime,
    pub has_fired: bool,
}

impl From<StoredReminder> for Reminder {
    fn from(reminder: StoredReminder) -> Self {
        Self::from_stored(reminder)
    }
}

impl From<Reminder> for StoredReminder {
    fn from(reminder: Reminder) -> Self {
        Self {
            id: reminder.id,
            text: reminder.text,
            due: reminder.due,
            has_fired: reminder.has_fired,
        }
    }
}

fn local_offset() -> &'static time::UtcOffset {
    static LOCAL: OnceCell<time::UtcOffset> = OnceCell::new();
    LOCAL.get_or_init(|| {
        time::UtcOffset::current_local_offset().expect("unable to get local offset")
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(from = "StoredReminder", into = "StoredReminder")]
pub struct Reminder {
    id: TinyId,
    text: String,
    due: ReminderTime,
    has_fired: bool,
    date_display: String,
    time_display: String,
}

impl Default for Reminder {
    #[tracing::instrument(level = "trace")]
    fn default() -> Self {
        let due = OffsetDateTime::now_local()
            .expect("unable to get now_local")
            .checked_add(time::Duration::days(1))
            .expect("unable to add one day to now_local");

        let (date, time) = dtf::short_date_and_time(&due);
        Self {
            id: TinyId::random(),
            text: String::from("New Reminder"),
            due: ReminderTime::from_time_dt(due),
            date_display: date,
            time_display: time,
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
    #[tracing::instrument(level = "trace", skip(text), fields(text = &text.to_string().as_str()))]
    #[must_use]
    pub fn new<T: ToString>(text: &T, due: OffsetDateTime) -> Self {
        let has_fired = OffsetDateTime::now_utc() > due;

        let (date, time) = dtf::short_date_and_time(&due);
        Self {
            id: TinyId::random(),
            text: text.to_string(),
            due: ReminderTime::from_time_dt(due),
            date_display: date,
            time_display: time,
            has_fired,
        }
    }

    #[tracing::instrument(level = "trace")]
    #[must_use]
    pub fn existing(id: TinyId, text: String, due: OffsetDateTime, has_fired: bool) -> Self {
        let (date, time) = dtf::short_date_and_time(&due);
        Self {
            id,
            text,
            due: ReminderTime::from_time_dt(due),
            date_display: date,
            time_display: time,
            has_fired,
        }
    }

    fn from_stored(stored: StoredReminder) -> Self {
        let (date, time) = dtf::short_date_and_time(&stored.due.to_time_dt());
        Self {
            id: stored.id,
            text: stored.text,
            due: stored.due,
            date_display: date,
            time_display: time,
            has_fired: stored.has_fired,
        }
    }

    #[tracing::instrument(level = "trace")]
    #[must_use]
    pub fn id(&self) -> TinyId {
        self.id
    }

    #[tracing::instrument(level = "trace")]
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    #[tracing::instrument(level = "trace")]
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    #[tracing::instrument(level = "trace")]
    #[must_use]
    pub fn due(&self) -> OffsetDateTime {
        self.due.to_time_dt()
    }

    #[tracing::instrument(level = "trace")]
    #[must_use]
    pub fn date_display(&self) -> &str {
        &self.date_display
    }
    #[tracing::instrument(level = "trace")]
    #[must_use]
    pub fn time_display(&self) -> &str {
        &self.time_display
    }

    #[tracing::instrument(level = "trace")]
    pub fn set_due(&mut self, mut due: OffsetDateTime) {
        dtf::ensure_time_is_utc(&mut due);
        if due != self.due.to_time_dt() {
            self.due = ReminderTime::from_time_dt(due);
            self.update_due_display();
        }
    }

    pub fn set_due_date(&mut self, date: time::Date) {
        if date != self.due.to_time_date() {
            self.due.replace_date_t(date);
            self.update_due_display();
        }
    }

    ///
    ///
    /// ## Panics
    #[allow(clippy::cast_possible_truncation)]
    pub fn set_due_date_chrono<Tz: chrono::TimeZone>(&mut self, date: &chrono::Date<Tz>) {
        let local = date.naive_utc();
        let year = local.year();
        let ordinal = local.ordinal() as u16;
        if let Ok(date) = time::Date::from_ordinal_date(year, ordinal) {
            self.set_due_date(date);
            return;
        }

        let month = dtf::u8_to_tmonth(local.month() as _);
        let day = local.day() as u8;

        if let Ok(date) = time::Date::from_calendar_date(year, month, day) {
            self.set_due_date(date);
            return;
        }

        let iso_week = date.naive_local().iso_week();
        let year = iso_week.year() as i32;
        let week = iso_week.week() as u8;
        let weekday = date.naive_local().weekday();
        if let Ok(date) =
            time::Date::from_iso_week_date(year, week, dtf::cweekday_to_tweekday(weekday))
        {
            self.set_due_date(date);
            return;
        }

        panic!("Unable to convert chrono date to time date");
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn set_due_time(&mut self, time: time::Time, offset: time::UtcOffset) {
        use time::{ext::NumericalDuration, OffsetDateTime, UtcOffset};

        let current = self.due().to_offset(offset);
        if current.time() != time {
            current.replace_time(time);
            self.set_due(current.to_offset(time::UtcOffset::UTC));
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn set_due_time_chrono<T: chrono::Timelike>(&mut self, time: &T, offset: time::UtcOffset) {
        let hour = time.hour();
        let minute = time.minute();
        let second = time.second();
        if let Ok(time) = time::Time::from_hms(hour as _, minute as _, second as _) {
            self.set_due_time(time, offset);
        }
    }

    pub fn mutate_due(&mut self, mutator: impl FnOnce(&mut OffsetDateTime)) {
        let mut copy = self.due();
        mutator(&mut copy);
        if copy != self.due() {
            self.due = ReminderTime::from_time_dt(copy);
            self.update_due_display();
        }
    }

    #[must_use]
    pub fn get_due_time(
        &self,
    ) -> (
        super::time::Hour12,
        super::time::Minute,
        super::time::TimePeriod,
    ) {
        self.due.time.to_hmp()
    }

    #[must_use]
    pub fn get_due_date(&self) -> super::time::Date {
        self.due.date
    }

    #[must_use]
    pub fn get_due_date_chrono(&self) -> chrono::Date<chrono::Utc> {
        self.due.to_chrono_date_utc()
    }

    #[must_use]
    pub fn get_due_date_chrono_local(&self) -> chrono::Date<chrono::Local> {
        self.due.to_chrono_date_local()
    }

    pub fn set_due_hour(&mut self, hour: Hour12) {
        self.due.replace_hour(hour);
    }

    pub fn set_due_minute(&mut self, minute: Minute) {
        self.due.replace_mins(minute);
    }

    pub fn set_due_period(&mut self, period: TimePeriod) {
        self.due.replace_period(period);
    }

    pub fn set_due_hour_minute(&mut self, hour: Hour, minute: Minute) {
        self.due.time = SimpleTime::from_military(hour, minute);
    }

    pub fn set_due_hmp(&mut self, hour: Hour12, minute: Minute, period: TimePeriod) {
        self.due.time = SimpleTime::new(hour, minute, period);
    }

    #[tracing::instrument(level = "trace")]
    #[must_use]
    pub fn has_fired(&self) -> bool {
        self.has_fired
    }

    #[tracing::instrument(level = "trace")]
    #[must_use]
    pub fn is_due_utc(&self) -> bool {
        self.is_due_based_on(&OffsetDateTime::now_utc())
    }

    #[tracing::instrument(level = "trace")]
    #[must_use]
    pub fn is_due_local(&self) -> crate::Result<bool> {
        OffsetDateTime::now_local()
            .map(|now| now > self.due())
            .map_err(|err| crate::Error::Time(err.into()))
    }

    /// Determines if this [`Reminder`] is due using given `dt` as now.
    #[tracing::instrument(level = "trace")]
    #[must_use]
    pub fn is_due_based_on(&self, dt: &OffsetDateTime) -> bool {
        *dt > self.due()
    }

    #[tracing::instrument(level = "trace")]
    pub fn fire(&mut self) {
        self.has_fired = true;
    }

    #[tracing::instrument(level = "trace")]
    pub fn clear(&mut self) {
        self.due = ReminderTime::epoch();
        self.has_fired = true;
        self.text.clear();
        self.id.make_null();
        self.date_display.clear();
        self.time_display.clear();
    }

    #[tracing::instrument(level = "trace")]
    #[must_use]
    pub fn is_null(&self) -> bool {
        self.id.is_null()
    }

    fn update_due_display(&mut self) {
        use once_cell::sync::OnceCell;
        let local = local_offset();
        let (date, time) = dtf::short_date_and_time(&self.due.to_time_dt().to_offset(*local));
        self.date_display = date;
        self.time_display = time;
    }
}

impl HasId for Reminder {
    fn id(&self) -> TinyId {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne, assert_str_eq};

    #[allow(clippy::cast_lossless)]
    #[test]
    #[cfg_attr(coverage, no_coverage)]
    fn it_works() {
        use std::sync::{Arc, Mutex};
        use time::{ext::NumericalDuration, OffsetDateTime, UtcOffset};

        let now_utc = OffsetDateTime::now_utc();
        let now_local = OffsetDateTime::now_local().unwrap();

        println!("UTC: {}", now_utc);
        println!("LOC: {}", now_local);

        println!("UTC Offset: {:?}", now_utc.offset());
        println!("LOC Offset: {:?}", now_local.offset());

        let (h, m, s) = now_local.offset().as_hms();
        let new_time = now_local
            .saturating_sub((h as i64).hours())
            .saturating_sub((m as i64).minutes())
            .saturating_sub((s as i64).seconds());

        println!("UTC: {}", now_utc);
        println!("NEW: {}", new_time);

        let mut string = Arc::new(Mutex::new(String::from("Hello World")));
        let mut clone = Arc::clone(&string);
        let func = move || {
            println!("{}", clone.lock().expect("Unable to print from closure"));
            if let Ok(ref mut string) = clone.lock() {
                if let Some(pos) = string.find(' ') {
                    string.replace_range(0..pos, "Goodbye");
                } else {
                    panic!("Unable to find space");
                }
            } else {
                panic!("Unable to get string");
            }
            clone
        };
        func();
        println!("{}", string.lock().expect("Unable to print from main"));
    }
}
