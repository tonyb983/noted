// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use time::OffsetDateTime;

pub mod persist;
pub mod scoped;
pub mod validation;
pub mod variadic;

#[allow(clippy::cast_sign_loss, reason = "We are verifying before casting")]
fn abs_i64(x: i64) -> u64 {
    if x < 0 {
        -x as u64
    } else {
        x as u64
    }
}

#[must_use]
pub fn humanize_timespan(from: OffsetDateTime, to: OffsetDateTime) -> impl std::fmt::Display {
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
    const AS_DAYS: std::ops::Range<u64> = DAY..(WEEK);
    const AS_WEEKS: std::ops::Range<u64> = WEEK..(MONTH);
    const AS_MONTHS: std::ops::Range<u64> = MONTH..THREE_MONTHS;
    const MINUTE_I: i64 = 60;
    const HOUR_I: i64 = const { 60 * MINUTE_I };
    const DAY_I: i64 = const { 24 * HOUR_I };
    const WEEK_I: i64 = const { 7 * DAY_I };
    const MONTH_I: i64 = const { 30 * DAY_I };
    const THREE_MONTHS_I: i64 = const { 3 * MONTH_I };

    let diff = to - from;
    let secs = diff.whole_seconds();
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
            format!("In {} second{}", secs, if secs == 1 { "" } else { "s" })
        } else {
            format!("{} second{} ago", secs, if secs == 1 { "" } else { "s" })
        }
    } else if AS_MINS.contains(&abs_secs) {
        let time = secs / MINUTE_I;
        if is_neg {
            format!("In {} minute{}", time, if time == 1 { "" } else { "s" })
        } else {
            format!("{} minute{} ago", time, if time == 1 { "" } else { "s" })
        }
    } else if AS_HOURS.contains(&abs_secs) {
        let time = secs / HOUR_I;
        if is_neg {
            format!("In {} hour{}", time, if time == 1 { "" } else { "s" })
        } else {
            format!("{} hour{} ago", time, if time == 1 { "" } else { "s" })
        }
    } else if AS_DAYS.contains(&abs_secs) {
        let time = secs / DAY_I;
        if is_neg {
            format!("In {} day{}", time, if time == 1 { "" } else { "s" })
        } else {
            format!("{} day{} ago", time, if time == 1 { "" } else { "s" })
        }
    } else if AS_WEEKS.contains(&abs_secs) {
        let time = secs / WEEK_I;
        if is_neg {
            format!("In {} week{}", time, if time == 1 { "" } else { "s" })
        } else {
            format!("{} week{} ago", time, if time == 1 { "" } else { "s" })
        }
    } else if AS_MONTHS.contains(&abs_secs) {
        let time = secs / MONTH_I;
        if is_neg {
            format!("In {} month{}", time, if time == 1 { "" } else { "s" })
        } else {
            format!("{} month{} ago", time, if time == 1 { "" } else { "s" })
        }
    } else {
        format!(
            "On {}",
            from.format(time::macros::format_description!(
                "[month]/[day]/[year] [hour]:[minute]:[second]"
            ))
            .expect("Unable to format date")
        )
    }
}

#[must_use]
pub fn humanize_to_now(dt: OffsetDateTime) -> impl std::fmt::Display {
    humanize_timespan(dt, OffsetDateTime::now_utc())
}
