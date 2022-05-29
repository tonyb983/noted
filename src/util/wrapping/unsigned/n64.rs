// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::super::{StandardOps, WrappedNumber};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WrappedU64<const MIN: u64 = { u64::MIN }, const MAX: u64 = { u64::MAX }>(u64);

impl<const MINIMUM: u64, const MAXIMUM: u64> WrappedU64<MINIMUM, MAXIMUM> {
    pub const MAX: u64 = MAXIMUM;
    pub const MIN: u64 = MINIMUM;
    pub const RANGE_SIZE: usize = {
        if cfg!(target_pointer_width = "64") {
            #[allow(clippy::cast_possible_truncation, reason = "cfg ensures this is safe")]
            {
                Self::MIN.abs_diff(Self::MAX).saturating_add(1) as usize
            }
        } else {
            match Self::MIN.abs_diff(Self::MAX).saturating_add(1).try_into() {
                Ok(value) => value,
                // TODO: Should this panic instead of potentially not working correctly?
                Err(_) => usize::MAX,
            }
        }
    };

    #[must_use]
    pub const fn new(mut value: u64) -> Self {
        loop {
            if value >= MINIMUM && value <= MAXIMUM {
                break Self(value);
            }

            if value < MINIMUM {
                value += MAXIMUM - MINIMUM + 1;
            } else {
                value -= MAXIMUM - MINIMUM + 1;
            }
        }
    }

    #[must_use]
    pub const fn create<const N: u64>() -> Self {
        Self::from_any_unsigned(N)
    }

    #[must_use]
    pub const fn from_any_unsigned(value: impl Into<u64>) -> Self {
        let mut value = value.into();
        loop {
            if value >= MINIMUM && value <= MAXIMUM {
                #[allow(
                    clippy::cast_possible_truncation,
                    reason = "we have confirmed the value is within the range"
                )]
                break Self(value as u64);
            }

            if value < MINIMUM {
                value += MAXIMUM - MINIMUM + 1;
            } else {
                value -= MAXIMUM - MINIMUM + 1;
            }
        }
    }

    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_possible_wrap,
        reason = "i dont think u64 can ever wrap when converting to isize"
    )]
    #[must_use]
    pub const fn from_any_signed(value: impl Into<isize>) -> Self {
        let mut value = value.into();
        loop {
            if value >= MINIMUM as isize && value <= MAXIMUM as isize {
                break Self(value as u64);
            }

            if value < MINIMUM as isize {
                value += MAXIMUM as isize - MINIMUM as isize + 1;
            } else {
                value -= MAXIMUM as isize - MINIMUM as isize + 1;
            }
        }
    }

    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl<const MINIMUM: u64, const MAXIMUM: u64> WrappedNumber for WrappedU64<MINIMUM, MAXIMUM> {
    type BaseType = u64;

    const RANGE_SIZE: usize = WrappedU64::<MINIMUM, MAXIMUM>::RANGE_SIZE;

    const MIN: Self::BaseType = WrappedU64::<MINIMUM, MAXIMUM>::MIN;

    const MAX: Self::BaseType = WrappedU64::<MINIMUM, MAXIMUM>::MAX;

    fn value(self) -> Self::BaseType {
        WrappedU64::value(self)
    }

    fn create(n: impl Into<Self::BaseType>) -> Self {
        WrappedU64::<MINIMUM, MAXIMUM>::new(n.into())
    }
}

impl<const MINIMUM: u64, const MAXIMUM: u64> std::ops::Add<Self> for WrappedU64<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_any_unsigned(
            self.value()
                .checked_add(rhs.value())
                .unwrap_or_else(|| self.value().wrapping_add(rhs.value())),
        )
    }
}

impl<const MINIMUM: u64, const MAXIMUM: u64> std::ops::Add<u64> for WrappedU64<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        self + Self::new(rhs)
    }
}

