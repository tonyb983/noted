// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::super::{StandardOps, WrappedNumber};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WrappedU32<const MIN: u32 = { u32::MIN }, const MAX: u32 = { u32::MAX }>(u32);

impl<const MINIMUM: u32, const MAXIMUM: u32> WrappedU32<MINIMUM, MAXIMUM> {
    pub const MAX: u32 = MAXIMUM;
    pub const MIN: u32 = MINIMUM;
    pub const RANGE_SIZE: usize = Self::MIN.abs_diff(Self::MAX) as usize + 1;

    #[must_use]
    pub const fn new(mut value: u32) -> Self {
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
    pub const fn create<const N: usize>() -> Self {
        Self::from_any_unsigned(N)
    }

    #[must_use]
    pub const fn from_any_unsigned(value: impl Into<usize>) -> Self {
        let mut value = value.into();
        loop {
            if value >= MINIMUM as usize && value <= MAXIMUM as usize {
                #[allow(
                    clippy::cast_possible_truncation,
                    reason = "we have confirmed the value is within the range"
                )]
                break Self(value as u32);
            }

            if value < MINIMUM as usize {
                value += MAXIMUM as usize - MINIMUM as usize + 1;
            } else {
                value -= MAXIMUM as usize - MINIMUM as usize + 1;
            }
        }
    }

    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_possible_wrap,
        reason = "i dont think u32 can ever wrap when converting to isize"
    )]
    #[must_use]
    pub const fn from_any_signed(value: impl Into<isize>) -> Self {
        let mut value = value.into();
        loop {
            if value >= MINIMUM as isize && value <= MAXIMUM as isize {
                break Self(value as u32);
            }

            if value < MINIMUM as isize {
                value += MAXIMUM as isize - MINIMUM as isize + 1;
            } else {
                value -= MAXIMUM as isize - MINIMUM as isize + 1;
            }
        }
    }

    #[must_use]
    pub const fn value(self) -> u32 {
        self.0
    }
}

impl<const MINIMUM: u32, const MAXIMUM: u32> WrappedNumber for WrappedU32<MINIMUM, MAXIMUM> {
    type BaseType = u32;

    const RANGE_SIZE: usize = WrappedU32::<MINIMUM, MAXIMUM>::RANGE_SIZE;

    const MIN: Self::BaseType = WrappedU32::<MINIMUM, MAXIMUM>::MIN;

    const MAX: Self::BaseType = WrappedU32::<MINIMUM, MAXIMUM>::MAX;

    fn value(self) -> Self::BaseType {
        WrappedU32::value(self)
    }

    fn create(n: impl Into<Self::BaseType>) -> Self {
        WrappedU32::<MINIMUM, MAXIMUM>::new(n.into())
    }
}

impl<const MINIMUM: u32, const MAXIMUM: u32> std::ops::Add<Self> for WrappedU32<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_any_unsigned(self.value() as usize + rhs.value() as usize)
    }
}

impl<const MINIMUM: u32, const MAXIMUM: u32> std::ops::Add<u32> for WrappedU32<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn add(self, rhs: u32) -> Self::Output {
        self + Self::new(rhs)
    }
}

impl<const MINIMUM: u32, const MAXIMUM: u32> std::ops::Sub<Self> for WrappedU32<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        #[allow(clippy::cast_possible_wrap)]
        WrappedU32::from_any_signed((self.value() as isize) - rhs.value() as isize)
    }
}

impl<const MINIMUM: u32, const MAXIMUM: u32> std::ops::Sub<u32> for WrappedU32<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn sub(self, rhs: u32) -> Self::Output {
        self - Self::new(rhs)
    }
}

impl<const MINIMUM: u32, const MAXIMUM: u32> std::ops::Mul<Self> for WrappedU32<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        WrappedU32::from_any_unsigned(self.value() as usize * rhs.value() as usize)
    }
}

impl<const MINIMUM: u32, const MAXIMUM: u32> std::ops::Mul<u32> for WrappedU32<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self::Output {
        self * Self::new(rhs)
    }
}

impl<const MINIMUM: u32, const MAXIMUM: u32> std::ops::Div<Self> for WrappedU32<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        WrappedU32::from_any_unsigned(self.value() as usize / rhs.value() as usize)
    }
}

impl<const MINIMUM: u32, const MAXIMUM: u32> std::ops::Div<u32> for WrappedU32<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn div(self, rhs: u32) -> Self::Output {
        self / Self::new(rhs)
    }
}

impl<const MINIMUM: u32, const MAXIMUM: u32> std::ops::Rem<Self> for WrappedU32<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        WrappedU32::from_any_unsigned(self.value() as usize % rhs.value() as usize)
    }
}

impl<const MINIMUM: u32, const MAXIMUM: u32> std::ops::Rem<u32> for WrappedU32<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn rem(self, rhs: u32) -> Self::Output {
        self % Self::new(rhs)
    }
}

impl<const MINIMUM: u32, const MAXIMUM: u32> std::ops::Deref for WrappedU32<MINIMUM, MAXIMUM> {
    type Target = u32;

    #[must_use]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const MINIMUM: u32, const MAXIMUM: u32> std::ops::DerefMut for WrappedU32<MINIMUM, MAXIMUM> {
    #[must_use]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const MINIMUM: u32, const MAXIMUM: u32>
    StandardOps<WrappedU32<MINIMUM, MAXIMUM>, WrappedU32<MINIMUM, MAXIMUM>>
    for WrappedU32<MINIMUM, MAXIMUM>
{
}

impl<const MINIMUM: u32, const MAXIMUM: u32> StandardOps<WrappedU32<MINIMUM, MAXIMUM>, u32>
    for WrappedU32<MINIMUM, MAXIMUM>
{
}

impl<const MINIMUM: u32, const MAXIMUM: u32> From<u8> for WrappedU32<MINIMUM, MAXIMUM> {
    #[must_use]
    fn from(n: u8) -> Self {
        WrappedU32::new(n.into())
    }
}

impl<const MINIMUM: u32, const MAXIMUM: u32> From<u16> for WrappedU32<MINIMUM, MAXIMUM> {
    #[must_use]
    fn from(n: u16) -> Self {
        WrappedU32::new(n.into())
    }
}

impl<const MINIMUM: u32, const MAXIMUM: u32> From<u32> for WrappedU32<MINIMUM, MAXIMUM> {
    #[must_use]
    fn from(n: u32) -> Self {
        WrappedU32::new(n)
    }
}

impl<const MINIMUM: u32, const MAXIMUM: u32> From<u64> for WrappedU32<MINIMUM, MAXIMUM> {
    #[must_use]
    fn from(n: u64) -> Self {
        #[allow(
            clippy::cast_possible_truncation,
            reason = "we ensure value is in range"
        )]
        let wrapped = super::wrap_within(n, Self::MIN.into(), Self::MAX.into()) as u32;
        WrappedU32::new(wrapped)
    }
}

impl<const MINIMUM: u32, const MAXIMUM: u32> From<usize> for WrappedU32<MINIMUM, MAXIMUM> {
    #[must_use]
    fn from(n: usize) -> Self {
        #[allow(
            clippy::cast_possible_truncation,
            reason = "we ensure value is in range"
        )]
        let wrapped = super::wrap_within(n as u64, Self::MIN.into(), Self::MAX.into()) as u32;
        WrappedU32::new(wrapped)
    }
}
