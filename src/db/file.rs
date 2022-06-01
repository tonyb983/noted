// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{collections::HashSet, path::Path};

use crossbeam_channel::{Receiver, Sender};
use serde::{Deserialize, Serialize};
use tinyid::TinyId;
use uuid::Uuid;

use crate::{
    types::{CreateNote, DeleteNote, Note, NoteDto, UpdateNote},
    util::{persist::Persistence, variadic::OneOrMore},
    DatabaseError, Error, Result, flame_guard,
};

use super::DatabaseMessage;

/// Intermediate type that is used to serialize [`Database`] so that the
/// internal ID-list can be built from the notes as it is constructed and
/// does not need to be serialized as a duplicate.
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
/// 
/// State Changes:
/// - 
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(try_from = "IntermediateDatabase")]
pub struct Database {
    notes: Vec<Note>,
    #[serde(skip)]
    ids: HashSet<TinyId>,
    #[serde(skip)]
    sender: Sender<DatabaseMessage>,
    #[serde(skip)]
    receiver: Receiver<DatabaseMessage>,
    // TODO: I think I should have an option to deactivate message sending since it will not be necessary in all scenarios.
    //       There are two ways to go about this I can see, add a separate `send_messages` field like below, OR we could simply
    //       hold the `sender` and `receiver` fields as an `Option` which would save from initializing them if they won't be used.
    // send_messages: bool,
    // OR
    // channel: Option<(Sender<DatabaseMessage>, Receiver<DatabaseMessage>)>,
    //       The second form would ensure that I don't need to verify that two different fields are Some
}

/// Constructors
impl Database {
    #[tracing::instrument(level = "trace")]
    #[must_use]
    pub fn empty() -> Self {
        crate::profile_guard!("empty", "db::file::Database");

        let (sender, receiver) = crossbeam_channel::unbounded();
        
        Database {
            notes: Vec::new(),
            ids: HashSet::new(),
            sender, receiver,
        }
    }

