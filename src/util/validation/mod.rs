// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::util::variadic::ZeroOrMore;

pub trait Validator {
    type Input;
    type Err: std::error::Error;

    /// Validates the given input, returning [`Self::Err`] on failure.
    ///
    /// ## Errors
    /// - Any validation errors that occur.
    fn validate(&self, input: &Self::Input) -> Result<(), Self::Err>;
}

type ValidatorFunc<T, E> = Box<dyn Fn(&T) -> Result<(), E>>;

struct GenericValidator<TInput, TErr>
where
    TErr: std::error::Error,
{
    reqs: Vec<ValidatorFunc<TInput, TErr>>,
}

impl<T, E: std::error::Error> GenericValidator<T, E> {
    pub fn new() -> Self {
        Self { reqs: Vec::new() }
    }

    pub fn from_reqs(reqs: impl Into<ZeroOrMore<ValidatorFunc<T, E>>>) -> Self {
        Self {
            reqs: reqs.into().into_values(),
        }
    }

    pub fn len(&self) -> usize {
        self.reqs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.reqs.is_empty()
    }

    pub fn has_reqs(&self) -> bool {
        !self.is_empty()
    }

    pub fn add_req(&mut self, req: ValidatorFunc<T, E>) {
        self.reqs.push(req);
    }

    pub fn clear_reqs(&mut self) {
        self.reqs.clear();
    }

    pub fn validate(&self, input: &T) -> Result<(), E> {
        for req in &self.reqs {
            req(input)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn temp() {
        #[derive(Debug)]
        enum TestError {
            Empty,
            Message(String),
        }
        impl std::fmt::Display for TestError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Self::Empty => write!(f, "TestError::Empty"),
                    Self::Message(s) => write!(f, "TestError::Message({})", s),
                }
            }
        }
        impl std::error::Error for TestError {}
        impl From<String> for TestError {
            fn from(err: String) -> Self {
                Self::Message(err)
            }
        }
        impl<'s> From<&'s str> for TestError {
            fn from(err: &'s str) -> Self {
                Self::Message(err.to_string())
            }
        }

        let vs: Vec<ValidatorFunc<u64, String>> = vec![
            Box::new(|x| {
                if *x > 0 {
                    Ok(())
                } else {
                    Err("Must be greater than 0".to_string())
                }
            }),
            Box::new(|x| {
                if *x < 100 {
                    Ok(())
                } else {
                    Err("Must be less than 100".to_string())
                }
            }),
        ];
    }
}
