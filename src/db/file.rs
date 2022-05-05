// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{collections::HashSet, path::Path};

use serde::{Deserialize, Serialize};
use tinyid::TinyId;
use uuid::Uuid;

use crate::{
    types::{CreateNote, DeleteNote, Note, NoteDto, UpdateNote},
    util::persist::Persistence,
    DatabaseError, Error, Result,
};

/// Intermediate type that is used to serialize [`Database`] so that the
/// internal ID-list can be built from the notes as it is constructed.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
struct IntermediateDatabase {
    notes: Vec<Note>,
}

impl TryFrom<IntermediateDatabase> for Database {
    type Error = Error;

    fn try_from(value: IntermediateDatabase) -> Result<Self> {
        Self::from_notes_vec(value.notes)
    }
}

/// Implementation of a Database that stores data in a file.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(try_from = "IntermediateDatabase")]
pub struct Database {
    notes: Vec<Note>,
    // #[serde(skip)]
    ids: HashSet<TinyId>,
}

/// Constructors
impl Database {
    #[must_use]
    pub fn empty() -> Self {
        Database {
            notes: Vec::new(),
            ids: HashSet::new(),
        }
    }

    /// Create a new [`Database`] from the given slice of [`Note`]s.
    ///
    /// ## Errors
    /// - [`DatabaseError::InvalidId`] if the given notes contains an invalid ID.
    /// - [`DatabaseError::InvalidState`] if a list of IDs cannot be built from the list of notes, usually indicating that the notes contain duplicate or invalid ids.
    pub fn from_notes(notes: &[Note]) -> Result<Self> {
        let mut db = Database {
            notes: notes.to_vec(),
            ids: notes.iter().map(Note::id).collect(),
        };
        db.init()?;
        Ok(db)
    }

    /// Create a new [`Database`] from the given [`Vec<Note>`], taking ownership of each item.
    ///
    /// ## Errors
    /// - [`DatabaseError::InvalidId`] if the given notes contains an invalid ID.
    /// - [`DatabaseError::InvalidState`] if a list of IDs cannot be built from the list of notes, usually indicating that the notes contain duplicate or invalid ids.
    pub fn from_notes_vec(notes: Vec<Note>) -> Result<Self> {
        let ids = notes.iter().map(Note::id).collect();
        let mut db = Database { notes, ids };
        db.init()?;
        Ok(db)
    }

    /// Attempts to deserialize the given bytes into an instance of [`Database`].
    ///
    /// ## Errors
    /// - [`DatabaseError::InvalidId`] if the given notes contains an invalid ID.
    /// - [`DatabaseError::InvalidState`] if a list of IDs cannot be built from the list of notes, usually indicating that the notes contain duplicate or invalid ids.
    /// - Forwards any errors from [`Persistence::load_from_bytes_default`].
    pub fn load_from_bytes(bytes: &[u8]) -> Result<Self> {
        Persistence::load_from_bytes_default(bytes)
    }

    /// Attempts to load a [`Database`] from the given filepath.
    ///
    /// ## Errors
    /// - [`DatabaseError::InvalidId`] if the given notes contains an invalid ID.
    /// - [`DatabaseError::InvalidState`] if a list of IDs cannot be built from the list of notes, usually indicating that the notes contain duplicate or invalid ids.
    /// - Forwards any errors from [`Persistence::load_from_file_default`].
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut db: Self = Persistence::load_from_file_default(path)?;
        db.init()?;
        Ok(db)
    }
}

/// Public Methods
impl Database {
    pub const DEFAULT_UPDATE_POLICY: UpdateFailurePolicy = UpdateFailurePolicy::AllOrNothing;