    /// Create a new [`Database`] from the given slice of [`Note`]s.
    ///
    /// ## Errors
    /// - [`DatabaseError::InvalidId`] if the given notes contains an invalid ID.
    /// - [`DatabaseError::InvalidState`] if a list of IDs cannot be built from the list of notes, usually indicating that the notes contain duplicate or invalid ids.
    #[tracing::instrument(level = "trace", skip(notes))]
    pub fn from_notes(notes: &[Note]) -> Result<Self> {
        // crate::profile_guard!("from_notes", "db::file::Database");

        let (sender, receiver) = crossbeam_channel::unbounded();

        let mut db = Database {
            notes: notes.to_vec(),
            ids: notes.iter().map(Note::id).collect(),
            sender, receiver,
        };
        if let Err(error) = db.init() {
            #[cfg(feature = "trace")] {
                tracing::error!(?error, "database initialization failed");
            }
            // No point in sending a message here as the channel could not possibly have a listener yet.
            return Err(error);
        }
        Ok(db)
    }

    /// Create a new [`Database`] from the given [`Vec<Note>`], taking ownership of each item.
    ///
    /// ## Errors
    /// - [`DatabaseError::InvalidId`] if the given notes contains an invalid ID.
    /// - [`DatabaseError::InvalidState`] if a list of IDs cannot be built from the list of notes, usually indicating that the notes contain duplicate or invalid ids.
    #[tracing::instrument(level = "trace", skip(notes))]
    pub fn from_notes_vec(notes: Vec<Note>) -> Result<Self> {
        // crate::profile_guard!("from_notes_vec", "db::file::Database");

        let ids = notes.iter().map(Note::id).collect();
        let (sender, receiver) = crossbeam_channel::unbounded();
        let mut db = Database { notes, ids, sender, receiver };
        if let Err(error) = db.init() {
            #[cfg(feature = "trace")] {
                tracing::error!(?error, "database initialization failed");
            }
            // No point in sending a message here as the channel could not possibly have a listener yet.
            return Err(error);
        }
        Ok(db)
    }

    /// Attempts to deserialize the given bytes into an instance of [`Database`].
    ///
    /// ## Errors
    /// - [`DatabaseError::InvalidId`] if the given notes contains an invalid ID.
    /// - [`DatabaseError::InvalidState`] if a list of IDs cannot be built from the list of notes, usually indicating that the notes contain duplicate or invalid ids.
    /// - Forwards any errors from [`Persistence::load_from_bytes_default`].
    #[tracing::instrument(level = "trace", skip(bytes))]
    pub fn load_from_bytes(bytes: &[u8]) -> Result<Self> {
        // crate::profile_guard!("load_from_bytes", "db::file::Database");

        Persistence::load_from_bytes_default(bytes)
    }

    /// Attempts to load a [`Database`] from the given filepath.
    ///
    /// ## Errors
    /// - [`DatabaseError::InvalidId`] if the given notes contains an invalid ID.
    /// - [`DatabaseError::InvalidState`] if a list of IDs cannot be built from the list of notes, usually indicating that the notes contain duplicate or invalid ids.
    /// - Forwards any errors from [`Persistence::load_from_file_default`].
    #[tracing::instrument(level = "trace", skip(path), fields(path = path.as_ref().display().to_string().as_str()))]
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        // crate::profile_guard!("load", "db::file::Database");

        let mut db: Self = Persistence::load_from_file_default(path)?;
        if let Err(error) = db.init() {
            #[cfg(feature = "trace")] {
                tracing::error!(?error, "database initialization failed");
            }
            // No point in sending a message here as the channel could not possibly have a listener yet.
            return Err(error);
        }
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
    #[tracing::instrument(level = "trace", skip(self, path), fields(path = path.as_ref().display().to_string().as_str()))]
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result {
        // crate::profile_guard!("save", "db::file::Database");

        match Persistence::save_to_file_default(self, path.as_ref()) {
            Ok(_) => {
                #[cfg(feature = "trace")] {
                    tracing::trace!(path = %path.as_ref().display().to_string(), "database saved to path");
                }
                Self::send_msg(&self.sender, DatabaseMessage::DataSaved { path: path.as_ref().display().to_string() });
                Ok(())
            },
            Err(err) => {
                #[cfg(feature = "trace")] {
                    tracing::error!(error = ?err, "database initialization failed");
                }

                Self::send_error(&self.sender, || err.to_string());

                Err(err)
            },
        }
    }

    /// Attempts to apply the given data transfer object to this [`Database`].
    ///
    /// ## Errors
    /// - See [`Database::apply_create`], [`Database::apply_update`], and [`Database::apply_delete`].
    #[tracing::instrument(level = "trace", skip_all)]
    pub fn apply_dto(&mut self, dto: impl Into<NoteDto>) -> Result<DtoResponse> {
        // crate::profile_guard!("apply_dto", "db::file::Database");

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
    #[tracing::instrument(level = "trace", skip_all)]
    pub fn apply_create(&mut self, create: impl Into<CreateNote>) -> Result<Note> {
        // crate::profile_guard!("apply_create", "db::file::Database");

        let create: CreateNote = create.into();
        let note = Note::create_for(self, create.clone());
        if !self.ids.insert(note.id()) {
            #[cfg(feature = "trace")] {
                tracing::error!(?create, ?note, "duplicate id created for note");
            }
            let error = DatabaseError::DuplicateId(note.id());
            Self::send_error(&self.sender, || error.to_string());
            return Err(error.into());
        }
        Self::send_msg(&self.sender, DatabaseMessage::NoteCreated { dto: create, created: note.clone() });
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
    #[tracing::instrument(level = "trace", skip_all)]
    pub fn apply_update(&mut self, update: impl Into<UpdateNote>) -> Result<bool> {
        // crate::profile_guard!("apply_update", "db::file::Database");

        let update = update.into();

        if let Some(idx) = self.notes.iter().position(|n| n.id() == update.id()) {
            let before = self.notes[idx].clone();
            if !self.notes[idx].update(update.clone()) {
                return Ok(false);
            }
            self.notes[idx].clear_flags();
            let after = self.notes[idx].clone();
            #[cfg(feature = "trace")] {
                tracing::trace!(?before, ?after, "note updated");
            }
            Self::send_msg(&self.sender, DatabaseMessage::NoteUpdated { before, after });
            Ok(true)
        } else {
            let error = DatabaseError::IdNotFound(*update.id());
            Self::send_error(&self.sender, || error.to_string());
            Err(error.into())
        }
    }

    /// Deletes an existing [`Note`] using the information from the [`DeleteNote`] dto.
    ///
    /// TODO: This should probably be a unit return instead of bool.
    ///
    /// ## Errors
    /// - [`DatabaseError::IdNotFound`] if the given ID is not found in this [`Database`].
    #[tracing::instrument(level = "trace", skip_all)]
    pub fn apply_delete(&mut self, delete: impl Into<DeleteNote>) -> Result<bool> {
        // crate::profile_guard!("apply_delete", "db::file::Database");

        let id = *delete.into().id();
        let start = self.notes.len();
        match self.notes.iter().position(|n| n.id() == id) {
            Some(index) => {
                let removed = self.notes.remove(index);
                self.ids.remove(&id);
                #[cfg(feature = "trace")] {
                    tracing::trace!(?removed, "note deleted");
                }
                Self::send_msg(&self.sender, DatabaseMessage::NoteDeleted { deleted: removed});

                Ok(true)
            }
            None => {
                let error = DatabaseError::IdNotFound(id);
                Self::send_error(&self.sender, || error.to_string());
                Err(error.into())
            },
        }
    }

    #[tracing::instrument(level = "trace", skip_all)]
    pub fn ensure_sync<'n>(&mut self, input: impl Into<OneOrMore<&'n mut Note>>) {
        // crate::profile_guard!("ensure_sync", "db::file::Database");
        
        let input = input.into();
        for note in input.into_values() {
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
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn get(&self, id: TinyId) -> Result<&Note> {
        // crate::profile_guard!("get", "db::file::Database");
        
        self.notes
            .iter()
            .find(|n| n.id() == id)
            .ok_or_else(|| DatabaseError::IdNotFound(id).into())
    }

    /// Searches for a [`Note`] with the given ID and returns a cloned version of it.
    ///
    /// ## Errors
    /// - [`DatabaseError::IdNotFound`] if the given ID is not found in this [`Database`].
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn get_clone(&self, id: TinyId) -> Result<Note> {
        // crate::profile_guard!("get_clone", "db::file::Database");
        
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
    #[tracing::instrument(level = "trace", skip_all)]
    pub fn get_and_modify(&mut self, id: TinyId, mut f: impl FnMut(&mut Note)) -> Result {
        // crate::profile_guard!("get_and_modify", "db::file::Database");
        
        if let Some(idx) = self.notes
            .iter()
            .position(|n| n.id() == id) {
                let original = self.notes[idx].clone();
                f(&mut self.notes[idx]);
                if self.notes[idx].pending_delete() {
                    // TODO: Delete this note
                    let deleted = self.notes.remove(idx);
                    self.ids.remove(&id);
                    #[cfg(feature = "trace")] {
                        tracing::trace!(?deleted, "note deleted by get_and_modify");
                    }
                    Self::send_msg(&self.sender, DatabaseMessage::NoteDeleted { deleted });
                } else if self.notes[idx].dirty() {
                    self.notes[idx].clear_flags();
                    let updated = self.notes[idx].clone();
                    #[cfg(feature = "trace")] {
                        tracing::trace!(?original, ?updated, "note updated by get_and_modify");
                    }
                    Self::send_msg(&self.sender, DatabaseMessage::NoteUpdated { before: original, after: updated });
                }
                
                Ok(())
            } else {
                let error = DatabaseError::IdNotFound(id);
                #[cfg(feature = "trace")] {
                    tracing::error!(?error, %id, "note with id not found");
                }
                Self::send_error(&self.sender, || error.to_string());
                Err(error.into())
            }
    }

    /// Returns a slice containing all [`Note`]s in this [`Database`].
    #[tracing::instrument(level = "trace", skip_all)]
    #[must_use]
    pub fn get_all(&self) -> &[Note] {
        // crate::profile_guard!("get_all", "db::file::Database");
        
        &self.notes
    }

    /// TODO: This seems like it's going to be an expensive operation, should we consider keeping a
    ///       tag-list similar to the ID-list we are already storing?
    #[tracing::instrument(level = "trace", skip_all)]
    #[must_use]
    pub fn get_all_tags(&self) -> Vec<&String> {
        // crate::profile_guard!("get_all_tags", "db::file::Database");
        
        let mut tags = self.notes.iter().flat_map(Note::tags).collect::<Vec<_>>();
        tags.sort_unstable();
        tags.dedup();
        tags
    }

    /// TODO: This seems like it's going to be an expensive operation, should we consider keeping a
    ///       tag-list similar to the ID-list we are already storing?
    #[tracing::instrument(level = "trace", skip_all)]
    #[must_use]
    pub fn get_all_tags_v2(&self) -> Vec<&String> {
        // crate::profile_guard!("get_all_tags_v2", "db::file::Database");
        
        let mut tags = std::collections::HashSet::new();
        for note in &self.notes {
            tags.extend(note.tags());
        }
        tags.into_iter().collect()
    }

    #[tracing::instrument(level = "trace", skip_all)]
    #[must_use]
    pub fn get_all_tags_and_counts(&self) -> Vec<(String, usize)> {
        // crate::profile_guard!("get_all_tags_and_counts", "db::file::Database");
        
        let mut map = std::collections::HashMap::new();
        for note in &self.notes {
            for tag in note.tags() {
                *map.entry(tag).or_insert(0usize) += 1;
            }
        }
        map.into_iter().map(|(s, i)| (s.clone(), i)).collect()
    }

    /// Returns a [`Vec`] containing all [`Note`]s in this [`Database`] that match
    /// the given predicate `pred`.
    #[tracing::instrument(level = "trace", skip_all, fields(len))]
    #[must_use]
    pub fn find(&self, pred: impl Fn(&&Note) -> bool) -> Vec<&Note> {
        // crate::profile_guard!("find", "db::file::Database");
        
        let results = self.notes.iter().filter(pred).collect::<Vec<_>>();
        
        #[cfg(feature = "trace")] {
            tracing::Span::current().record("len", &results.len());
        }

        results
    }

    /// Performs a full text search using `query` against all [`Note`]s in this [`Database`].
    #[tracing::instrument(level = "trace", skip(self), fields(len))]
    #[must_use]
    pub fn text_search(&self, query: &str) -> Vec<&Note> {
        // crate::profile_guard!("text_search", "db::file::Database");
        
        let results = self.notes
            .iter()
            .filter(|n| n.full_text_search(query))
            .collect::<Vec<_>>();

        #[cfg(feature = "trace")] {
            tracing::Span::current().record("len", &results.len());
        }

        results
    }

    /// The number of [`Note`]s in this [`Database`].
    #[tracing::instrument(level = "trace", skip(self), fields(len))]
    #[must_use]
    pub fn len(&self) -> usize {
        // crate::profile_guard!("len", "db::file::Database");
        
        let len = self.notes.len();
        #[cfg(feature = "trace")] {
            tracing::Span::current().record("len", &len);
        }
        len
    }

    /// Whether this [`Database`] is currently empty (contains zero [`Note`]s).
    #[tracing::instrument(level = "trace", skip_all)]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        // crate::profile_guard!("is_empty", "db::file::Database");
        
        self.notes.is_empty()
    }

    /// Checks whether the given `id` is currently being used in this [`Database`].
    #[tracing::instrument(level = "trace", skip(self), fields(result))]
    #[must_use]
    pub fn id_in_use(&self, id: TinyId) -> bool {
        // crate::profile_guard!("id_in_use", "db::file::Database");
        
        let result = self.notes.iter().any(|n| n.id() == id);
        #[cfg(feature = "trace")] {
            tracing::Span::current().record("result", &result);
        }
        result
    }

    /// Attempts to create a new [`TinyId`] using [`TinyId::random_against_db`].
    ///
    /// **This does NOT add the returned ID to the db in any way.**
    #[tracing::instrument(level = "trace", skip(self), fields(result))]
    pub(crate) fn create_id(&self) -> TinyId {
        // crate::profile_guard!("create_id", "db::file::Database");
        
        let mut id = TinyId::random();
        while self.ids.contains(&id) {
            id = TinyId::random();
        }
        #[cfg(feature = "trace")] {
            tracing::Span::current().record("result", &id.to_string().as_str());
        }
        id
    }

    /// Inserts the given [`Note`] into the [`Database`], failing if the ID is already in use.
    ///
    /// ## Errors
    /// - [`DatabaseError::DuplicateId`] if the given ID is already in use.
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn insert(&mut self, note: &Note) -> Result {
        // crate::profile_guard!("insert", "db::file::Database");
        
        if self.id_in_use(note.id()) {
            let error = DatabaseError::DuplicateId(note.id());
            #[cfg(feature = "trace")] {
                tracing::error!(insertion = ?note, "duplicate ID attempted to be inserted into db");
            }
            Self::send_error(&self.sender, || error.to_string());
            return Err(error.into());
        }
        self.notes.push(note.clone());
        self.ids.insert(note.id());
        Ok(())
    }

    /// Inserts the given [`Note`] into the [`Database`] if it doesn't already exist, updating it otherwise.
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn upsert(&mut self, note: &Note) {
        // crate::profile_guard!("upsert", "db::file::Database");
        
        if let Err(err) = self.insert(note) && let Error::Database(DatabaseError::DuplicateId(id)) = err {
                self.get_and_modify(id, |n| n.update_from(note))
                    .expect("file::Database::upsert - note already confirmed to exist in db");
        }
    }

    /// Currently this is guaranteed to be `Some`
    #[tracing::instrument(level = "trace", skip(self))]
    #[must_use] 
    pub fn get_receiver(&self) -> Option<Receiver<DatabaseMessage>> {
        Some(self.receiver.clone())
    }
}

