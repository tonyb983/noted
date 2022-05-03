// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub trait ScopeFunctions {
    fn mutate(&mut self, f: impl FnOnce(&mut Self));

    fn run(&self, f: impl FnOnce(&Self));
}

impl<T> ScopeFunctions for T {
    fn mutate(&mut self, f: impl FnOnce(&mut Self)) {
        f(self);
    }

    fn run(&self, f: impl FnOnce(&Self)) {
        f(self);
    }
}