    /// Attempts to serialize this [`Database`] into bytes and writes them to a file at the given path.
    /// If the file exists it will be overwritten, and if it does not exist it will be created.
    ///
    /// ## Errors
    /// - See [`Persistence::save_to_file_default`].
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        Persistence::save_to_file_default(self, path)
    }

    /// Attempts to apply the given data transfer object to this [`Database`].
    ///
    /// ## Errors
    /// - See [`Database::apply_create`], [`Database::apply_update`], and [`Database::apply_delete`].
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

    /// Creates a new [`Note`] using the information from the [`CreateNote`] dto. The returned
    /// result contains the newly created [`Note`] upon success (for getting the `id`, for example).
    ///
    /// ## Errors
    /// - [`DatabaseError::DuplicateId`] if the given ID is already contained in this [`Database`].
    pub fn apply_create(&mut self, create: impl Into<CreateNote>) -> Result<Note> {
        let create = create.into();
        let note = Note::create_for(self, create);
        if !self.ids.insert(note.id()) {
            return Err(DatabaseError::DuplicateId(note.id()).into());
        }
        self.notes.push(note.clone());
        Ok(note)
    }

    /// Updates an existing [`Note`] using the information from the [`UpdateNote`] dto. The returned
    /// result contains `true` if the [`Note`] was found and changed, `false` if it was found but there
    /// were no changes detected, or an error if the [`Note`] could not be found, or another problem
    /// was encountered..
    ///
    /// ## Errors
    /// - [`DatabaseError::IdNotFound`] if the given ID is not found in this [`Database`].
    pub fn apply_update(&mut self, update: impl Into<UpdateNote>) -> Result<bool> {
        let update = update.into();
        let mut note = self
            .notes
            .iter_mut()
            .find(|n| n.id() == *update.id())
            .ok_or_else(|| DatabaseError::IdNotFound(*update.id()))?;
        Ok(note.update(update))
    }

    /// Deletes an existing [`Note`] using the information from the [`DeleteNote`] dto.
    ///
    /// TODO: This should probably be a unit return instead of bool.
    ///
    /// ## Errors
    /// - [`DatabaseError::IdNotFound`] if the given ID is not found in this [`Database`].
    pub fn apply_delete(&mut self, delete: impl Into<DeleteNote>) -> Result<bool> {
        let id = *delete.into().id();
        let start = self.notes.len();
        match self.notes.iter().position(|n| n.id() == id) {
            Some(index) => {
                self.notes.remove(index);
                self.ids.remove(&id);
                Ok(true)
            }
            None => Err(DatabaseError::IdNotFound(id).into()),
        }
    }

    /// Checks the given list of [`Note`]s, and applies any changes, modifications,
    /// or pending deletions that are detected. This call will reset the flags on
    /// each [`Note`], which is why it requires a mutable reference to each.
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

            self.upsert(note);
            note.clear_flags();
        }
    }

    /// Searches for a [`Note`] with the given ID.
    ///
    /// ## Errors
    /// - [`DatabaseError::IdNotFound`] if the given ID is not found in this [`Database`].
    pub fn get(&self, id: TinyId) -> Result<&Note> {
        self.notes
            .iter()
            .find(|n| n.id() == id)
            .ok_or_else(|| DatabaseError::IdNotFound(id).into())
    }

    /// Searches for a [`Note`] with the given ID and returns a cloned version of it.
    ///
    /// ## Errors
    /// - [`DatabaseError::IdNotFound`] if the given ID is not found in this [`Database`].
    pub fn get_clone(&self, id: TinyId) -> Result<Note> {
        self.notes
            .iter()
            .find(|n| n.id() == id)
            .cloned()
            .ok_or_else(|| DatabaseError::IdNotFound(id).into())
    }

    /// Uses the given function `f` to modify the [`Note`] that has the given [`TinyId`].
    ///
    /// ## Errors
    /// - [`DatabaseError::IdNotFound`] if the given ID is not found in this [`Database`].
    pub fn get_and_modify(&mut self, id: TinyId, f: impl FnMut(&mut Note)) -> Result<()> {
        self.notes
            .iter_mut()
            .find(|n| n.id() == id)
            .map(f)
            .ok_or_else(|| DatabaseError::IdNotFound(id).into())
    }

    /// Returns a slice containing all [`Note`]s in this [`Database`].
    #[must_use]
    pub fn get_all(&self) -> &[Note] {
        &self.notes
    }

    /// Returns a [`Vec`] containing all [`Note`]s in this [`Database`] that match
    /// the given predicate `pred`.
    #[must_use]
    pub fn find(&self, pred: impl Fn(&&Note) -> bool) -> Vec<&Note> {
        self.notes.iter().filter(pred).collect::<Vec<_>>()
    }

    /// Performs a full text search using `query` against all [`Note`]s in this [`Database`].
    #[must_use]
    pub fn text_search(&self, query: &str) -> Vec<&Note> {
        self.notes
            .iter()
            .filter(|n| n.full_text_search(query))
            .collect::<Vec<_>>()
    }

    /// The number of [`Note`]s in this [`Database`].
    #[must_use]
    pub fn len(&self) -> usize {
        self.notes.len()
    }

    /// Whether this [`Database`] is currently empty (contains zero [`Note`]s).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.notes.is_empty()
    }

    /// Checks whether the given `id` is currently being used in this [`Database`].
    #[must_use]
    pub fn id_in_use(&self, id: TinyId) -> bool {
        self.notes.iter().any(|n| n.id() == id)
    }

    /// Attempts to create a new [`TinyId`] using [`TinyId::random_against_db`].
    ///
    /// **This does NOT add the returned ID to the db in any way.**
    pub(crate) fn create_id(&self) -> TinyId {
        let mut id = TinyId::random();
        while self.ids.contains(&id) {
            id = TinyId::random();
        }
        id
    }

    /// Inserts the given [`Note`] into the [`Database`], failing if the ID is already in use.
    ///
    /// ## Errors
    /// - [`DatabaseError::DuplicateId`] if the given ID is already in use.
    pub fn insert(&mut self, note: &Note) -> Result<()> {
        if self.id_in_use(note.id()) {
            return Err(DatabaseError::DuplicateId(note.id()).into());
        }
        self.notes.push(note.clone());
        self.ids.insert(note.id());
        Ok(())
    }

    /// Inserts the given [`Note`] into the [`Database`] if it doesn't already exist, updating it otherwise.
    pub fn upsert(&mut self, note: &Note) {
        if let Err(err) = self.insert(note) && let Error::Database(DatabaseError::DuplicateId(id)) = err {
                self.get_and_modify(id, |n| n.update_from(note))
                    .expect("file::Database::upsert - note already confirmed to exist in db");
        }
    }
}

