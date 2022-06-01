// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Unit;

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "()")
    }
}

impl From<()> for Unit {
    fn from(_: ()) -> Self {
        Self
    }
}

pub trait StringCalculator {
    type Input;

    fn calculate(&self, input: Self::Input) -> String;
}

pub struct StoredString<T: StringCalculator> {
    param: T::Input,
    value: String,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> StoredString<T> {
    pub fn new() -> Self {}
}

impl<T> Memoize<String> for StoredString<T> {
    type Input = T;

    fn get_value(input: Self::Input) -> String {
        input.to_string()
    }
}
