// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use num_traits::Bounded;

mod signed;
mod unsigned;

pub use signed::*;
pub use unsigned::*;

pub trait StandardOps<This, Other = This>:
    std::ops::Add<Other, Output = This>
    + std::ops::Sub<Other, Output = This>
    + std::ops::Mul<Other, Output = This>
    + std::ops::Div<Other, Output = This>
    + std::ops::Rem<Other, Output = This>
{
}

pub trait WrappedNumber:
    StandardOps<Self, Self::BaseType> + StandardOps<Self, Self> + std::fmt::Debug + Copy + Sized
{
    type BaseType: Copy + std::fmt::Debug;

    const RANGE_SIZE: usize;
    const MIN: Self::BaseType;
    const MAX: Self::BaseType;

    fn create(n: impl Into<Self::BaseType>) -> Self;
    fn value(self) -> Self::BaseType;
}
