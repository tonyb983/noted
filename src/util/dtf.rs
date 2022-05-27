// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use time::{format_description::FormatItem, OffsetDateTime};

#[allow(clippy::cast_sign_loss, reason = "We are verifying before casting")]
fn abs_i64(x: i64) -> u64 {
    crate::flame_guard!("util", "dtf", "abs_i64");
    if x < 0 {
        -x as u64
    } else {
        x as u64
    }
}

#[must_use]
pub fn humanize_timespan_to_now(dt: OffsetDateTime) -> impl std::fmt::Display {
    crate::flame_guard!("util", "dtf", "humanize_timespan_to_now");
    humanize_timespan_between(dt, OffsetDateTime::now_utc())
}

#[must_use]
pub fn humanize_timespan_from_now(dt: OffsetDateTime) -> impl std::fmt::Display {
    crate::flame_guard!("util", "dtf", "humanize_timespan_from_now");
    humanize_timespan_between(OffsetDateTime::now_utc(), dt)
}

#[must_use]
pub fn humanize_timespan_between(
    from: OffsetDateTime,
    to: OffsetDateTime,
) -> impl std::fmt::Display {
    crate::flame_guard!("util", "dtf", "humanize_timespan_between");
    let diff = to - from;
    humanize_timespan(diff)
}

#[must_use]
pub fn humanize_timespan(dur: time::Duration) -> impl std::fmt::Display {
    const MINUTE: u64 = 60;
    const HOUR: u64 = const { 60 * MINUTE };
    const DAY: u64 = const { 24 * HOUR };
    const WEEK: u64 = const { 7 * DAY };
    const MONTH: u64 = const { 30 * DAY };
    const THREE_MONTHS: u64 = const { 3 * MONTH };
    const JUST_NOW: std::ops::Range<u64> = 0..6;
    const AS_SECS: std::ops::Range<u64> = 6..MINUTE;
    const AS_MINS: std::ops::Range<u64> = MINUTE..HOUR;
    const AS_HOURS: std::ops::Range<u64> = HOUR..DAY;
    const AS_DAYS: std::ops::Range<u64> = DAY..WEEK;
    const AS_WEEKS: std::ops::Range<u64> = WEEK..MONTH;
    const AS_MONTHS: std::ops::Range<u64> = MONTH..THREE_MONTHS;

    crate::flame_guard!("util", "dtf", "humanize_timespan");

    let secs = dur.whole_seconds();
    let abs_secs = abs_i64(secs);
    let is_neg = secs < 0;

    if JUST_NOW.contains(&abs_secs) {
        if is_neg {
            "Imminently".to_string()
        } else {
            "Just now".to_string()
        }
    } else if AS_SECS.contains(&abs_secs) {
        if is_neg {
            format!(
                "In {} second{}",
                abs_secs,
                if abs_secs == 1 { "" } else { "s" }
            )
        } else {
            format!(
                "{} second{} ago",
                abs_secs,
                if abs_secs == 1 { "" } else { "s" }
            )
        }
    } else if AS_MINS.contains(&abs_secs) {
        let time = abs_secs / MINUTE;
        if is_neg {
            format!("In {} minute{}", time, if time == 1 { "" } else { "s" })
        } else {
            format!("{} minute{} ago", time, if time == 1 { "" } else { "s" })
        }
    } else if AS_HOURS.contains(&abs_secs) {
        let time = abs_secs / HOUR;
        if is_neg {
            format!("In {} hour{}", time, if time == 1 { "" } else { "s" })
        } else {
            format!("{} hour{} ago", time, if time == 1 { "" } else { "s" })
        }
    } else if AS_DAYS.contains(&abs_secs) {
        let time = abs_secs / DAY;
        if is_neg {
            format!("In {} day{}", time, if time == 1 { "" } else { "s" })
        } else {
            format!("{} day{} ago", time, if time == 1 { "" } else { "s" })
        }
    } else if AS_WEEKS.contains(&abs_secs) {
        let time = abs_secs / WEEK;
        if is_neg {
            format!("In {} week{}", time, if time == 1 { "" } else { "s" })
        } else {
            format!("{} week{} ago", time, if time == 1 { "" } else { "s" })
        }
    } else if AS_MONTHS.contains(&abs_secs) {
        let time = abs_secs / MONTH;
        if is_neg {
            format!("In {} month{}", time, if time == 1 { "" } else { "s" })
        } else {
            format!("{} month{} ago", time, if time == 1 { "" } else { "s" })
        }
    } else {
        /// TODO: Make this better
        dur.to_string()
    }
}

