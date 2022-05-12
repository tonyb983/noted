// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub enum ZeroOrMore<T> {
    None,
    One(T),
    Many(Vec<T>),
}

impl<T> ZeroOrMore<T> {
    #[must_use]
    pub fn none() -> Self {
        Self::None
    }

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
            Self::None => 0,
            Self::One(_) => 1,
            Self::Many(ts) => ts.len(),
        }
    }

    #[must_use]
    pub fn into_values(self) -> Vec<T> {
        match self {
            Self::None => vec![],
            Self::One(t) => vec![t],
            Self::Many(ts) => ts,
        }
    }

    #[must_use]
    pub fn into_values_with_count(self) -> (Vec<T>, usize) {
        match self {
            Self::None => (vec![], 0),
            Self::One(t) => (vec![t], 1),
            Self::Many(ts) => {
                let len = ts.len();
                (ts, len)
            }
        }
    }
}

impl<T> From<T> for ZeroOrMore<T> {
    fn from(t: T) -> Self {
        // TODO: Find out how (in)efficient this call is and whether it's worth it.
        if std::any::type_name::<T>() == "()" {
            Self::none()
        } else {
            Self::one(t)
        }
    }
}

impl<T> From<Option<T>> for ZeroOrMore<T> {
    fn from(t: Option<T>) -> Self {
        match t {
            None => Self::none(),
            Some(t) => Self::one(t),
        }
    }
}

impl<T> From<Vec<T>> for ZeroOrMore<T> {
    fn from(t: Vec<T>) -> Self {
        match t.len() {
            0 => Self::none(),
            1 => {
                let single = t.into_iter().next().unwrap();
                Self::one(single)
            }
            _ => Self::many(t),
        }
    }
}

impl<T: Clone> From<&T> for ZeroOrMore<T> {
    fn from(t: &T) -> Self {
        Self::one(t.clone())
    }
}

impl<T: Clone> From<&Option<T>> for ZeroOrMore<T> {
    fn from(t: &Option<T>) -> Self {
        match t {
            None => Self::none(),
            Some(t) => Self::one(t.clone()),
        }
    }
}

impl<T: Clone> From<&Vec<T>> for ZeroOrMore<T> {
    fn from(t: &Vec<T>) -> Self {
        Self::Many(t.clone())
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for ZeroOrMore<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => {
                write!(f, "ZeroOrMore::<{}>::None", std::any::type_name::<T>())
            }
            Self::One(t) => write!(
                f,
                "ZeroOrMore::<{}>::One({:?})",
                std::any::type_name::<T>(),
                t
            ),
            Self::Many(ts) => write!(
                f,
                "ZeroOrMore::<{}>::Many({:?})",
                std::any::type_name::<T>(),
                ts
            ),
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for ZeroOrMore<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => {
                write!(f, "None")
            }
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

impl<T: Clone> Clone for ZeroOrMore<T> {
    fn clone(&self) -> Self {
        match self {
            Self::None => Self::none(),
            Self::One(t) => Self::one(t.clone()),
            Self::Many(ts) => Self::many(ts.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne, assert_str_eq};

    fn takes_any_number(ts: impl Into<ZeroOrMore<usize>>) -> usize {
        let ts = ts.into().into_values();
        ts.iter().sum()
    }

    #[test]
    #[no_coverage]
    fn all_variants() {
        let mut zom = ZeroOrMore::none();
        assert_eq!(zom.count(), 0);
        let mut values = zom.clone().into_values();
        assert_eq!(values, Vec::<usize>::new());
        // println!("Zom Debug: {:?}", zom);
        // println!("Zom Display: {}", zom);

        zom = ZeroOrMore::one(0usize);
        assert_eq!(zom.count(), 1);
        values = zom.clone().into_values();
        assert_eq!(values, vec![0]);
        // println!("Zom Debug: {:?}", zom);
        // println!("Zom Display: {}", zom);

        zom = ZeroOrMore::many(vec![0usize, 1, 2, 3, 4, 5]);
        assert_eq!(zom.count(), 6);
        values = zom.clone().into_values();
        assert_eq!(values, vec![0, 1, 2, 3, 4, 5]);
        // println!("Zom Debug: {:?}", zom);
        // println!("Zom Display: {}", zom);

        let silence = zom.into_values();
    }

    #[test]
    #[no_coverage]
    fn conv() {
        let mut zom = ZeroOrMore::none();
        assert_eq!(zom.count(), 0);
        let mut values = zom.clone().into_values();
        assert_eq!(values, Vec::<usize>::new());
        // println!("Zom Debug: {:?}", zom);
        // println!("Zom Display: {}", zom);

        zom = 0usize.into();
        assert_eq!(zom.count(), 1);
        values = zom.clone().into_values();
        assert_eq!(values, vec![0]);
        // println!("Zom Debug: {:?}", zom);
        // println!("Zom Display: {}", zom);

        zom = vec![0usize, 1, 2, 3, 4, 5].into();
        assert_eq!(zom.count(), 6);
        values = zom.clone().into_values();
        assert_eq!(values, vec![0, 1, 2, 3, 4, 5]);
        // println!("Zom Debug: {:?}", zom);
        // println!("Zom Display: {}", zom);

        let silence = zom.into_values();
    }

    #[test]
    #[no_coverage]
    fn as_param() {
        let sum = takes_any_number(vec![0usize, 1, 2, 3, 4]);
        let sum = takes_any_number(10);
    }
}
