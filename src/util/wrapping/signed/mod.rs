// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod n16;
mod n32;
mod n64;
mod n8;
mod nsize;

fn abs_diff(a: isize, b: isize) -> usize {
    a.abs_diff(b)
}

const fn wrap_within(mut value: i64, mut min: i64, mut max: i64) -> i64 {
    debug_assert!(min <= max);
    if max < min {
        (min, max) = (max, min);
    }

    let size = max.abs_diff(min) + 1;
    loop {
        if value >= min && value <= max {
            break value;
        }

        if value < min {
            value = value.wrapping_add_unsigned(size);
        } else {
            value = value.wrapping_sub_unsigned(size);
        }
    }
}

pub use n8::WrappedI8;
