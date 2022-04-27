// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::path::Path;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    types::{CreateNote, DeleteNote, Note, NoteDto, UpdateNote},
    util::persist::Persistence,
    DatabaseError, Error, Result, ShortId,
};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Database {
    notes: Vec<Note>,
    #[serde(skip)]
    ids: Vec<ShortId>,
}

impl Database {
    pub const DEFAULT_UPDATE_POLICY: UpdateFailurePolicy = UpdateFailurePolicy::AllOrNothing;

    fn init(&mut self) -> Result<()> {
        self.validate()?;
        Ok(())
    }

    pub fn empty() -> Self {
        Database {
            notes: Vec::new(),
            ids: Vec::new(),
        }
    }

    pub fn from_notes(notes: &[Note]) -> Result<Self> {
        let mut db = Database {
            notes: notes.to_vec(),
            ids: notes.iter().map(Note::id).collect(),
        };
        db.init()?;
        Ok(db)
    }

    pub fn from_notes_vec(notes: Vec<Note>) -> Result<Self> {
        let ids = notes.iter().map(Note::id).collect();
        let mut db = Database { notes, ids };
        db.init()?;
        Ok(db)
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut db: Self = Persistence::load_from_file_default(path)?;
        db.init()?;
        Ok(db)
    }

    pub(crate) fn load_dev() -> Result<Self> {
        let project_dir = std::env::var("CARGO_MANIFEST_DIR")?;
        let path = Path::new(&project_dir).join("data").join("dev.fdb");
        Self::load(path)
    }

    pub(crate) fn load_dev_with(
        filename: &str,
        f: impl FnOnce(std::io::BufReader<std::fs::File>) -> Result<Self>,
    ) -> Result<Self> {
        use std::fs::File;
        use std::io::Read;
        let project_dir = std::env::var("CARGO_MANIFEST_DIR")?;
        let path = Path::new(&project_dir).join("data").join(filename);
        let mut file = File::open(path)?;
        let mut buf_reader = std::io::BufReader::new(file);
        let db: Self = f(buf_reader)?;
        Ok(db)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        Persistence::save_to_file_default(self, path)
    }

    pub(crate) fn save_dev(&self) -> Result<()> {
        let project_dir = std::env::var("CARGO_MANIFEST_DIR")?;
        let path = Path::new(&project_dir).join("data").join("dev.fdb");
        self.save(path)
    }

    pub(crate) fn save_dev_with(
        &self,
        filename: &str,
        writer: impl FnOnce(std::io::BufWriter<std::fs::File>, &Self) -> Result<()>,
    ) -> Result<()> {
        use std::fs::File;
        use std::io::Write;
        let project_dir = std::env::var("CARGO_MANIFEST_DIR")?;
        let path = Path::new(&project_dir).join("data").join(filename);
        let mut file = File::create(path)?;
        let mut buf_writer = std::io::BufWriter::new(file);
        writer(buf_writer, self)?;
        Ok(())
    }

    pub fn apply_dto(&mut self, dto: impl Into<NoteDto>) -> Result<DtoResponse> {
        match dto.into() {
            NoteDto::Create(create_note) => {
                self.apply_create(create_note).map(DtoResponse::Created)
            }
            NoteDto::Update(update_note) => {
                self.apply_update(update_note).map(DtoResponse::Updated)
            }
            NoteDto::Delete(delete_note) => {
                self.apply_delete(delete_note).map(DtoResponse::Deleted)
            }
        }
    }

    pub fn apply_create(&mut self, create: impl Into<CreateNote>) -> Result<Note> {
        let create = create.into();
        let note = Note::create_for(self, create);
        self.ids.push(note.id());
        self.notes.push(note.clone());
        Ok(note)
    }

    pub fn apply_update(&mut self, update: impl Into<UpdateNote>) -> Result<bool> {
        let update = update.into();
        let mut note = self
            .notes
            .iter_mut()
            .find(|n| n.id() == *update.id())
            .ok_or_else(|| DatabaseError::IdNotFound(*update.id()))?;
        Ok(note.update(update))
    }

    pub fn apply_delete(&mut self, delete: impl Into<DeleteNote>) -> Result<bool> {
        let id = *delete.into().id();
        let start = self.notes.len();
        self.notes.retain(|n| n.id() != id);
        Ok(start != self.notes.len())
    }

