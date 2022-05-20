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

impl Method {
    pub(crate) fn all_methods() -> impl Iterator<Item = Self> {
        [
            Method::Json,
            Method::Cbor,
            Method::MsgPack,
            Method::Protobuf,
            Method::Flatbuffer,
            Method::Flexbuffer,
        ]
        .iter()
        .copied()
    }

    pub(crate) fn working_methods() -> impl Iterator<Item = Self> {
        [Method::Json, Method::Cbor, Method::MsgPack]
            .iter()
            .copied()
    }
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Method::Json => write!(f, "json"),
            Method::Cbor => write!(f, "cbor"),
            Method::MsgPack => write!(f, "msgpack"),
            Method::Protobuf => write!(f, "protobuf"),
            Method::Flatbuffer => write!(f, "flatbuffer"),
            Method::Flexbuffer => write!(f, "flexbuffer"),
        }
    }
}

/// Empty struct holding methods for persisting and retrieving data.
pub struct Persistence;

impl Persistence {
    pub const DEFAULT_METHOD: Method = Method::MsgPack;

    /// Attempts to deserialize the given bytes into the requested type, using the
    /// default serialization method (queried through [`Persistence::DEFAULT_METHOD`]),
    /// returning an error if this process fails.
    ///
    /// ## Errors
    /// - `Error::Io` - If any i/o errors occur
    /// - `Error::Json` or `Error::SerDe` - If the (de)serialization process fails
    /// - `Error::NotImplemented` - If the requested method is not (yet) implemented
    pub fn load_from_bytes_default<T>(bytes: &[u8]) -> crate::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        crate::profile_guard!("load_from_bytes_default", "util::Persistence");
        Self::load_from_bytes(bytes, Self::DEFAULT_METHOD)
    }

    /// Attempts to deserialize the given bytes into the requested type, using the given serialization
    /// `method`, returning an error if this process fails.
    ///
    /// ## Errors
    /// - `Error::Io` - If any i/o errors occur
    /// - `Error::Json` or `Error::SerDe` - If the (de)serialization process fails
    /// - `Error::NotImplemented` - If the requested method is not (yet) implemented
    pub fn load_from_bytes<T>(bytes: &[u8], method: Method) -> crate::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        crate::profile_guard!("load_from_bytes", "util::Persistence");
        match method {
            Method::Json => {
                let output: T = serde_json::from_slice(bytes)?;
                Ok(output)
            }
            Method::Cbor => {
                let output = ciborium::de::from_reader(bytes)?;
                Ok(output)
            }
            Method::MsgPack => {
                let output = rmp_serde::from_read(bytes)?;
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

    /// Attempts to serialize the given `data` into bytes, returning an error
    /// if this process fails.
    ///
    /// ## Errors
    /// - `Error::Io` - If any i/o errors occur
    /// - `Error::Json` or `Error::SerDe` - If the (de)serialization process fails
    /// - `Error::NotImplemented` - If the requested method is not (yet) implemented
    pub fn save_to_bytes<T>(data: &T, method: Method) -> crate::Result<Vec<u8>>
    where
        T: serde::Serialize,
    {
        crate::profile_guard!("save_to_bytes", "util::Persistence");
        let mut bytes = Vec::with_capacity(2048);
        match method {
            Method::Json => {
                serde_json::to_writer(&mut bytes, data)?;
                Ok(bytes)
            }
            Method::Cbor => {
                ciborium::ser::into_writer(data, &mut bytes)?;
                Ok(bytes)
            }
            Method::MsgPack => {
                rmp_serde::encode::write(&mut bytes, data)?;
                Ok(bytes)
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

    /// Attempts to serialize the given `data` into bytes, using the default serialization
    /// method, returning an error if this process fails.
    ///
    /// ## Errors
    /// - `Error::Io` - If any i/o errors occur
    /// - `Error::Json` or `Error::SerDe` - If the (de)serialization process fails
    /// - `Error::NotImplemented` - If the requested method is not (yet) implemented
    pub fn save_to_bytes_default<T>(data: &T) -> crate::Result<Vec<u8>>
    where
        T: serde::Serialize,
    {
        crate::profile_guard!("save_to_bytes_default", "util::Persistence");
        Self::save_to_bytes(data, Self::DEFAULT_METHOD)
    }

    /// Loads data from the specified file, deserializing it using the indicated method.
    ///
    /// ## Errors
    /// - `Error::Io` - If any i/o errors occur
    /// - `Error::Json` or `Error::SerDe` - If the (de)serialization process fails
    /// - `Error::NotImplemented` - If the requested method is not (yet) implemented
    pub fn load_from_file<T>(path: impl AsRef<Path>, method: Method) -> crate::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        use std::fs::File;
        use std::io::Read;
        crate::profile_guard!("load_from_file", "util::Persistence");
        let path = path.as_ref();

        if !path.exists() {
            return crate::Error::Database(crate::DatabaseError::DataFileNotFound(
                path.to_path_buf(),
            ))
            .into();
        }

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
    /// - `Error::NotImplemented` - If the requested method is not (yet) implemented
    pub fn load_from_file_default<T>(path: impl AsRef<Path>) -> crate::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        crate::profile_guard!("load_from_file_default", "util::Persistence");
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
    /// - `Error::NotImplemented` - If the requested method is not (yet) implemented
    pub fn save_to_file<T>(data: &T, path: impl AsRef<Path>, method: Method) -> crate::Result
    where
        T: serde::Serialize,
    {
        use std::fs::File;
        use std::io::Write;

        crate::profile_guard!("save_to_file", "util::Persistence");

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
    /// - `Error::NotImplemented` - If the requested method is not (yet) implemented
    pub fn save_to_new_file<T>(data: &T, path: impl AsRef<Path>, method: Method) -> crate::Result
    where
        T: serde::Serialize,
    {
        crate::profile_guard!("save_to_new_file", "util::Persistence");
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
    /// - `Error::NotImplemented` - If the requested method is not (yet) implemented
    pub fn save_to_file_default<T>(data: &T, path: impl AsRef<Path>) -> crate::Result
    where
        T: serde::Serialize,
    {
        crate::profile_guard!("save_to_file_default", "util::Persistence");
        Self::save_to_file(data, path, Self::DEFAULT_METHOD)
    }

    /// TODO: Checkout [this serde docs page](https://serde.rs/transcode.html) to simplify this.
    ///
    /// Converts a file from one serialization format to another. Unfortunately there is
    /// no way to check whether a file was actually serialized with the given format in
    /// the first place, so a backup of the file is made before the conversion takes place.
    ///
    /// ## Errors
    /// - `Error::Io` - If any i/o errors occur
    /// - `Error::Json` or `Error::SerDe` - If the (de)serialization process fails
    /// - `Error::NotImplemented` - If the requested method is not (yet) implemented
    pub fn convert_file<T>(path: impl AsRef<Path>, from: Method, to: Method) -> crate::Result
    where
        T: serde::Serialize + serde::de::DeserializeOwned,
    {
        crate::profile_guard!("convert_file", "util::Persistence");
        let path = path.as_ref();
        let backup = format!("{}.bak", path.display());
        std::fs::copy(path, backup)?;
        let data: T = Self::load_from_file(path, from)?;
        Self::save_to_file(&data, path, to)?;
        Ok(())
    }

    /// TODO: Checkout [this serde docs page](https://serde.rs/transcode.html) to simplify this.
    ///
    /// Converts a file from one serialization format to another. Unfortunately there is
    /// no way to check whether a file was actually serialized with the given format in
    /// the first place, so a backup of the file is made before the conversion takes place.
    ///
    /// ## Errors
    /// - `Error::Io` - If any i/o errors occur
    /// - `Error::Json` or `Error::SerDe` - If the (de)serialization process fails
    /// - `Error::NotImplemented` - If the requested method is not (yet) implemented
    pub fn convert_bytes<T>(bytes: &[u8], from: Method, to: Method) -> crate::Result<Vec<u8>>
    where
        T: serde::Serialize + serde::de::DeserializeOwned,
    {
        crate::profile_guard!("convert_bytes", "util::Persistence");
        let data: T = Self::load_from_bytes(bytes, from)?;
        Self::save_to_bytes(&data, to)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Default, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
    struct TestStruct {
        length: usize,
        flag: bool,
        decimal: f64,
        number: i64,
        text: String,
    }

    #[test]
    #[no_coverage]
    fn bytes() {
        let data = TestStruct {
            length: 10,
            flag: true,
            decimal: 1.0,
            number: -1,
            text: "hello".to_string(),
        };

        for method in Method::all_methods() {
            if method == Method::Flexbuffer
                || method == Method::Flatbuffer
                || method == Method::Protobuf
            {
                let result = Persistence::save_to_bytes(&data, method);
                assert!(result.is_err());
                let back: Result<TestStruct, _> = Persistence::load_from_bytes(&Vec::new(), method);
                assert!(back.is_err());
            } else {
                let result = Persistence::save_to_bytes(&data, method);
                assert!(result.is_ok());
                let bytes = result.unwrap();
                let back: Result<TestStruct, _> = Persistence::load_from_bytes(&bytes, method);
                assert!(back.is_ok());
                let cereal = back.unwrap();
                assert_eq!(cereal, data);
                assert_ne!(cereal, TestStruct::default());
            }
        }

        let result = Persistence::save_to_bytes_default(&data);
        assert!(result.is_ok());
        let bytes = result.unwrap();
        let back: Result<TestStruct, _> = Persistence::load_from_bytes_default(&bytes);
        assert!(back.is_ok());
        let cereal = back.unwrap();
        assert_eq!(cereal, data);
        assert_ne!(cereal, TestStruct::default());
    }

    #[test]
    #[no_coverage]
    fn save_and_load_file() {
        let data = TestStruct {
            length: 10,
            flag: true,
            decimal: 1.0,
            number: -1,
            text: "hello".to_string(),
        };
        let tempfile = std::env::temp_dir().join(format!(
            "persist-tests-save_and_load_file-{:010}.tmp",
            fastrand::u32(..)
        ));
        assert!(!tempfile.exists(), "tempfile should not already exist!");
        println!("Tempfile Path: {}", tempfile.display());

        for method in Method::all_methods() {
            if method == Method::Flexbuffer
                || method == Method::Flatbuffer
                || method == Method::Protobuf
            {
                let result = Persistence::save_to_file(&data, &tempfile, method);
                assert!(result.is_err());
                let back: Result<TestStruct, _> = Persistence::load_from_file(&tempfile, method);
                assert!(back.is_err());
            } else {
                let result = Persistence::save_to_file(&data, &tempfile, method);
                assert!(result.is_ok());
                assert!(Persistence::save_to_new_file(&data, &tempfile, method).is_err());
                let back: Result<TestStruct, _> = Persistence::load_from_file(&tempfile, method);
                assert!(back.is_ok());
                let cereal = back.unwrap();
                assert_eq!(cereal, data);
            }
        }

        let result = Persistence::save_to_file_default(&data, &tempfile);
        assert!(result.is_ok());
        let back: Result<TestStruct, _> = Persistence::load_from_file_default(&tempfile);
        assert!(back.is_ok());
        let cereal = back.unwrap();
        assert_eq!(cereal, data);
        std::fs::remove_file(tempfile).expect("Unable to delete tempfile");

        let bad_file = std::env::temp_dir().join(format!(
            "persist-tests-save_and_load_file-bad_file-{:010}.tmp",
            fastrand::u32(..)
        ));
        assert!(
            !bad_file.exists(),
            "bad_file ({}) should not already exist!",
            bad_file.display()
        );
        assert!(Persistence::load_from_file::<TestStruct>(&bad_file, Method::Json).is_err());
        assert!(Persistence::load_from_file_default::<TestStruct>(&bad_file).is_err());
    }

    #[test]
    #[no_coverage]
    fn convert_bytes() {
        let data = TestStruct {
            length: 10,
            flag: true,
            decimal: 1.0,
            number: -1,
            text: "hello".to_string(),
        };

        let mut json_bytes = Persistence::save_to_bytes(&data, Method::Json).unwrap();

        let mut cbor_bytes =
            Persistence::convert_bytes::<TestStruct>(&json_bytes, Method::Json, Method::Cbor)
                .unwrap();
        let cbor_data: TestStruct =
            Persistence::load_from_bytes(&cbor_bytes, Method::Cbor).unwrap();
        assert_eq!(cbor_data, data);

        let mut msgpack_bytes =
            Persistence::convert_bytes::<TestStruct>(&cbor_bytes, Method::Cbor, Method::MsgPack)
                .unwrap();
        let mp_data: TestStruct =
            Persistence::load_from_bytes(&msgpack_bytes, Method::MsgPack).unwrap();
        assert_eq!(mp_data, data);

        json_bytes =
            Persistence::convert_bytes::<TestStruct>(&msgpack_bytes, Method::MsgPack, Method::Json)
                .unwrap();
        let json_data: TestStruct =
            Persistence::load_from_bytes(&json_bytes, Method::Json).unwrap();
        assert_eq!(json_data, data);
    }

    #[test]
    #[no_coverage]
    fn convert_file() {
        let data = TestStruct {
            length: 10,
            flag: true,
            decimal: 1.0,
            number: -1,
            text: "hello".to_string(),
        };

        let tempfile = std::env::temp_dir().join(format!(
            "persist-tests-convert_file-{:010}.tmp",
            fastrand::u32(..)
        ));
        assert!(
            !tempfile.exists(),
            "tempfile ({}) should not already exist!",
            tempfile.display()
        );

        println!("Tempfile Path: {}", tempfile.display());

        let result = Persistence::save_to_new_file(&data, &tempfile, Method::Json);
        assert!(result.is_ok());

        for &(from, to) in &[
            (Method::Json, Method::Cbor),
            (Method::Cbor, Method::MsgPack),
            (Method::MsgPack, Method::Json),
        ] {
            assert!(
                Persistence::convert_file::<TestStruct>(&tempfile, from, to).is_ok(),
                "Converting tempfile from {} to {} failed",
                from,
                to
            );
            let back: Result<TestStruct, _> = Persistence::load_from_file(&tempfile, to);
            assert!(back.is_ok());
            let cereal = back.unwrap();
            assert_eq!(cereal, data);
        }

        std::fs::remove_file(tempfile).expect("Unable to delete tempfile");

        let bad_file = std::env::temp_dir().join(format!(
            "persist-tests-convert_file-bad_file-{:010}.tmp",
            fastrand::u32(..)
        ));
        assert!(
            !bad_file.exists(),
            "bad_file ({}) should not already exist!",
            bad_file.display()
        );
        assert!(
            Persistence::convert_file::<TestStruct>(&bad_file, Method::Json, Method::Cbor).is_err()
        );
    }

    #[test]
    #[no_coverage]
    fn save_and_load_file_default() {
        let data = TestStruct {
            length: 10,
            flag: true,
            decimal: 1.0,
            number: -1,
            text: "hello".to_string(),
        };
        let tempfile = std::env::temp_dir().join(format!(
            "persist-tests-save_and_load_file_default-{:010}.tmp",
            fastrand::u32(..)
        ));
        assert!(!tempfile.exists(), "tempfile should not already exist!");
        println!("Tempfile Path: {}", tempfile.display());

        let result = Persistence::save_to_file_default(&data, &tempfile);
        assert!(result.is_ok());
        let back: Result<TestStruct, _> = Persistence::load_from_file_default(&tempfile);
        assert!(back.is_ok());
        let cereal = back.unwrap();
        assert_eq!(cereal, data);
    }

    #[test]
    #[no_coverage]
    fn method() {
        assert_eq!(Method::Json.to_string(), "json");
        assert_eq!(Method::Cbor.to_string(), "cbor");
        assert_eq!(Method::MsgPack.to_string(), "msgpack");
        assert_eq!(Method::Protobuf.to_string(), "protobuf");
        assert_eq!(Method::Flatbuffer.to_string(), "flatbuffer");
        assert_eq!(Method::Flexbuffer.to_string(), "flexbuffer");

        assert_eq!(Method::all_methods().count(), 6);
        assert_eq!(Method::working_methods().count(), 3);
    }

    crate::flame_all_tests!(
        ["persist", "Persistence", "tests"],
        bytes,
        save_and_load_file,
        convert_bytes,
        convert_file,
        save_and_load_file_default
    );
}