/// Private / Crate Methods
impl Database {
    pub(crate) fn save_dev(&self) -> Result {
        crate::profile_guard!("save_dev", "db::file::Database");
        
        let project_dir = std::env::var("CARGO_MANIFEST_DIR")?;
        let path = Path::new(&project_dir).join("data").join("dev.fdb");
        self.save(path)
    }

    pub(crate) fn create_random() -> Self {
        crate::profile_guard!("create_random", "db::file::Database");
        
        let entries = fastrand::usize(500..=1000);
        let mut notes = Vec::new();
        for i in 0..entries {
            let mut note = Note::create((
                format!("Title {}", i),
                format!("Here is the content for note number {}.", i),
            ));
            for i in 0..fastrand::usize(0..20) {
                let tag = format!("tag{}", fastrand::usize(1..=20));
                if !note.tag_matches(&tag) {
                    note.add_tag(tag);
                }
            }
            notes.push(note);
        }

        Database::from_notes_vec(notes).expect("Failed to create random database!")
    }

    #[tracing::instrument(skip(self, writer))]
    pub(crate) fn save_dev_with(
        &self,
        filename: &str,
        writer: impl FnOnce(std::io::BufWriter<std::fs::File>, &Self) -> Result,
    ) -> Result {
        use std::fs::File;
        use std::io::Write;
        // crate::profile_guard!("save_dev_with", "db::file::Database");
        
        let project_dir = std::env::var("CARGO_MANIFEST_DIR")?;
        let path = Path::new(&project_dir).join("data").join(filename);
        let mut file = File::create(path)?;
        let mut buf_writer = std::io::BufWriter::new(file);
        writer(buf_writer, self)?;
        Ok(())
    }