    pub fn ensure_sync(&mut self, notes: &mut [Note]) {
        for note in notes.iter_mut() {
            if note.pending_delete() {
                let _result = self.apply_delete(note.id());
                note.clear_flags();
                note.make_invalid();
                continue;
            }

            if !note.dirty() {
                continue;
            }

            match self.notes.iter_mut().find(|n| n.id() == note.id()) {
                Some(existing) => {
                    existing.update_from(note);
                }
                None => self.notes.push(note.clone()),
            }
            note.clear_flags();
        }
    }

    pub fn get(&self, id: ShortId) -> Result<&Note> {
        self.notes
            .iter()
            .find(|n| n.id() == id)
            .ok_or_else(|| DatabaseError::IdNotFound(id).into())
    }

    pub fn get_clone(&self, id: ShortId) -> Result<Note> {
        self.notes
            .iter()
            .find(|n| n.id() == id)
            .cloned()
            .ok_or_else(|| DatabaseError::IdNotFound(id).into())
    }

    pub fn get_and_modify(&mut self, id: ShortId, f: impl FnMut(&mut Note)) -> Result<()> {
        self.notes
            .iter_mut()
            .find(|n| n.id() == id)
            .map(f)
            .ok_or_else(|| DatabaseError::IdNotFound(id).into())
    }

    pub fn get_all(&self) -> &[Note] {
        &self.notes
    }

    pub fn find(&self, pred: impl Fn(&&Note) -> bool) -> Vec<&Note> {
        self.notes.iter().filter(pred).collect::<Vec<_>>()
    }

    pub fn text_search(&self, query: &str) -> Vec<&Note> {
        self.notes
            .iter()
            .filter(|n| n.full_text_search(query))
            .collect::<Vec<_>>()
    }

    pub fn len(&self) -> usize {
        self.notes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.notes.is_empty()
    }

    /// Checks whether the given `id` is currently being used in this [`Database`].
    pub fn id_in_use(&self, id: ShortId) -> bool {
        self.notes.iter().any(|n| n.id() == id)
    }

    /// Attempts to create a new [`ShortId`] using [`ShortId::random_against`].
    pub fn create_id(&self) -> Option<ShortId> {
        ShortId::random_against(self)
    }

    /// Attempts to create a new [`ShortId`] for use in this [`Database`]
    /// until it is successful. This could hypothetically lead to an
    /// infinite loop but it seems unlikely.
    pub fn create_id_force(&self) -> ShortId {
        loop {
            if let Some(id) = ShortId::random_against(self) {
                return id;
            }
        }
    }

    fn validate(&mut self) -> Result<()> {
        if self.ids.len() != self.notes.len() {
            self.register_ids();
        }

        if self.notes.iter().any(|n| n.id().is_null()) {
            return Err(DatabaseError::InvalidId.into());
        }
        let start = self.ids.len();
        self.ids.sort_unstable();
        self.ids.dedup();
        if start != self.ids.len() {
            return Err(DatabaseError::DuplicateId.into());
        }
        Ok(())
    }

    fn register_ids(&mut self) {
        self.ids.clear();
        self.ids = Vec::with_capacity(self.notes.len());
        for note in &self.notes {
            self.ids.push(note.id());
        }
    }
}

impl TryFrom<Vec<Note>> for Database {
    type Error = Error;

    fn try_from(notes: Vec<Note>) -> Result<Self> {
        Self::from_notes_vec(notes)
    }
}

impl<'input> TryFrom<&'input [Note]> for Database {
    type Error = Error;

