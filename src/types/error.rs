// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{array::TryFromSliceError, path::PathBuf};

use uuid::Uuid;

use tinyid::{TinyId, TinyIdError};

#[derive(Debug)]
pub enum Error {
    EnvVar(std::env::VarError),
    Io(std::io::Error),
    Json(serde_json::Error),
    SerDe(String),
    // Sqlx(sqlx::Error),
    // Rusqlite(rusqlite::Error),
    Database(DatabaseError),
    Unknown(String),
    NotImplemented(String),
    TinyId(TinyIdError),
    Interface(String),
    Time(time::Error),
}

impl Error {
    #[must_use]
    pub fn env_var(err: std::env::VarError) -> Self {
        Self::EnvVar(err)
    }

    #[must_use]
    pub fn io(err: std::io::Error) -> Self {
        Self::Io(err)
    }

    #[must_use]
    pub fn json(err: serde_json::Error) -> Self {
        Self::Json(err)
    }

    #[must_use]
    pub fn serde<S: AsRef<str>>(err: S) -> Self {
        Self::SerDe(err.as_ref().to_string())
    }

    // #[must_use]
    // pub fn sqlx(err: sqlx::Error) -> Self {
    //     Self::Sqlx(err)
    // }

    // #[must_use]
    // pub fn rusqlite(err: rusqlite::Error) -> Self {
    //     Self::Rusqlite(err)
    // }

    #[must_use]
    pub fn database(err: DatabaseError) -> Self {
        Self::Database(err)
    }

    #[must_use]
    pub fn unknown<S: AsRef<str>>(err: S) -> Self {
        Self::Unknown(err.as_ref().to_string())
    }

    #[must_use]
    pub fn not_implemented<S: AsRef<str>>(err: S) -> Self {
        Self::NotImplemented(err.as_ref().to_string())
    }

    pub fn ui<S: std::fmt::Display>(err: S) -> Self {
        Self::Interface(err.to_string())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EnvVar(e) => e.fmt(f),
            Self::Io(e) => e.fmt(f),
            Self::Json(e) => e.fmt(f),
            // Self::Sqlx(e) => e.fmt(f),
            // Self::Rusqlite(e) => e.fmt(f),
            Self::Database(e) => e.fmt(f),
            Self::Unknown(s) => write!(f, "Unknown error: {}", s),
            Self::SerDe(s) => write!(f, "De/Serialization error occurred: {}", s),
            Self::NotImplemented(s) => write!(f, "Not implemented: {}", s),
            Self::TinyId(e) => e.fmt(f),
            Self::Interface(s) => write!(f, "User interface error: {}", s),
            Self::Time(e) => e.fmt(f),
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

impl From<time::Error> for Error {
    fn from(err: time::Error) -> Self {
        Self::Time(err)
    }
}

impl From<std::env::VarError> for Error {
    fn from(e: std::env::VarError) -> Self {
        Self::EnvVar(e)
    }
}
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}
impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err)
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
impl From<inquire::error::InquireError> for Error {
    fn from(err: inquire::error::InquireError) -> Self {
        Self::ui(err)
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

#[must_use]
pub type Result<T = ()> = std::result::Result<T, Error>;

impl<T> From<Error> for Result<T> {
    fn from(err: Error) -> Self {
        Err(err)
    }
}
