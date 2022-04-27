// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[macro_export]
macro_rules! value_type {
    ($name:ident { $($fields:ident : $types:ty,)* }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name {
            $($fields: $types,)*
        }

        impl $name {
            pub fn new($($fields: $types,)*) -> Self {
                Self { $($fields,)* }
            }

            $(
                pub fn $fields(&self) -> &$types {
                    &self.$fields
                }
            )*
        }
    };
}

#[macro_export]
macro_rules! value_wrapper {
    ($name:ident, $type:ty) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(pub $type);

        impl $name {
            pub fn new(value: $type) -> Self {
                Self(value)
            }

            pub fn get(&self) -> &$type {
                &self.0
            }

            pub fn get_mut(&mut self) -> &mut $type {
                &mut self.0
            }

            pub fn into_inner(self) -> $type {
                self.0
            }
        }

        impl From<$type> for $name {
            fn from(value: $type) -> Self {
                Self(value)
            }
        }
    };
}

mod tester {
    value_type!(Test {
        a: u32,
        b: u32,
        c: u32,
    });

    value_wrapper!(TestWrapper, Test);
}
