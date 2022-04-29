// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{array::TryFromSliceError, path::PathBuf};

use uuid::Uuid;

use crate::{TinyId, TinyIdError};

#[derive(Debug)]
pub enum Error {
    EnvVar(std::env::VarError),
    Io(std::io::Error),
    Json(serde_json::Error),
    SerDe(String),
    Sqlx(sqlx::Error),
    Rusqlite(rusqlite::Error),
    Database(DatabaseError),
    Unknown(String),
    NotImplemented(String),
    TinyId(TinyIdError),
}

impl Error {
    #[must_use]
    pub fn env_var(err: std::env::VarError) -> Self {
        Error::EnvVar(err)
    }

    #[must_use]
    pub fn io(err: std::io::Error) -> Self {
        Error::Io(err)
    }

    #[must_use]
    pub fn json(err: serde_json::Error) -> Self {
        Error::Json(err)
    }

    #[must_use]
    pub fn serde<S: AsRef<str>>(err: S) -> Self {
        Error::SerDe(err.as_ref().to_string())
    }

    #[must_use]
    pub fn sqlx(err: sqlx::Error) -> Self {
        Error::Sqlx(err)
    }

    #[must_use]
    pub fn rusqlite(err: rusqlite::Error) -> Self {
        Error::Rusqlite(err)
    }

    #[must_use]
    pub fn database(err: DatabaseError) -> Self {
        Error::Database(err)
    }

    #[must_use]
    pub fn unknown<S: AsRef<str>>(err: S) -> Self {
        Error::Unknown(err.as_ref().to_string())
    }

    #[must_use]
    pub fn not_implemented<S: AsRef<str>>(err: S) -> Self {
        Error::NotImplemented(err.as_ref().to_string())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::EnvVar(e) => e.fmt(f),
            Error::Io(e) => e.fmt(f),
            Error::Json(e) => e.fmt(f),
            Error::Sqlx(e) => e.fmt(f),
            Error::Rusqlite(e) => e.fmt(f),
            Error::Database(e) => e.fmt(f),
            Error::Unknown(s) => write!(f, "Unknown error: {}", s),
            Error::SerDe(s) => write!(f, "De/Serialization error occurred: {}", s),
            Error::NotImplemented(s) => write!(f, "Not implemented: {}", s),
            Error::TinyId(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {
    // fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    //     None
    // }

    // fn type_id(&self, _: private::Internal) -> std::any::TypeId
    // where
    //     Self: 'static,
    // {
    //     std::any::TypeId::of::<Self>()
    // }

    // fn backtrace(&self) -> Option<&std::backtrace::Backtrace> {
    //     None
    // }

    // fn description(&self) -> &str {
    //     "description() is deprecated; use Display"
    // }

    // fn cause(&self) -> Option<&dyn std::error::Error> {
    //     self.source()
    // }
}

impl From<std::env::VarError> for Error {
    fn from(e: std::env::VarError) -> Self {
        Error::EnvVar(e)
    }
}
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}
impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err)
    }
}
impl From<ciborium::ser::Error<std::io::Error>> for Error {
    fn from(err: ciborium::ser::Error<std::io::Error>) -> Self {
        Self::SerDe(err.to_string())
    }
}
impl From<ciborium::de::Error<std::io::Error>> for Error {
    fn from(err: ciborium::de::Error<std::io::Error>) -> Self {
        Self::SerDe(err.to_string())
    }
}
impl From<Box<bincode::ErrorKind>> for Error {
    fn from(err: Box<bincode::ErrorKind>) -> Self {
        Self::SerDe(err.to_string())
    }
}
impl From<rmp_serde::encode::Error> for Error {
    fn from(err: rmp_serde::encode::Error) -> Self {
        Self::SerDe(err.to_string())
    }
}
impl From<rmp_serde::decode::Error> for Error {
    fn from(err: rmp_serde::decode::Error) -> Self {
        Self::SerDe(err.to_string())
    }
}
impl From<flexbuffers::SerializationError> for Error {
    fn from(err: flexbuffers::SerializationError) -> Self {
        Self::SerDe(err.to_string())
    }
}
impl From<flexbuffers::DeserializationError> for Error {
    fn from(err: flexbuffers::DeserializationError) -> Self {
        Self::SerDe(err.to_string())
    }
}
impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Self::Sqlx(err)
    }
}
impl From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Self {
        Self::Rusqlite(err)
    }
}
impl From<DatabaseError> for Error {
    fn from(err: DatabaseError) -> Self {
        Self::Database(err)
    }
}
impl From<String> for Error {
    fn from(err: String) -> Self {
        Self::Unknown(err)
    }
}
impl From<&String> for Error {
    fn from(err: &String) -> Self {
        Self::Unknown(err.clone())
    }
}
impl<'s> From<&'s str> for Error {
    fn from(err: &'s str) -> Self {
        Self::Unknown(err.to_string())
    }
}
impl From<TinyIdError> for Error {
    fn from(err: TinyIdError) -> Self {
        Self::TinyId(err)
    }
}

#[derive(Debug)]
pub enum DatabaseError {
    DataFileNotFound(PathBuf),
    IdNotFound(TinyId),
    PolicyFailure(String),
    DuplicateId(TinyId),
    InvalidState(String),
    InvalidId,
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseError::IdNotFound(id) => write!(f, "Note with id {} not found", id),
            DatabaseError::PolicyFailure(s) => write!(f, "Policy failure: {}", s),
            DatabaseError::DuplicateId(id) => write!(f, "Duplicate ID for Database: {}", id),
            DatabaseError::InvalidId => write!(f, "Invalid ID found in Database"),
            DatabaseError::DataFileNotFound(pb) => {
                write!(f, "Data file not found at path '{}'", pb.display())
            }
            DatabaseError::InvalidState(s) => write!(f, "Invalid Database state: {}", s),
        }
    }
}

impl std::error::Error for DatabaseError {}

pub type Result<T> = std::result::Result<T, Error>;

impl<T> From<Error> for Result<T> {
    fn from(err: Error) -> Self {
        Err(err)
    }
}
