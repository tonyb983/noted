// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

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

fn hopefully_uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[must_use]
pub fn humanize_from_now(dt: time::OffsetDateTime) -> impl std::fmt::Display {
    use std::time::Duration;
    use time::OffsetDateTime;
    use time_humanize::Humanize;
    let now = OffsetDateTime::now_utc();
    let dur = now - dt;

    hopefully_uppercase_first_letter(
        Duration::from_secs(abs_i64(dur.whole_seconds()))
            .humanize()
            .as_str(),
    )
}
