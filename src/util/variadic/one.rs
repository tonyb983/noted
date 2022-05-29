// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub enum OneOrMore<T> {
    One(T),
    Many(Vec<T>),
}

impl<T> OneOrMore<T> {
    #[must_use]
    pub fn one(t: T) -> Self {
        Self::One(t)
    }

    #[must_use]
    pub fn many(ts: impl Into<Vec<T>>) -> Self {
        Self::Many(ts.into())
    }

    #[must_use]
    pub fn count(&self) -> usize {
        match self {
            Self::One(_) => 1,
            Self::Many(ts) => ts.len(),
        }
    }

    #[must_use]
    pub fn into_values(self) -> Vec<T> {
        match self {
            Self::One(t) => vec![t],
            Self::Many(ts) => ts,
        }
    }

    #[must_use]
    pub fn into_values_with_count(self) -> (Vec<T>, usize) {
        match self {
            Self::One(t) => (vec![t], 1),
            Self::Many(ts) => {
                let len = ts.len();
                (ts, len)
            }
        }
    }
}

impl<T> From<T> for OneOrMore<T> {
    fn from(t: T) -> Self {
        Self::one(t)
    }
}

impl<T> From<Vec<T>> for OneOrMore<T> {
    fn from(ts: Vec<T>) -> Self {
        Self::many(ts)
    }
}

impl<T: Clone> From<&[T]> for OneOrMore<T> {
    fn from(ts: &[T]) -> Self {
        Self::many(ts.to_vec())
    }
}

impl<T> FromIterator<T> for OneOrMore<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let ts: Vec<T> = iter.into_iter().collect();
        if ts.len() == 1 {
            Self::one(ts.into_iter().next().unwrap())
        } else {
            Self::many(ts)
        }
    }
}

impl<T: Clone> From<&T> for OneOrMore<T> {
    fn from(t: &T) -> Self {
        Self::one(t.clone())
    }
}

impl<T: Clone> From<&Vec<T>> for OneOrMore<T> {
    fn from(ts: &Vec<T>) -> Self {
        let values: Vec<T> = ts.clone();
        Self::many(values)
    }
}

impl<T: Clone> From<&Vec<&T>> for OneOrMore<T> {
    fn from(ts: &Vec<&T>) -> Self {
        let values: Vec<T> = ts.iter().map(|&t| t.clone()).collect();
        Self::many(values)
    }
}

impl<'input, T: 'input + Clone> FromIterator<&'input T> for OneOrMore<T> {
    fn from_iter<I: IntoIterator<Item = &'input T>>(iter: I) -> Self {
        let ts: Vec<T> = iter.into_iter().cloned().collect();
        if ts.len() == 1 {
            Self::one(ts.into_iter().next().unwrap())
        } else {
            Self::many(ts)
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for OneOrMore<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::One(t) => write!(
                f,
                "OneOrMore::<{}>::One({:?})",
                std::any::type_name::<T>(),
                t
            ),
            Self::Many(ts) => write!(
                f,
                "OneOrMore::<{}>::Many({:?})",
                std::any::type_name::<T>(),
                ts
            ),
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for OneOrMore<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::One(t) => write!(f, "{}", t),
            Self::Many(ts) => {
                let mut first = true;
                for t in ts {
                    if first {
                        first = false;
                    } else {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", t)?;
                }
                Ok(())
            }
        }
    }
}

impl<T: Clone> Clone for OneOrMore<T> {
    fn clone(&self) -> Self {
        match self {
            Self::One(t) => Self::one(t.clone()),
            Self::Many(ts) => Self::many(ts.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne, assert_str_eq};

    #[test]
    #[no_coverage]
    fn all_variants() {
        let mut zom = OneOrMore::one(0usize);
        assert_eq!(zom.count(), 1);
        let mut values = zom.clone().into_values();
        assert_eq!(values, vec![0]);
        println!("Zom Debug: {:?}", zom);
        println!("Zom Display: {}", zom);

        zom = OneOrMore::many(vec![0usize, 1, 2, 3, 4, 5]);
        assert_eq!(zom.count(), 6);
        values = zom.clone().into_values();
        assert_eq!(values, vec![0, 1, 2, 3, 4, 5]);
        println!("Zom Debug: {:?}", zom);
        println!("Zom Display: {}", zom);
    }

    #[test]
    #[no_coverage]
    fn conv() {
        let mut zom: OneOrMore<_> = 0usize.into();
        assert_eq!(zom.count(), 1);
        let mut values = zom.clone().into_values();
        assert_eq!(values, vec![0]);
        println!("Zom Debug: {:?}", zom);
        println!("Zom Display: {}", zom);

        zom = vec![0usize, 1, 2, 3, 4, 5].into();
        assert_eq!(zom.count(), 6);
        values = zom.clone().into_values();
        assert_eq!(values, vec![0, 1, 2, 3, 4, 5]);
        println!("Zom Debug: {:?}", zom);
        println!("Zom Display: {}", zom);
    }
}