    #[tracing::instrument]
    pub(crate) fn load_dev() -> Result<Self> {
        crate::profile_guard!("load_dev", "db::file::Database");
        
        let project_dir = std::env::var("CARGO_MANIFEST_DIR")?;
        let path = Path::new(&project_dir).join("data").join("dev.fdb");
        Self::load(path)
    }

    #[tracing::instrument(skip(f))]
    pub(crate) fn load_dev_with(
        filename: &str,
        f: impl FnOnce(std::io::BufReader<std::fs::File>) -> Result<Self>,
    ) -> Result<Self> {
        use std::fs::File;
        use std::io::Read;

        crate::profile_guard!("load_dev_with", "db::file::Database");
        
        let project_dir = std::env::var("CARGO_MANIFEST_DIR")?;
        let path = Path::new(&project_dir).join("data").join(filename);
        let mut file = File::open(path)?;
        let mut buf_reader = std::io::BufReader::new(file);
        let db: Self = f(buf_reader)?;
        Ok(db)
    }

    #[tracing::instrument(skip(self))]
    fn validate(&mut self) -> Result {
        const ID_NOTE_MISMATCH_MSG: &str = "register_ids could not successfully build id list";
        
        if self.ids.len() != self.notes.len() {
            self.register_ids();
        }

        if self.ids.len() != self.notes.len() {
            #[cfg(feature = "trace")] {
                tracing::error!(ids = ?self.ids, notes = ?self.notes, "Failed to synchronize note-list and id-list");
            }
            
            let msg = ID_NOTE_MISMATCH_MSG.to_string();
            Self::send_error(&self.sender, || msg.clone());
            
            return Err(DatabaseError::InvalidState(msg).into());
        }

