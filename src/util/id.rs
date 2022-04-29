// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

const LETTER_COUNT: usize = 64;
const LETTERS: [u8; LETTER_COUNT] = [
    b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p',
    b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'A', b'B', b'C', b'D', b'E', b'F',
    b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V',
    b'W', b'X', b'Y', b'Z', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0', b'_', b'-',
];
const NULL_CHAR: u8 = b'\0';
const NULL_INSTANCE: [u8; 8] = [NULL_CHAR; 8];

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShortIdError {
    InvalidLength,
    InvalidCharacters,
    Conversion(String),
}

impl std::fmt::Display for ShortIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShortIdError::InvalidLength => write!(f, "Invalid length"),
            ShortIdError::InvalidCharacters => write!(f, "Invalid characters"),
            ShortIdError::Conversion(s) => write!(f, "Conversion error: {}", s),
        }
    }
}

impl From<std::array::TryFromSliceError> for ShortIdError {
    fn from(err: std::array::TryFromSliceError) -> Self {
        Self::Conversion(err.to_string())
    }
}

impl std::error::Error for ShortIdError {}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Deserialize, serde::Serialize,
)]
pub struct ShortId {
    data: [u8; 8],
}

impl ShortId {
    /// Create an instance of the `null` [`ShortId`].
    #[must_use]
    pub fn null() -> Self {
        Self {
            data: [NULL_CHAR; 8],
        }
    }

    /// Create a new random [`ShortId`].
    #[must_use]
    pub fn random() -> Self {
        Self::random_oor()
    }

    /// Attempts to create a new [`ShortId`] for use in the given [`Database`](crate::db::Database).
    /// It will attempt to create an ID that is not in use a certain number of times
    /// (currently arbitrarily set to 100), and return [`Option::None`] if this fails.
    #[must_use]
    pub fn random_against_db(db: &crate::db::Database) -> Option<Self> {
        const MAX_ATTEMPTS: usize = 100;
        for _ in 0..MAX_ATTEMPTS {
            let id = Self::random();
            if !db.id_in_use(id) {
                return Some(id);
            }
        }

        None
    }

    /// Attempts to create a new [`ShortId`] that is not contained within the given [`HashSet`](std::collections::HashSet).
    /// It will attempt to create an ID that is not in use a certain number of times
    /// (currently arbitrarily set to 100), and return [`Option::None`] if this fails.
    #[must_use]
    pub fn random_against_list(list: &std::collections::HashSet<Self>) -> Option<Self> {
        const MAX_ATTEMPTS: usize = 100;
        for _ in 0..MAX_ATTEMPTS {
            let id = Self::random();
            if !list.contains(&id) {
                return Some(id);
            }
        }

        None
    }

    /// Checks whether this [`ShortId`] is null or has any invalid bytes.
    #[must_use]
    pub fn is_valid(self) -> bool {
        !self.is_null() && self.data.iter().all(|&ch| LETTERS.contains(&ch))
    }

    #[must_use]
    pub fn is_null(self) -> bool {
        self.data == NULL_INSTANCE
    }

    fn from_str(s: &str) -> Result<Self, ShortIdError> {
        if s.len() != 8 {
            return Err(ShortIdError::InvalidLength);
        }

        let mut data = [NULL_CHAR; 8];
        for (i, ch) in s.bytes().enumerate() {
            if !LETTERS.contains(&ch) {
                return Err(ShortIdError::InvalidCharacters);
            }
            data[i] = ch;
        }
        Ok(Self { data })
    }

    /// Convert from [`&str`] to [`ShortId`], without checking the length or
    /// individual characters of the input.
    #[must_use]
    pub fn from_str_unchecked(s: &str) -> Self {
        let mut data = [NULL_CHAR; 8];
        for (i, ch) in s.bytes().enumerate() {
            data[i] = ch;
        }
        Self { data }
    }

    /// Convert this [`ShortId`] to an array of 8 bytes.
    #[must_use]
    pub fn to_bytes(self) -> [u8; 8] {
        self.data
    }

