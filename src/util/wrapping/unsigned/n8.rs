// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::super::{StandardOps, WrappedNumber};

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Deserialize, serde::Serialize,
)]
pub struct WrappedU8<const MIN: u8 = { u8::MIN }, const MAX: u8 = { u8::MAX }>(u8);

impl<const MINIMUM: u8, const MAXIMUM: u8> WrappedU8<MINIMUM, MAXIMUM> {
    pub const MAX: u8 = MAXIMUM;
    pub const MIN: u8 = MINIMUM;
    pub const RANGE_SIZE: usize = Self::MIN.abs_diff(Self::MAX) as usize + 1;

    #[must_use]
    pub const fn new(mut value: u8) -> Self {
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
                break Self(value as u8);
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
        reason = "i dont think u8 can ever wrap when converting to isize"
    )]
    #[must_use]
    pub const fn from_any_signed(value: impl Into<isize>) -> Self {
        let mut value = value.into();
        loop {
            if value >= MINIMUM as isize && value <= MAXIMUM as isize {
                break Self(value as u8);
            }

            if value < MINIMUM as isize {
                value += MAXIMUM as isize - MINIMUM as isize + 1;
            } else {
                value -= MAXIMUM as isize - MINIMUM as isize + 1;
            }
        }
    }

    #[must_use]
    pub const fn value(self) -> u8 {
        self.0
    }
}

impl<const MINIMUM: u8, const MAXIMUM: u8> WrappedNumber for WrappedU8<MINIMUM, MAXIMUM> {
    type BaseType = u8;

    const RANGE_SIZE: usize = WrappedU8::<MINIMUM, MAXIMUM>::RANGE_SIZE;

    const MIN: Self::BaseType = WrappedU8::<MINIMUM, MAXIMUM>::MIN;

    const MAX: Self::BaseType = WrappedU8::<MINIMUM, MAXIMUM>::MAX;

    fn value(self) -> Self::BaseType {
        WrappedU8::value(self)
    }

    fn create(n: impl Into<Self::BaseType>) -> Self {
        WrappedU8::<MINIMUM, MAXIMUM>::new(n.into())
    }
}

impl<const MINIMUM: u8, const MAXIMUM: u8> std::ops::Add<Self> for WrappedU8<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_any_unsigned(self.value() as usize + rhs.value() as usize)
    }
}

impl<const MINIMUM: u8, const MAXIMUM: u8> std::ops::Add<u8> for WrappedU8<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn add(self, rhs: u8) -> Self::Output {
        self + Self::new(rhs)
    }
}

impl<const MINIMUM: u8, const MAXIMUM: u8> std::ops::Sub<Self> for WrappedU8<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        #[allow(clippy::cast_possible_wrap)]
        WrappedU8::from_any_signed((self.value() as isize) - rhs.value() as isize)
    }
}

impl<const MINIMUM: u8, const MAXIMUM: u8> std::ops::Sub<u8> for WrappedU8<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn sub(self, rhs: u8) -> Self::Output {
        self - Self::new(rhs)
    }
}

impl<const MINIMUM: u8, const MAXIMUM: u8> std::ops::Mul<Self> for WrappedU8<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        WrappedU8::from_any_unsigned(self.value() as usize * rhs.value() as usize)
    }
}

impl<const MINIMUM: u8, const MAXIMUM: u8> std::ops::Mul<u8> for WrappedU8<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn mul(self, rhs: u8) -> Self::Output {
        self * Self::new(rhs)
    }
}

impl<const MINIMUM: u8, const MAXIMUM: u8> std::ops::Div<Self> for WrappedU8<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        WrappedU8::from_any_unsigned(self.value() as usize / rhs.value() as usize)
    }
}

impl<const MINIMUM: u8, const MAXIMUM: u8> std::ops::Div<u8> for WrappedU8<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn div(self, rhs: u8) -> Self::Output {
        self / Self::new(rhs)
    }
}

impl<const MINIMUM: u8, const MAXIMUM: u8> std::ops::Rem<Self> for WrappedU8<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        WrappedU8::from_any_unsigned(self.value() as usize % rhs.value() as usize)
    }
}

impl<const MINIMUM: u8, const MAXIMUM: u8> std::ops::Rem<u8> for WrappedU8<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn rem(self, rhs: u8) -> Self::Output {
        self % Self::new(rhs)
    }
}

impl<const MINIMUM: u8, const MAXIMUM: u8> std::ops::Deref for WrappedU8<MINIMUM, MAXIMUM> {
    type Target = u8;

