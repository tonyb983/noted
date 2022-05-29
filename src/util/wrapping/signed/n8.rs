// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::super::{StandardOps, WrappedNumber};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WrappedI8<const MIN: i8 = { i8::MIN }, const MAX: i8 = { i8::MAX }>(i8);

impl<const MINIMUM: i8, const MAXIMUM: i8> WrappedI8<MINIMUM, MAXIMUM> {
    pub const MAX: i8 = MAXIMUM;
    pub const MIN: i8 = MINIMUM;
    pub const RANGE_SIZE: usize = Self::MIN.abs_diff(Self::MAX) as usize + 1;

    #[must_use]
    pub const fn new(mut value: i8) -> Self {
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
    pub const fn create<const N: isize>() -> Self {
        Self::from_any_signed(N)
    }

    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_possible_wrap,
        reason = "i dont think i8 can ever wrap when converting to isize"
    )]
    #[must_use]
    pub const fn from_any_signed(value: impl Into<isize>) -> Self {
        let mut value = value.into();
        loop {
            if value >= MINIMUM as isize && value <= MAXIMUM as isize {
                break Self(value as i8);
            }

            if value < MINIMUM as isize {
                value += MAXIMUM as isize - MINIMUM as isize + 1;
            } else {
                value -= MAXIMUM as isize - MINIMUM as isize + 1;
            }
        }
    }

    #[must_use]
    pub const fn value(self) -> i8 {
        self.0
    }
}

impl<const MINIMUM: i8, const MAXIMUM: i8> WrappedNumber for WrappedI8<MINIMUM, MAXIMUM> {
    type BaseType = i8;

    const RANGE_SIZE: usize = WrappedI8::<MINIMUM, MAXIMUM>::RANGE_SIZE;

    const MIN: Self::BaseType = WrappedI8::<MINIMUM, MAXIMUM>::MIN;

    const MAX: Self::BaseType = WrappedI8::<MINIMUM, MAXIMUM>::MAX;

    fn value(self) -> Self::BaseType {
        WrappedI8::value(self)
    }

    fn create(n: impl Into<Self::BaseType>) -> Self {
        WrappedI8::<MINIMUM, MAXIMUM>::new(n.into())
    }
}

impl<const MINIMUM: i8, const MAXIMUM: i8> std::ops::Add<Self> for WrappedI8<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_any_signed(self.value() as isize + rhs.value() as isize)
    }
}

impl<const MINIMUM: i8, const MAXIMUM: i8> std::ops::Add<i8> for WrappedI8<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn add(self, rhs: i8) -> Self::Output {
        self + Self::new(rhs)
    }
}

impl<const MINIMUM: i8, const MAXIMUM: i8> std::ops::Sub<Self> for WrappedI8<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        #[allow(clippy::cast_possible_wrap)]
        WrappedI8::from_any_signed((self.value() as isize) - rhs.value() as isize)
    }
}

impl<const MINIMUM: i8, const MAXIMUM: i8> std::ops::Sub<i8> for WrappedI8<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn sub(self, rhs: i8) -> Self::Output {
        self - Self::new(rhs)
    }
}

impl<const MINIMUM: i8, const MAXIMUM: i8> std::ops::Mul<Self> for WrappedI8<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        WrappedI8::from_any_signed(self.value() as isize * rhs.value() as isize)
    }
}

impl<const MINIMUM: i8, const MAXIMUM: i8> std::ops::Mul<i8> for WrappedI8<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn mul(self, rhs: i8) -> Self::Output {
        self * Self::new(rhs)
    }
}

impl<const MINIMUM: i8, const MAXIMUM: i8> std::ops::Div<Self> for WrappedI8<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        WrappedI8::from_any_signed(self.value() as isize / rhs.value() as isize)
    }
}

impl<const MINIMUM: i8, const MAXIMUM: i8> std::ops::Div<i8> for WrappedI8<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn div(self, rhs: i8) -> Self::Output {
        self / Self::new(rhs)
    }
}

impl<const MINIMUM: i8, const MAXIMUM: i8> std::ops::Rem<Self> for WrappedI8<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        WrappedI8::from_any_signed(self.value() as isize % rhs.value() as isize)
    }
}

impl<const MINIMUM: i8, const MAXIMUM: i8> std::ops::Rem<i8> for WrappedI8<MINIMUM, MAXIMUM> {
    type Output = Self;

    fn rem(self, rhs: i8) -> Self::Output {
        self % Self::new(rhs)
    }
}

impl<const MINIMUM: i8, const MAXIMUM: i8> std::ops::Deref for WrappedI8<MINIMUM, MAXIMUM> {
    type Target = i8;

    #[must_use]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const MINIMUM: i8, const MAXIMUM: i8> std::ops::DerefMut for WrappedI8<MINIMUM, MAXIMUM> {
    #[must_use]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const MINIMUM: i8, const MAXIMUM: i8>
    StandardOps<WrappedI8<MINIMUM, MAXIMUM>, WrappedI8<MINIMUM, MAXIMUM>>
    for WrappedI8<MINIMUM, MAXIMUM>
{
}

impl<const MINIMUM: i8, const MAXIMUM: i8> StandardOps<WrappedI8<MINIMUM, MAXIMUM>, i8>
    for WrappedI8<MINIMUM, MAXIMUM>
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne, assert_str_eq};

    #[test]
    #[no_coverage]
    fn ctor01() {
        type Type = WrappedI8<0, 1>;
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
        type Type = WrappedI8<1, 1>;
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
        type Type = WrappedI8<1, 5>;
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
        type Type = WrappedI8<1, 5>;
        let tester = Type::from_any_signed(-100isize);
        assert!(tester.value() == 5);
        let tester = Type::from_any_signed(-1isize);
        assert!(tester.value() == 4);
    }

    #[test]
    #[no_coverage]
    fn add() {
        type Tester = WrappedI8<0, 10>;
        let tester = Tester::new(0);
        assert_eq!(tester.value(), 0);
        let tester = Tester::new(10);
        assert_eq!(tester.value(), 10);
        let tester = Tester::new(11);
        assert_eq!(tester.value(), 0);
        let tester = Tester::new(22);
        assert_eq!(tester.value(), 1);
        let tester = Tester::new(33);
        assert_eq!(tester.value(), 2);
    }
}