    /// Attempt to create a new [`ShortId`] from a u64.
    ///
    /// ## Errors
    /// - [`ShortIdError::InvalidLength`] if the input is not 8 bytes long.
    /// - [`ShortIdError::InvalidCharacters`] if the input contains invalid chars/bytes.
    pub fn from_u64(n: u64) -> Result<Self, ShortIdError> {
        let bytes: [u8; 8] = n.to_be_bytes();
        Self::from_bytes(bytes)
    }

    /// Creates a new [`ShortId`] from the given `u64`, without validating
    /// that the bytes are valid.
    #[must_use]
    pub fn from_u64_unchecked(n: u64) -> Self {
        let mut data: [u8; 8] = n.to_be_bytes();
        Self { data }
    }

    /// Convert this [`ShortId`] to a u64 representation.
    #[must_use]
    pub fn to_u64(self) -> u64 {
        u64::from_be_bytes(self.data)
    }

    /// Attempt to create a new [`ShortId`] from the given byte array.
    ///
    /// ## Errors
    /// - [`ShortIdError::InvalidCharacters`] if the input contains invalid chars/bytes.
    pub fn from_bytes(bytes: [u8; 8]) -> Result<Self, ShortIdError> {
        let id = Self { data: bytes };
        if id.is_valid() {
            Ok(id)
        } else {
            Err(ShortIdError::InvalidCharacters)
        }
    }

    /// Creates a new [`ShortId`] from the given `[u8; 8]`, without validating
    /// that the bytes are valid.
    #[must_use]
    pub fn from_bytes_unchecked(bytes: [u8; 8]) -> Self {
        Self { data: bytes }
    }
}

/// RNG Type Comparison
impl ShortId {
    /// Create a new random [`ShortId`].
    #[must_use]
    pub(crate) fn random_fastrand() -> Self {
        let mut data = [NULL_CHAR; 8];
        for ch in &mut data {
            *ch = LETTERS[fastrand::usize(0..LETTER_COUNT)];
        }
        Self { data }
    }

    #[must_use]
    pub(crate) fn random_nanorand1() -> Self {
        use nanorand::Rng;
        let mut rng = nanorand::tls_rng();
        let mut data = [NULL_CHAR; 8];
        for ch in &mut data {
            *ch = LETTERS[rng.generate_range(0..LETTER_COUNT)];
        }

        Self { data }
    }

    #[must_use]
    pub(crate) fn random_nanorand2() -> Self {
        use nanorand::{BufferedRng, Rng, WyRand};
        let mut rng = BufferedRng::new(nanorand::tls_rng());

        let mut data = [NULL_CHAR; 8];
        rng.fill_bytes(&mut data);
        for ch in &mut data {
            *ch /= 4;
        }
        Self { data }
    }

    #[must_use]
    pub(crate) fn random_rand1() -> Self {
        use rand::distributions::{Alphanumeric, Distribution, Uniform};
        let range = Uniform::new(0, LETTER_COUNT);
        let mut rng = rand::thread_rng();
        let mut data = [NULL_CHAR; 8];
        for b in range
            .sample_iter(&mut rng)
            .take(8)
            .enumerate()
            .map(|(i, l)| (i, LETTERS[l]))
        {
            data[b.0] = b.1;
        }

        Self { data }
    }

    #[allow(clippy::cast_possible_truncation)]
    #[must_use]
    pub(crate) fn random_oor() -> Self {
        const SEED: u64 = 1_000_000;
        const LETTER_COUNT_U32: u32 = LETTER_COUNT as u32;
        let mut rng: oorandom::Rand32 = oorandom::Rand32::new(SEED);
        let mut data = [NULL_CHAR; 8];
        for ch in &mut data {
            *ch = LETTERS[rng.rand_range(0..LETTER_COUNT_U32) as usize];
        }

        Self { data }
    }
}

impl TryFrom<[u8; 8]> for ShortId {
    type Error = ShortIdError;