/// Private / Crate Methods
impl Database {
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

    fn validate(&mut self) -> Result<()> {
        if self.ids.len() != self.notes.len() {
            self.register_ids();
        }

        if self.ids.len() != self.notes.len() {
            return Err(DatabaseError::InvalidState(
                "register_ids could not successfully build id list".to_string(),
            )
            .into());
        }

        if self.notes.iter().any(|n| !n.id().is_valid()) {
            return Err(DatabaseError::InvalidId.into());
        }
        Ok(())
    }

    fn register_ids(&mut self) {
        self.ids.clear();
        self.ids = HashSet::with_capacity(self.notes.len());
        for note in &self.notes {
            self.ids.insert(note.id());
        }
    }

    fn init(&mut self) -> Result<()> {
        self.validate()?;
        Ok(())
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
    use crate::Method;

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

    #[allow(clippy::too_many_lines, clippy::to_string_in_format_args)]
    #[test]
    #[ignore]
    fn serde_compare() {
        use std::io::{Read, Write};
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        enum Op {
            Ser,
            De,
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
            size: u64,
        }
        impl std::fmt::Display for Op {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Op::Ser => write!(f, "write"),
                    Op::De => write!(f, "read"),
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

        let cargo_dir = std::env::var("CARGO_MANIFEST_DIR")
            .expect("Unable to get the value of CARGO_MANIFEST_DIR");
        let save_dir = std::path::Path::new(&cargo_dir)
            .to_path_buf()
            .join("data")
            .join("testing");
        assert!(save_dir.exists(), "./data/testing does not exist!");
        for &(i, db) in &dbs {
            for method in Method::working_methods() {
                let file_name = format!("test-{}-{}.db", method, i);
                let save_path = save_dir.clone().join(&file_name);
                let now = std::time::Instant::now();
                let result = Persistence::save_to_file(&db, &save_path, method);
                let elapsed = now.elapsed();
                assert!(result.is_ok(), "Failed to save db-{} using {}", i, method);
                timings.push(Timing::ser(method, i, elapsed));

                let now = std::time::Instant::now();
                let result: Result<Database> = Persistence::load_from_file(&save_path, method);
                let elapsed = now.elapsed();
                assert!(result.is_ok(), "Failed to load db-{} using {}", i, method);
                timings.push(Timing::de(method, i, elapsed));
                let reversed = result.unwrap();
                assert_eq!(db.len(), reversed.len());

                let file = std::fs::File::open(&save_path)
                    .unwrap_or_else(|_| panic!("Unable to open file {}", save_path.display()));
                let size = file.metadata().expect("Unable to get file metadata").len();
                sizes.push(Size {
                    method,
                    entries: i,
                    size,
                });
            }
        }

        // Order Data
        timings.sort_by(|a, b| a.entries.cmp(&b.entries));
        sizes.sort_by(|a, b| a.entries.cmp(&b.entries));

        // Print Results
        println!("Comparison completed.");
        println!("Timings:");
        println!(
            "|{:^10}|{:^10}|{:^10}|{:^10}|",
            "Method", "Entries", "Op", "Time"
        );
        println!(
            "|{:^10}|{:^10}|{:^10}|{:^10}|",
            "-".repeat(10),
            "-".repeat(10),
            "-".repeat(10),
            "-".repeat(10)
        );
        for timing in &timings {
            println!(
                "|{:<10}|{:^10}|{:^10}|{:>10?}|",
                timing.method.to_string(),
                timing.entries,
                timing.op.to_string(),
                timing.elapsed
            );
        }
        println!("Sizes:");
        println!("|{:^10}|{:^10}|{:^10}|", "Method", "Entries", "Bytes");
        println!(
            "|{:^10}|{:^10}|{:^10}|",
            "-".repeat(10),
            "-".repeat(10),
            "-".repeat(10)
        );
        for size in &sizes {
            println!(
                "|{:<10}|{:^10}|{:>10}|",
                size.method.to_string(),
                size.entries,
                size.size
            );
        }
    }
}
