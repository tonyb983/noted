// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Method {
    Json,
    Cbor,
    MsgPack,
    Protobuf,
    Flatbuffer,
    Flexbuffer,
}

/// Empty struct holding methods for persisting and retrieving data.
pub struct Persistence;

impl Persistence {
    pub const DEFAULT_METHOD: Method = Method::MsgPack;

    /// Loads data from the specified file, deserializing it using the indicated method.
    ///
    /// ## Errors
    /// - `Error::Io` - If any i/o errors occur
    /// - `Error::Json` or `Error::SerDe` - If the (de)serialization process fails
    pub fn load_from_file<T, P: AsRef<Path>>(path: P, method: Method) -> crate::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        use std::fs::File;
        use std::io::Read;
        let path = path.as_ref();
        let mut file = File::open(path)?;
        let mut buf = std::io::BufReader::new(file);
        match method {
            Method::Json => {
                let output = serde_json::from_reader(buf)?;
                Ok(output)
            }
            Method::Cbor => {
                let output = ciborium::de::from_reader(buf)?;
                Ok(output)
            }
            Method::MsgPack => {
                let output = rmp_serde::from_read(buf)?;
                Ok(output)
            }
            Method::Protobuf => {
                crate::Error::not_implemented("protobuf persistence is not yet implemented.").into()
            }
            Method::Flatbuffer => {
                crate::Error::not_implemented("flatbuffer persistence is not yet implemented.")
                    .into()
            }
            Method::Flexbuffer => {
                crate::Error::not_implemented("flexbuffer persistence is not yet implemented.")
                    .into()
            }
        }
    }

    /// Loads data from the specified file, deserializing it using the default method,
    /// which can be queried using [`Persistence::DEFAULT_METHOD`].
    ///
    /// ## Errors
    /// - `Error::Io` - If any i/o errors occur
    /// - `Error::Json` or `Error::SerDe` - If the (de)serialization process fails
    pub fn load_from_file_default<T, P: AsRef<Path>>(path: P) -> crate::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        Self::load_from_file(path, Self::DEFAULT_METHOD)
    }

    /// Serializes the given data using the indicated method/format and saves it to a file
    /// at the specified path. The file will be created if it does not exist, and overwritten
    /// if it does exist. See [`Persistence::save_to_new_file`] for a method that will not
    /// overwrite.
    ///
    /// ## Errors
    /// - `Error::Io` - If any i/o errors occur
    /// - `Error::Json` or `Error::SerDe` - If the (de)serialization process fails
    pub fn save_to_file<T, P: AsRef<Path>>(data: &T, path: P, method: Method) -> crate::Result<()>
    where
        T: serde::Serialize,
    {
        use std::fs::File;
        use std::io::Write;
        let path = path.as_ref();
        let mut file = File::create(path)?;
        let mut buf = std::io::BufWriter::new(file);
        match method {
            Method::Json => {
                serde_json::to_writer(buf, data)?;
                Ok(())
            }
            Method::Cbor => {
                ciborium::ser::into_writer(data, buf)?;
                Ok(())
            }
            Method::MsgPack => {
                rmp_serde::encode::write(&mut buf, data)?;
                Ok(())
            }

            Method::Protobuf => {
                crate::Error::not_implemented("protobuf persistence is not yet implemented.").into()
            }
            Method::Flatbuffer => {
                crate::Error::not_implemented("flatbuffer persistence is not yet implemented.")
                    .into()
            }
            Method::Flexbuffer => {
                crate::Error::not_implemented("flexbuffer persistence is not yet implemented.")
                    .into()
            }
        }
    }

    /// Saves the given data to a file **only if it does not already exist**. Otherwise it
    /// functions identically to [`Persistence::save_to_file`].
    ///
    /// ## Errors
    /// - `Error::Io` - If any i/o errors occur
    /// - `Error::Json` or `Error::SerDe` - If the (de)serialization process fails
    pub fn save_to_new_file<T, P: AsRef<Path>>(
        data: &T,
        path: P,
        method: Method,
    ) -> crate::Result<()>
    where
        T: serde::Serialize,
    {
        if path.as_ref().exists() {
            return Err(
                std::io::Error::new(std::io::ErrorKind::Other, "file already exists").into(),
            );
        }

        Self::save_to_file(data, path, method)
    }

    /// Serializes the given data into a file at the given path using the default serialization
    /// method/format, which can be queried with [`Persistence::DEFAULT_METHOD`].
    ///
    /// ## Errors
    /// - `Error::Io` - If any i/o errors occur
    /// - `Error::Json` or `Error::SerDe` - If the (de)serialization process fails
    pub fn save_to_file_default<T, P: AsRef<Path>>(data: &T, path: P) -> crate::Result<()>
    where
        T: serde::Serialize,
    {
        Self::save_to_file(data, path, Self::DEFAULT_METHOD)
    }

    /// Converts a file from one serialization format to another. Unfortunately there is
    /// no way to check whether a file was actually serialized with the given format in
    /// the first place, so a backup of the file is made before the conversion takes place.
    ///
    /// ## Errors
    /// - `Error::Io` - If any i/o errors occur
    /// - `Error::Json` or `Error::SerDe` - If the (de)serialization process fails
    pub fn convert_file<T, P: AsRef<Path>>(path: P, from: Method, to: Method) -> crate::Result<()>
    where
        T: serde::Serialize + serde::de::DeserializeOwned,
    {
        let path = path.as_ref();
        let backup = format!("{}.bak", path.display());
        std::fs::copy(path, backup)?;
        let data: T = Self::load_from_file(path, from)?;
        Self::save_to_file(&data, path, to)?;
        Ok(())
    }
}