    fn try_from(value: [u8; 8]) -> std::result::Result<Self, Self::Error> {
        if value.iter().any(|ch| !LETTERS.contains(ch)) {
            Err(ShortIdError::InvalidCharacters)
        } else {
            Ok(Self { data: value })
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for ShortId {
    type Error = ShortIdError;

    fn try_from(value: &'a [u8]) -> std::result::Result<Self, Self::Error> {
        let data = value.to_vec();
        if data.len() != 8 {
            return Err(ShortIdError::InvalidLength);
        }
        if data.iter().any(|ch| !LETTERS.contains(ch)) {
            return Err(ShortIdError::InvalidCharacters);
        }

        let data = <[u8; 8]>::try_from(data.as_slice())?;

        Ok(Self { data })
    }
}

impl From<u64> for ShortId {
    fn from(n: u64) -> Self {
        let mut data: [u8; 8] = n.to_be_bytes();
        Self { data }
    }
}

impl FromIterator<u8> for ShortId {
    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
        let mut data: [u8; 8] = [NULL_CHAR; 8];
        for (i, ch) in iter.into_iter().enumerate() {
            data[i] = ch;
        }
        let id = Self { data };
        assert!(id.is_valid(), "ShortId::from_iter: invalid id");
        id
    }
}

impl std::str::FromStr for ShortId {
    type Err = ShortIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}

impl std::fmt::Display for ShortId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ch in &self.data {
            write!(f, "{}", *ch as char)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

    #[test]
    fn basic_usage() {
        let id = ShortId::random();
        println!("Created id {} ({:?})", id, id);
        assert!(id.is_valid());
        assert!(!id.is_null());
        let num = id.to_u64();
        let back = ShortId::from_u64(num).expect("Unable to convert back to u64");
        assert_eq!(id, back);
        assert_eq!(num, back.to_u64());
        let bytes = id.to_bytes();
        let back = ShortId::from_bytes(bytes).expect("Unable to convert back to bytes");
        assert_eq!(id, back);
        assert_eq!(bytes, back.to_bytes());
        let bad_id = ShortId::null();
        assert!(!bad_id.is_valid());
        assert!(bad_id.is_null());
    }

    #[test]
    fn collision_test_one_million() {
        use std::collections::HashSet;
        let mut ids = HashSet::new();
        for _ in 0..1_000_000 {
            let id = ShortId::random();
            assert!(ids.insert(id));
        }
    }

    /// To run this test use the following cargo command:
    /// ```shell
    /// # remove `ignored` attribute and use this for simpler test output:
    /// cargo test --package noted --lib -- util::id::tests::ignored_shortid_runs_until_collision --exact --nocapture
    /// # or more simply use this without removing the attribute:
    /// cargo test ignored_shortid_runs_until_collision -- --ignored --nocapture
    /// ```
    ///
    /// Test Runs:
    /// | Time (secs) | Iterations |
    /// | --- | --- |
    /// | 100.05 | 35,224,093 |
    /// | 4.26 | 1,880,476 |
    /// | 70.58 | 29,636,196 |
    /// | 45.56 | 21,053,701 |
    #[test]
    #[ignore]
    fn ignored_shortid_runs_until_collision() {
        use std::collections::HashSet;
        let mut ids = HashSet::new();
        let mut new_id = ShortId::random();
        let mut counter: usize = 0;
        let mut failures: usize = 0;
        while failures < 2 {
            while ids.insert(new_id) {
                new_id = ShortId::random();
                counter += 1;
            }
            failures += 1;
        }
        println!("Collision detected after {} iterations", counter);
    }

    /// Compares generating 1,000,000 instances of:
    /// - [`ShortId::random`]
    /// - [`uuid::Uuid::new_v4`]
    /// - [`fastrand::u64`]
    /// - [`fastrand::u8`] x8
    /// Results are ...not great...
    #[allow(clippy::cast_possible_truncation, clippy::similar_names)]
    #[test]
    #[ignore]
    fn generation_comparison() {
        const ITERS: usize = 1_000_000;
        let now = std::time::Instant::now();
        for _ in 0..ITERS {
            let _id = ShortId::random();
        }
        let sid_elapsed = now.elapsed();
        let sid_average = sid_elapsed / ITERS as u32;

        let now = std::time::Instant::now();
        for _ in 0..ITERS {
            let _num = fastrand::u64(..);
        }
        let num_elapsed = now.elapsed();
        let num_average = num_elapsed / ITERS as u32;

        let now = std::time::Instant::now();
        for _ in 0..ITERS {
            let _n1 = fastrand::u8(..);
            let _n2 = fastrand::u8(..);
            let _n3 = fastrand::u8(..);
            let _n4 = fastrand::u8(..);
            let _n5 = fastrand::u8(..);
            let _n6 = fastrand::u8(..);
            let _n7 = fastrand::u8(..);
            let _n8 = fastrand::u8(..);
        }
        let num8_elapsed = now.elapsed();
        let num8_average = num8_elapsed / ITERS as u32;

        let now = std::time::Instant::now();
        for _ in 0..ITERS {
            let _uuid = uuid::Uuid::new_v4();
        }
        let uuid_elapsed = now.elapsed();
        let uuid_average = uuid_elapsed / ITERS as u32;

        println!("Results after {} iterations:", ITERS);
        println!();
        println!(
            "         ShortId: {:>10?} ({:>10?} ave.)",
            sid_elapsed, sid_average
        );
        println!(
            "   fastrand::u64: {:>10?} ({:>10?} ave.)",
            num_elapsed, num_average
        );
        println!(
            "fastrand::u8(x8): {:>10?} ({:>10?} ave.)",
            num8_elapsed, num8_average
        );
        println!(
            "    Uuid::new_v4: {:>10?} ({:>10?} ave.)",
            uuid_elapsed, uuid_average
        );
    }

    #[allow(clippy::cast_possible_truncation, clippy::similar_names)]
    #[test]
    #[ignore]
    fn rng_compare() {
        const ITERS: usize = 1_000_000;

        let now = std::time::Instant::now();
        for _ in 0..ITERS {
            let _id = ShortId::random();
        }
        let std_elapsed = now.elapsed();
        let std_average = std_elapsed / ITERS as u32;

        let now = std::time::Instant::now();
        for _ in 0..ITERS {
            let _id = ShortId::random_rand1();
        }
        let rand1_elapsed = now.elapsed();
        let rand1_average = rand1_elapsed / ITERS as u32;

        let now = std::time::Instant::now();
        for _ in 0..ITERS {
            let _id = ShortId::random_nanorand1();
        }
        let nano1_elapsed = now.elapsed();
        let nano1_average = nano1_elapsed / ITERS as u32;

        let now = std::time::Instant::now();
        for _ in 0..ITERS {
            let _id = ShortId::random_nanorand2();
        }
        let nano2_elapsed = now.elapsed();
        let nano2_average = nano2_elapsed / ITERS as u32;

        let now = std::time::Instant::now();
        for _ in 0..ITERS {
            let _id = ShortId::random_oor();
        }
        let oor_elapsed = now.elapsed();
        let oor_average = oor_elapsed / ITERS as u32;

        println!("Results after {} iterations:", ITERS);
        println!();
        println!(
            "   ShortId: {:>10?} ({:>10?} ave.)",
            std_elapsed, std_average
        );
        println!(
            "    rand 1: {:>10?} ({:>10?} ave.)",
            rand1_elapsed, rand1_average
        );
        println!(
            "nanorand 1: {:>10?} ({:>10?} ave.)",
            nano1_elapsed, nano1_average
        );
        println!(
            "nanorand 2: {:>10?} ({:>10?} ave.)",
            nano2_elapsed, nano2_average
        );
        println!(
            "  oorandom: {:>10?} ({:>10?} ave.)",
            oor_elapsed, oor_average
        );
    }
}
