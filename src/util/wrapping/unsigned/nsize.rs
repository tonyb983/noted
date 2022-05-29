// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::super::{StandardOps, WrappedNumber};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WrappedUSize<const MIN: usize = { usize::MIN }, const MAX: usize = { usize::MAX }>(
    usize,
);

impl<const MINIMUM: usize, const MAXIMUM: usize> WrappedUSize<MINIMUM, MAXIMUM> {
    pub const MAX: usize = MAXIMUM;
    pub const MIN: usize = MINIMUM;
    pub const RANGE_SIZE: usize = Self::MIN.abs_diff(Self::MAX) as usize + 1;

    #[must_use]
    pub const fn new(mut value: usize) -> Self {
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
                break Self(value as usize);
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
        reason = "i dont think usize can ever wrap when converting to isize"
    )]
    #[must_use]
    pub const fn from_any_signed(value: impl Into<isize>) -> Self {
        let mut value = value.into();
        loop {
            if value >= MINIMUM as isize && value <= MAXIMUM as isize {
                break Self(value as usize);
            }

            if value < MINIMUM as isize {
                value += MAXIMUM as isize - MINIMUM as isize + 1;
            } else {
                value -= MAXIMUM as isize - MINIMUM as isize + 1;
            }
        }
    }

    #[must_use]
    pub const fn value(self) -> usize {
        self.0
    }
}

impl<const MINIMUM: usize, const MAXIMUM: usize> WrappedNumber for WrappedUSize<MINIMUM, MAXIMUM> {
    type BaseType = usize;

    const RANGE_SIZE: usize = WrappedUSize::<MINIMUM, MAXIMUM>::RANGE_SIZE;

    const MIN: Self::BaseType = WrappedUSize::<MINIMUM, MAXIMUM>::MIN;

    const MAX: Self::BaseType = WrappedUSize::<MINIMUM, MAXIMUM>::MAX;

    fn value(self) -> Self::BaseType {
        WrappedUSize::value(self)
    }

    fn create(n: impl Into<Self::BaseType>) -> Self {
        WrappedUSize::<MINIMUM, MAXIMUM>::new(n.into())
    }
}

impl<const MINIMUM: usize, const MAXIMUM: usize> std::ops::Add<Self>
    for WrappedUSize<MINIMUM, MAXIMUM>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_any_unsigned(self.value() as usize + rhs.value() as usize)
    }
}

impl<const MINIMUM: usize, const MAXIMUM: usize> std::ops::Add<usize>
    for WrappedUSize<MINIMUM, MAXIMUM>
{
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        self + Self::new(rhs)
    }
}

impl<const MINIMUM: usize, const MAXIMUM: usize> std::ops::Sub<Self>
    for WrappedUSize<MINIMUM, MAXIMUM>
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        #[allow(clippy::cast_possible_wrap)]
        WrappedUSize::from_any_signed((self.value() as isize) - rhs.value() as isize)
    }
}

impl<const MINIMUM: usize, const MAXIMUM: usize> std::ops::Sub<usize>
    for WrappedUSize<MINIMUM, MAXIMUM>
{
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        self - Self::new(rhs)
    }
}

impl<const MINIMUM: usize, const MAXIMUM: usize> std::ops::Mul<Self>
    for WrappedUSize<MINIMUM, MAXIMUM>
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        WrappedUSize::from_any_unsigned(self.value() as usize * rhs.value() as usize)
    }
}

impl<const MINIMUM: usize, const MAXIMUM: usize> std::ops::Mul<usize>
    for WrappedUSize<MINIMUM, MAXIMUM>
{
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        self * Self::new(rhs)
    }
}

impl<const MINIMUM: usize, const MAXIMUM: usize> std::ops::Div<Self>
    for WrappedUSize<MINIMUM, MAXIMUM>
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        WrappedUSize::from_any_unsigned(self.value() as usize / rhs.value() as usize)
    }
}

impl<const MINIMUM: usize, const MAXIMUM: usize> std::ops::Div<usize>
    for WrappedUSize<MINIMUM, MAXIMUM>
{
    type Output = Self;

    fn div(self, rhs: usize) -> Self::Output {
        self / Self::new(rhs)
    }
}

impl<const MINIMUM: usize, const MAXIMUM: usize> std::ops::Rem<Self>
    for WrappedUSize<MINIMUM, MAXIMUM>
{
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        WrappedUSize::from_any_unsigned(self.value() as usize % rhs.value() as usize)
    }
}

impl<const MINIMUM: usize, const MAXIMUM: usize> std::ops::Rem<usize>
    for WrappedUSize<MINIMUM, MAXIMUM>
{
    type Output = Self;

    fn rem(self, rhs: usize) -> Self::Output {
        self % Self::new(rhs)
    }
}

impl<const MINIMUM: usize, const MAXIMUM: usize> std::ops::Deref
    for WrappedUSize<MINIMUM, MAXIMUM>
{
    type Target = usize;

    #[must_use]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const MINIMUM: usize, const MAXIMUM: usize> std::ops::DerefMut
    for WrappedUSize<MINIMUM, MAXIMUM>
{
    #[must_use]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const MINIMUM: usize, const MAXIMUM: usize>
    StandardOps<WrappedUSize<MINIMUM, MAXIMUM>, WrappedUSize<MINIMUM, MAXIMUM>>
    for WrappedUSize<MINIMUM, MAXIMUM>
{
}

impl<const MINIMUM: usize, const MAXIMUM: usize> StandardOps<WrappedUSize<MINIMUM, MAXIMUM>, usize>
    for WrappedUSize<MINIMUM, MAXIMUM>
{
}

impl<const MINIMUM: usize, const MAXIMUM: usize> From<u8> for WrappedUSize<MINIMUM, MAXIMUM> {
    #[must_use]
    fn from(n: u8) -> Self {
        Self::new(n.into())
    }
}

impl<const MINIMUM: usize, const MAXIMUM: usize> From<u16> for WrappedUSize<MINIMUM, MAXIMUM> {
    #[must_use]
    fn from(n: u16) -> Self {
        Self::new(n.into())
    }
}

impl<const MINIMUM: usize, const MAXIMUM: usize> From<u32> for WrappedUSize<MINIMUM, MAXIMUM> {
    #[must_use]
    fn from(n: u32) -> Self {
        #[cfg(target_pointer_width = "64")]
        {
            return Self::new(n as usize);
        }

        #[allow(
            clippy::cast_possible_truncation,
            reason = "we ensure value is in range"
        )]
        let wrapped = super::wrap_within(n.into(), Self::MIN as u64, Self::MAX as u64) as usize;
        Self::new(wrapped)
    }
}

impl<const MINIMUM: usize, const MAXIMUM: usize> From<u64> for WrappedUSize<MINIMUM, MAXIMUM> {
    #[must_use]
    fn from(n: u64) -> Self {
        #[cfg(target_pointer_width = "64")]
        {
            #[allow(clippy::cast_possible_truncation, reason = "cfg ensures this is valid")]
            return Self::new(n as usize);
        }

        #[allow(
            clippy::cast_possible_truncation,
            reason = "we ensure value is in range"
        )]
        let wrapped = super::wrap_within(n, Self::MIN as u64, Self::MAX as u64) as usize;
        Self::new(wrapped)
    }
}

impl<const MINIMUM: usize, const MAXIMUM: usize> From<usize> for WrappedUSize<MINIMUM, MAXIMUM> {
    #[must_use]
    fn from(n: usize) -> Self {
        Self::new(n)
    }
}
