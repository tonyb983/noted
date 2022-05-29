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

const fn wrap_within(mut value: u64, mut min: u64, mut max: u64) -> u64 {
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
            value = value.wrapping_add(size);
        } else {
            value = value.wrapping_sub(size);
        }
    }
}

pub use n16::WrappedU16;
pub use n32::WrappedU32;
#[cfg(target_pointer_width = "64")]
pub use n64::WrappedU64;
pub use n8::WrappedU8;
pub use nsize::WrappedUSize;