        if self.notes.iter().any(|n| !n.id().is_valid()) {
            return Err(DatabaseError::InvalidId.into());
        }
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    fn register_ids(&mut self) {
        // crate::profile_guard!("register_ids", "db::file::Database");
        
        self.ids.clear();
        self.ids = HashSet::with_capacity(self.notes.len());
        for note in &self.notes {
            self.ids.insert(note.id());
        }
    }

    #[tracing::instrument(skip(self))]
    fn init(&mut self) -> Result {
        // crate::profile_guard!("init", "db::file::Database");
        
        self.validate()?;
        Ok(())
    }

    fn send_msg(sender: &Sender<DatabaseMessage>, msg: DatabaseMessage) {
        if let Err(err) = sender.send(msg) {
            tracing::error!(error = ?err, "Failed to send database message");
        }
    }

    /// Making this take a lambda instead of a string should ensure that it is lazily evaluated which
    /// would give flexibility when I made message sending optional.
    fn send_error(sender: &Sender<DatabaseMessage>, err: impl FnOnce() -> String) {
        Self::send_msg(sender, DatabaseMessage::Error { msg: err() });
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

    #[no_coverage]
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

    #[no_coverage]
    /// Saves the given database to the standard dev location
    fn save_dev_db(db: &Database) -> Result {
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

    #[test]
    #[ignore]
    #[no_coverage]
    fn create_dev_db_1000() {
        save_dev_db(&create_dev_db(1000));
    }

    #[test]
    #[ignore]
    #[no_coverage]
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
    #[no_coverage]
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

    #[test]
    #[ignore]
    #[no_coverage]
    fn get_tags() {
        crate::profile_guard!("", "db::file::Database");

        #[cfg(not(feature = "flame"))]
        let now = std::time::Instant::now();
        let db = {
            #[cfg(feature = "flame")]
            let _guard = ::flame::start_guard("create_dev_db(10_000)");
            create_dev_db(10_000)
        };
        #[cfg(not(feature = "flame"))]
        let db_elapsed = now.elapsed();

        #[cfg(not(feature = "flame"))]
        let now = std::time::Instant::now();
        let tags = {
            #[cfg(feature = "flame")]
            let _guard = ::flame::start_guard("get_all_tags");
            db.get_all_tags()
        };
        #[cfg(not(feature = "flame"))]
        let tags_elapsed = now.elapsed();

        #[cfg(not(feature = "flame"))]
        let now = std::time::Instant::now();
        let tags_v2 = {
            #[cfg(feature = "flame")]
            let _guard = ::flame::start_guard("get_all_tags_v2");
            db.get_all_tags_v2()
        };
        #[cfg(not(feature = "flame"))]
        let tags_v2_elapsed = now.elapsed();

        #[cfg(not(feature = "flame"))]
        let now = std::time::Instant::now();
        let tag_counts = {
            #[cfg(feature = "flame")]
            let _guard = ::flame::start_guard("get_all_tags_and_counts");
            db.get_all_tags_and_counts()
        };
        #[cfg(not(feature = "flame"))]
        let counts_elapsed = now.elapsed();

        assert_eq!(tags.len(), tag_counts.len());
        assert_eq!(tags.len(), tags_v2.len());

        crate::flame_dump!(html, "file.Database.get_tags");

        #[cfg(not(feature = "flame"))]
        println!(
            "Getting Database Tags:\n\t{:<15} took {:>10?}\n\t{:<15} took {:>10?}\n\t{:<15} took {:>10?}\n\t{:<15} took {:>10?}", 
            "create_dev_db", 
            db_elapsed, 
            "all tags", 
            tags_elapsed,
            "all tags v2", 
            tags_v2_elapsed, 
            "tags & counts", 
            counts_elapsed
        );
    }
}