impl<const MINIMUM: u64, const MAXIMUM: u64> std::ops::Sub<Self> for WrappedU64<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if let Some(valid) = self.value().checked_sub(rhs.value()) {
            Self::new(valid)
        } else {
            // TODO: Validate that this is the best way to do this.
            Self::new(self.value().wrapping_sub(rhs.value()))
        }
    }
}

impl<const MINIMUM: u64, const MAXIMUM: u64> std::ops::Sub<u64> for WrappedU64<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn sub(self, rhs: u64) -> Self::Output {
        self - Self::new(rhs)
    }
}

impl<const MINIMUM: u64, const MAXIMUM: u64> std::ops::Mul<Self> for WrappedU64<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        WrappedU64::from_any_unsigned(self.value().wrapping_mul(rhs.value()))
    }
}

impl<const MINIMUM: u64, const MAXIMUM: u64> std::ops::Mul<u64> for WrappedU64<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self::Output {
        self * Self::new(rhs)
    }
}

impl<const MINIMUM: u64, const MAXIMUM: u64> std::ops::Div<Self> for WrappedU64<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        WrappedU64::from_any_unsigned(self.value().wrapping_div(rhs.value()))
    }
}

impl<const MINIMUM: u64, const MAXIMUM: u64> std::ops::Div<u64> for WrappedU64<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn div(self, rhs: u64) -> Self::Output {
        self / Self::new(rhs)
    }
}

impl<const MINIMUM: u64, const MAXIMUM: u64> std::ops::Rem<Self> for WrappedU64<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        WrappedU64::from_any_unsigned(self.value() % rhs.value())
    }
}

impl<const MINIMUM: u64, const MAXIMUM: u64> std::ops::Rem<u64> for WrappedU64<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn rem(self, rhs: u64) -> Self::Output {
        self % Self::new(rhs)
    }
}

impl<const MINIMUM: u64, const MAXIMUM: u64> std::ops::Deref for WrappedU64<MINIMUM, MAXIMUM> {
    type Target = u64;

    #[must_use]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const MINIMUM: u64, const MAXIMUM: u64> std::ops::DerefMut for WrappedU64<MINIMUM, MAXIMUM> {
    #[must_use]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const MINIMUM: u64, const MAXIMUM: u64>
    StandardOps<WrappedU64<MINIMUM, MAXIMUM>, WrappedU64<MINIMUM, MAXIMUM>>
    for WrappedU64<MINIMUM, MAXIMUM>
{
}

impl<const MINIMUM: u64, const MAXIMUM: u64> StandardOps<WrappedU64<MINIMUM, MAXIMUM>, u64>
    for WrappedU64<MINIMUM, MAXIMUM>
{
}

impl<const MINIMUM: u64, const MAXIMUM: u64> From<u8> for WrappedU64<MINIMUM, MAXIMUM> {
    #[must_use]
    fn from(n: u8) -> Self {
        WrappedU64::new(n.into())
    }
}

impl<const MINIMUM: u64, const MAXIMUM: u64> From<u16> for WrappedU64<MINIMUM, MAXIMUM> {
    #[must_use]
    fn from(n: u16) -> Self {
        WrappedU64::new(n.into())
    }
}

impl<const MINIMUM: u64, const MAXIMUM: u64> From<u32> for WrappedU64<MINIMUM, MAXIMUM> {
    #[must_use]
    fn from(n: u32) -> Self {
        WrappedU64::new(n.into())
    }
}

impl<const MINIMUM: u64, const MAXIMUM: u64> From<u64> for WrappedU64<MINIMUM, MAXIMUM> {
    #[must_use]
    fn from(n: u64) -> Self {
        WrappedU64::new(n)
    }
}

impl<const MINIMUM: u64, const MAXIMUM: u64> TryFrom<usize> for WrappedU64<MINIMUM, MAXIMUM> {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value.try_into() {
            Ok(n) => Ok(WrappedU64::new(n)),
            Err(_) => Err(()),
        }
    }
}