pub fn timestamp_to_string(timestamp: &OffsetDateTime) -> String {
    use once_cell::sync::OnceCell;
    static FORMAT: OnceCell<&'static [FormatItem<'static>]> = OnceCell::new();
    let format = FORMAT.get_or_init(|| {
        time::macros::format_description!(
            "[month]-[day]-[year repr:last_two] [hour repr:12]:[minute]:[second][period]"
        )
    });
    timestamp
        .format(format)
        .expect("Unable to format timestamp")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne, assert_str_eq};
    use time::macros::datetime;

    #[test]
    #[no_coverage]
    fn seconds() {
        // Test "just now"
        let output = humanize_timespan_between(
            datetime!(2020-06-15 9:59:59 UTC),
            datetime!(2020-06-15 10:00 UTC),
        )
        .to_string();
        assert_eq!(output, "Just now");
        let output = humanize_timespan_between(
            datetime!(2020-06-15 10:00 UTC),
            datetime!(2020-06-15 9:59:59 UTC),
        )
        .to_string();
        assert_eq!(output, "Imminently");

        // Test seconds
        let output = humanize_timespan_between(
            datetime!(2020-06-15 9:59:30 UTC),
            datetime!(2020-06-15 10:00 UTC),
        )
        .to_string();
        assert_eq!(output, "30 seconds ago");
        let output = humanize_timespan_between(
            datetime!(2020-06-15 10:00 UTC),
            datetime!(2020-06-15 9:59:30 UTC),
        )
        .to_string();
        assert_eq!(output, "In 30 seconds");
    }

    #[test]
    fn minutes() {
        // Test minute
        let output = humanize_timespan_between(
            datetime!(2020-06-15 9:59 UTC),
            datetime!(2020-06-15 10:00 UTC),
        )
        .to_string();
        assert_eq!(output, "1 minute ago");
        let output = humanize_timespan_between(
            datetime!(2020-06-15 10:00 UTC),
            datetime!(2020-06-15 9:59 UTC),
        )
        .to_string();
        assert_eq!(output, "In 1 minute");
        // Test minutes
        let output = humanize_timespan_between(
            datetime!(2020-06-15 9:55 UTC),
            datetime!(2020-06-15 10:00 UTC),
        )
        .to_string();
        assert_eq!(output, "5 minutes ago");
        let output = humanize_timespan_between(
            datetime!(2020-06-15 10:00 UTC),
            datetime!(2020-06-15 9:55 UTC),
        )
        .to_string();
        assert_eq!(output, "In 5 minutes");
    }

    #[test]
    fn hours() {
        // Test hour
        let output = humanize_timespan_between(
            datetime!(2020-06-15 9:00 UTC),
            datetime!(2020-06-15 10:00 UTC),
        )
        .to_string();
        assert_eq!(output, "1 hour ago");
        let output = humanize_timespan_between(
            datetime!(2020-06-15 10:00 UTC),
            datetime!(2020-06-15 9:00 UTC),
        )
        .to_string();
        assert_eq!(output, "In 1 hour");
        // Test hours
        let output = humanize_timespan_between(
            datetime!(2020-06-15 8:00 UTC),
            datetime!(2020-06-15 10:00 UTC),
        )
        .to_string();
        assert_eq!(output, "2 hours ago");
        let output = humanize_timespan_between(
            datetime!(2020-06-15 10:00 UTC),
            datetime!(2020-06-15 8:00 UTC),
        )
        .to_string();
        assert_eq!(output, "In 2 hours");
    }

    #[test]
    fn days() {
        // Test day
        let output = humanize_timespan_between(
            datetime!(2020-06-14 10:00 UTC),
            datetime!(2020-06-15 10:00 UTC),
        )
        .to_string();
        assert_eq!(output, "1 day ago");
        let output = humanize_timespan_between(
            datetime!(2020-06-15 10:00 UTC),
            datetime!(2020-06-14 10:00 UTC),
        )
        .to_string();
        assert_eq!(output, "In 1 day");
        // Test days
        let output = humanize_timespan_between(
            datetime!(2020-06-13 10:00 UTC),
            datetime!(2020-06-15 10:00 UTC),
        )
        .to_string();
        assert_eq!(output, "2 days ago");
        let output = humanize_timespan_between(
            datetime!(2020-06-15 10:00 UTC),
            datetime!(2020-06-13 10:00 UTC),
        )
        .to_string();
        assert_eq!(output, "In 2 days");
    }

    #[test]
    fn weeks() {
        // Test week
        let output = humanize_timespan_between(
            datetime!(2020-06-08 10:00 UTC),
            datetime!(2020-06-15 10:00 UTC),
        )
        .to_string();
        assert_eq!(output, "1 week ago");
        let output = humanize_timespan_between(
            datetime!(2020-06-15 10:00 UTC),
            datetime!(2020-06-08 10:00 UTC),
        )
        .to_string();
        assert_eq!(output, "In 1 week");
        // Test weeks
        let output = humanize_timespan_between(
            datetime!(2020-06-01 10:00 UTC),
            datetime!(2020-06-15 10:00 UTC),
        )
        .to_string();
        assert_eq!(output, "2 weeks ago");
        let output = humanize_timespan_between(
            datetime!(2020-06-15 10:00 UTC),
            datetime!(2020-06-01 10:00 UTC),
        )
        .to_string();
        assert_eq!(output, "In 2 weeks");
    }

    #[test]
    fn months() {
        // Test week
        let output = humanize_timespan_between(
            datetime!(2020-05-14 10:00 UTC),
            datetime!(2020-06-15 10:00 UTC),
        )
        .to_string();
        assert_eq!(output, "1 month ago");
        let output = humanize_timespan_between(
            datetime!(2020-06-15 10:00 UTC),
            datetime!(2020-05-08 10:00 UTC),
        )
        .to_string();
        assert_eq!(output, "In 1 month");
        // Test weeks
        let output = humanize_timespan_between(
            datetime!(2020-04-01 10:00 UTC),
            datetime!(2020-06-15 10:00 UTC),
        )
        .to_string();
        assert_eq!(output, "2 months ago");
        let output = humanize_timespan_between(
            datetime!(2020-06-15 10:00 UTC),
            datetime!(2020-04-01 10:00 UTC),
        )
        .to_string();
        assert_eq!(output, "In 2 months");
    }

    #[test]
    fn long_time() {
        let output = humanize_timespan_between(
            datetime!(2018-04-01 10:00 UTC),
            datetime!(2020-06-15 10:00 UTC),
        )
        .to_string();
        assert_eq!(output, "806d");
    }
}