    fn try_from(value: &'input [Note]) -> Result<Self> {
        Self::from_notes(value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum DtoResponse {
    Created(Note),
    Updated(bool),
    Deleted(bool),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UpdateFailurePolicy {
    // If any of the attempted updates would fail, the entire operation is aborted.
    AllOrNothing,
    // As many updates as possible are applied.
    Partial,
    // Updates are applied until an error is encountered.
    AbortOnError,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Creates a new database with the given number of entries
    fn create_dev_db(entries: usize) -> Database {
        let mut notes = Vec::new();
        for i in 0..entries {
            let mut note = Note::create((
                format!("Title {}", i),
                format!("Here is the content for note number {}.", i),
            ));
            for i in 0..fastrand::usize(0..20) {
                let tag = format!("tag{}", fastrand::usize(1..=15));
                if !note.tag_matches(&tag) {
                    note.add_tag(tag);
                }
            }
            notes.push(note);
        }

        Database::from_notes_vec(notes).expect("Failed to create database!")
    }

    /// Saves the given database to the standard dev location
    fn save_dev_db(db: &Database) -> Result<()> {
        println!("Created database with {} notes.", db.len());
        println!("Saving database...");
        let now = std::time::Instant::now();
        let result = db.save_dev();
        let elapsed = now.elapsed();
        if let Err(err) = &result {
            println!("Error occurred while saving database: {}", err);
        } else {
            println!("Saved database with {} entries in {:?}", db.len(), elapsed);
        }

        result
    }

    // #[test]
    fn create_dev_db_1000() {
        save_dev_db(&create_dev_db(1000));
    }

    // #[test]
    fn load_dev_db_time() {
        let now = std::time::Instant::now();
        let db = Database::load_dev().expect("Unable to load database!");
        let elapsed = now.elapsed();
        println!(
            "Loaded database containing {} entries in {:?}.",
            db.len(),
            elapsed
        );
    }

    #[allow(clippy::too_many_lines)]
    // #[test]
    fn serde_compare() {
        use std::io::{Read, Write};
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        enum Op {
            Ser,
            De,
        }
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        enum Method {
            Json,
            Cbor,
            Bincode,
            Msgpack,
            Flexbuffer,
        }
        struct Timing {
            op: Op,
            method: Method,
            entries: usize,
            elapsed: std::time::Duration,
        }
        impl Timing {
            pub fn ser(method: Method, entries: usize, elapsed: std::time::Duration) -> Self {
                Self {
                    op: Op::Ser,
                    method,
                    entries,
                    elapsed,
                }
            }

            pub fn de(method: Method, entries: usize, elapsed: std::time::Duration) -> Self {
                Self {
                    op: Op::De,
                    method,
                    entries,
                    elapsed,
                }
            }
        }
        struct Size {
            method: Method,
            entries: usize,
            size: usize,
        }
        impl std::fmt::Display for Op {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Op::Ser => write!(f, "serialize"),
                    Op::De => write!(f, "deserialize"),
                }
            }
        }
        impl std::fmt::Display for Method {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Method::Json => write!(f, "JSON"),
                    Method::Cbor => write!(f, "CBOR"),
                    Method::Bincode => write!(f, "Bincode"),
                    Method::Msgpack => write!(f, "Msgpack"),
                    Method::Flexbuffer => write!(f, "Flexbuffer"),
                }
            }
        }
        impl std::fmt::Display for Timing {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{:>8} - {}d {} entries in {:>15?}",
                    self.method, self.op, self.entries, self.elapsed
                )
            }
        }
        impl std::fmt::Display for Size {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{:>8} - {} entries is {} bytes",
                    self.method, self.entries, self.size
                )
            }
        }

        let db_100 = create_dev_db(100);
        let db_1000 = create_dev_db(1_000);
        let db_10000 = create_dev_db(10_000);
        // let db_100000 = create_dev_db(100_000);
        let dbs = vec![
            (100, &db_100),
            (1_000, &db_1000),
            (10_000, &db_10000),
            //     (100_000, &db_100000),
        ];

        let mut timings: Vec<Timing> = Vec::new();
        let mut sizes: Vec<Size> = Vec::new();
        let mut bytes: Vec<u8> = Vec::with_capacity(1_000_000);

        // JSON
        for &(i, db) in &dbs {
            let filename = format!("json_db_{}.json", i);
            // Serialize
            println!("Json {} - serializing", i);
            let now = std::time::Instant::now();
            db.save_dev_with(filename.as_str(), |w, d| {
                serde_json::to_writer(w, d).map_err(Error::from)
            });
            let elapsed = now.elapsed();
            timings.push(Timing::ser(Method::Json, i, elapsed));

            // Deserialize
            println!("Json {} - deserializing", i);
            let now = std::time::Instant::now();
            let loaded = Database::load_dev_with(filename.as_str(), |r| {
                serde_json::from_reader(r).map_err(Error::from)
            })
            .expect("Failure during json deserialization");
            let elapsed = now.elapsed();
            timings.push(Timing::de(Method::Json, i, elapsed));

            // Size
            println!("Json {} - getting byte count", i);
            bytes.clear();
            serde_json::to_writer(&mut bytes, &db).expect("Unable to get bytes from json");
            sizes.push(Size {
                method: Method::Json,
                entries: i,
                size: bytes.len(),
            });
        }

        // CBOR
        for &(i, db) in &dbs {
            let filename = format!("cbor_db_{}.cbor", i);
            // Ser
            println!("Cbor {} - serializing", i);
            let now = std::time::Instant::now();
            db.save_dev_with(filename.as_str(), |w, d| {
                ciborium::ser::into_writer(d, w).map_err(Error::from)
            });
            let elapsed = now.elapsed();
            timings.push(Timing::ser(Method::Cbor, i, elapsed));

            // De
            println!("Cbor {} - deserializing", i);
            let now = std::time::Instant::now();
            let loaded = Database::load_dev_with(filename.as_str(), |r| {
                ciborium::de::from_reader(r).map_err(Error::from)
            })
            .expect("Failure during cbor deserialization");
            let elapsed = now.elapsed();
            timings.push(Timing::de(Method::Cbor, i, elapsed));

            // Size
            println!("Cbor {} - getting byte count", i);
            bytes.clear();
            ciborium::ser::into_writer(db, &mut bytes).expect("Unable to write cbor as bytes");
            sizes.push(Size {
                method: Method::Cbor,
                entries: i,
                size: bytes.len(),
            });
        }

        // Bincode
        // for &(i, db) in &dbs {
        //     let filename = format!("bincode_db_{}.bc", i);
        //     // Ser
        //     println!("Bincode {} - serializing", i);
        //     let now = std::time::Instant::now();
        //     db.save_dev_with(filename.as_str(), |w, d| {
        //         bincode::serialize_into(w, d).map_err(Error::from)
        //     });
        //     let elapsed = now.elapsed();
        //     timings.push(Timing::ser(Method::Bincode, i, elapsed));

        //     // De
        //     println!("Bincode {} - deserializing", i);
        //     let now = std::time::Instant::now();
        //     let loaded = Database::load_dev_with(filename.as_str(), |r| {
        //         bincode::deserialize_from(r).map_err(Error::from)
        //     })
        //     .expect("Failure during bincode deserialization");
        //     let elapsed = now.elapsed();
        //     timings.push(Timing::de(Method::Bincode, i, elapsed));

        //     // Size
        //     println!("Bincode {} - getting byte count", i);
        //     bytes.clear();
        //     bincode::serialize_into(&mut bytes, db).expect("Unable to save bincode to bytes");
        //     sizes.push(Size {
        //         method: Method::Bincode,
        //         entries: i,
        //         size: bytes.len(),
        //     });
        // }

        // MsgPack
        for &(i, db) in &dbs {
            let filename = format!("msgpack_db_{}.rmp", i);
            // Ser
            println!("MsgPack {} - serializing", i);
            let now = std::time::Instant::now();
            db.save_dev_with(filename.as_str(), |mut w, d| {
                rmp_serde::encode::write(&mut w, d).map_err(Error::from)
            });
            let elapsed = now.elapsed();
            timings.push(Timing::ser(Method::Msgpack, i, elapsed));

            // De
            println!("MsgPack {} - deserializing", i);
            let now = std::time::Instant::now();
            let loaded = Database::load_dev_with(filename.as_str(), |r| {
                rmp_serde::from_read(r).map_err(Error::from)
            })
            .expect("Failure during msgpack deserialization");
            let elapsed = now.elapsed();
            timings.push(Timing::de(Method::Msgpack, i, elapsed));

            // Size
            println!("MsgPack {} - getting byte count", i);
            bytes.clear();
            bincode::serialize_into(&mut bytes, db).expect("Unable to save msgpack to bytes");
            sizes.push(Size {
                method: Method::Msgpack,
                entries: i,
                size: bytes.len(),
            });
        }

        // Order Data
        timings.sort_by(|a, b| a.entries.cmp(&b.entries));
        sizes.sort_by(|a, b| a.entries.cmp(&b.entries));

        // Print Results
        println!("Comparison completed.");
        println!("Timings:");
        for timing in &timings {
            println!("\t{}", timing);
        }
        println!("Sizes:");
        for size in &sizes {
            println!("\t{}", size);
        }
    }
}