    #[must_use]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const MINIMUM: u8, const MAXIMUM: u8> std::ops::DerefMut for WrappedU8<MINIMUM, MAXIMUM> {
    #[must_use]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const MINIMUM: u8, const MAXIMUM: u8>
    StandardOps<WrappedU8<MINIMUM, MAXIMUM>, WrappedU8<MINIMUM, MAXIMUM>>
    for WrappedU8<MINIMUM, MAXIMUM>
{
}

impl<const MINIMUM: u8, const MAXIMUM: u8> StandardOps<WrappedU8<MINIMUM, MAXIMUM>, u8>
    for WrappedU8<MINIMUM, MAXIMUM>
{
}

impl<const MINIMUM: u8, const MAXIMUM: u8> From<u8> for WrappedU8<MINIMUM, MAXIMUM> {
    #[must_use]
    fn from(n: u8) -> Self {
        Self::new(n)
    }
}
impl<const MINIMUM: u8, const MAXIMUM: u8> From<u16> for WrappedU8<MINIMUM, MAXIMUM> {
    #[must_use]
    fn from(n: u16) -> Self {
        #[allow(
            clippy::cast_possible_truncation,
            reason = "we ensure value is in range"
        )]
        let wrapped = super::wrap_within(n.into(), Self::MIN.into(), Self::MAX.into()) as u8;
        Self::new(wrapped)
    }
}
impl<const MINIMUM: u8, const MAXIMUM: u8> From<u32> for WrappedU8<MINIMUM, MAXIMUM> {
    #[must_use]
    fn from(n: u32) -> Self {
        #[allow(
            clippy::cast_possible_truncation,
            reason = "we ensure value is in range"
        )]
        let wrapped = super::wrap_within(n.into(), Self::MIN.into(), Self::MAX.into()) as u8;
        Self::new(wrapped)
    }
}
impl<const MINIMUM: u8, const MAXIMUM: u8> From<u64> for WrappedU8<MINIMUM, MAXIMUM> {
    #[must_use]
    fn from(n: u64) -> Self {
        #[allow(
            clippy::cast_possible_truncation,
            reason = "we ensure value is in range"
        )]
        let wrapped = super::wrap_within(n, Self::MIN.into(), Self::MAX.into()) as u8;
        Self::new(wrapped)
    }
}
impl<const MINIMUM: u8, const MAXIMUM: u8> From<usize> for WrappedU8<MINIMUM, MAXIMUM> {
    #[must_use]
    fn from(n: usize) -> Self {
        #[allow(
            clippy::cast_possible_truncation,
            reason = "we ensure value is in range"
        )]
        let wrapped = super::wrap_within(n as u64, Self::MIN.into(), Self::MAX.into()) as u8;
        Self::new(wrapped)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne, assert_str_eq};

    #[test]
    #[no_coverage]
    fn ctor01() {
        type Type = WrappedU8<0, 1>;
        let tester = Type::new(0);
        assert!(tester.value() == 0 || tester.value() == 1);
        let tester = Type::new(10);
        assert!(tester.value() == 0 || tester.value() == 1);
        let tester = Type::new(11);
        assert!(tester.value() == 0 || tester.value() == 1);
        let tester = Type::new(22);
        assert!(tester.value() == 0 || tester.value() == 1);
        let tester = Type::new(33);
        assert!(tester.value() == 0 || tester.value() == 1);
    }

    #[test]
    #[no_coverage]
    fn ctor11() {
        type Type = WrappedU8<1, 1>;
        let tester = Type::new(0);
        assert!(tester.value() == 1);
        let tester = Type::new(10);
        assert!(tester.value() == 1);
        let tester = Type::new(11);
        assert!(tester.value() == 1);
        let tester = Type::new(22);
        assert!(tester.value() == 1);
        let tester = Type::new(33);
        assert!(tester.value() == 1);
    }

    #[test]
    #[no_coverage]
    fn ctor15() {
        type Type = WrappedU8<1, 5>;
        let tester = Type::new(0);
        assert!(tester.value() == 5);
        let tester = Type::new(10);
        assert!(tester.value() == 5);
        let tester = Type::new(11);
        assert!(tester.value() == 1);
        let tester = Type::new(22);
        assert!(tester.value() == 2);
        let tester = Type::new(33);
        assert!(tester.value() == 3);
    }

    #[test]
    #[no_coverage]
    fn from_any() {
        type Type = WrappedU8<1, 5>;
        let tester = Type::from_any_unsigned(0usize);
        assert!(tester.value() == 5);
        let tester = Type::from_any_signed(-1isize);
        assert!(tester.value() == 4);
    }

    #[test]
    #[no_coverage]
    fn add() {
        type Tester = WrappedU8<0, 9>;
        let tester = Tester::new(0);
        assert_eq!(tester.value(), 0);
        let tester = Tester::new(10);
        assert_eq!(tester.value(), 0);
        let tester = Tester::new(11);
        assert_eq!(tester.value(), 1);
        let tester = Tester::new(22);
        assert_eq!(tester.value(), 2);
        let tester = Tester::new(33);
        assert_eq!(tester.value(), 3);

        let result = tester + 10u8;
        assert_eq!(result.value(), 3);
        let result = tester - 9u8;
        assert_eq!(result.value(), 4);
        let result = tester + Tester::new(2);
        assert_eq!(result.value(), 5);
    }
}
