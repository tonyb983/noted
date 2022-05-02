// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{
    db::{Database, MultiValueArgs},
    types::api::{Count, Filter, Ordering},
    types::{CreateNote, DeleteNote, Note, UpdateNote},
    Error, Result,
};

pub struct Repository {
    db: Database,
}

impl Repository {
    pub fn new() -> Result<Self> {
        Database::load_dev().map(|db| Self { db })
    }

    pub fn get_all(&self) -> Result<Vec<Note>> {
        Ok(self.db.get_all().to_vec())
    }

    /// TODO: Make this return an iterator maybe?
    pub fn get_all_with(&self, args: &MultiValueArgs) -> Result<Vec<Note>> {
        let mut all = self.db.get_all().to_vec();
        let comp = args.order.comparison();
        let pred = args.filter.predicate();
        all.sort_unstable_by(comp);
        Ok(all
            .iter()
            .filter(pred)
            .take(args.count.to_usize())
            .cloned()
            .collect())
    }

    /// TODO: Interior Mutability?
    pub fn create(&mut self, dto: impl Into<CreateNote>) -> Result<Note> {
        self.db.apply_create(dto)
    }

    /// TODO: Interior Mutability?
    pub fn update(&mut self, dto: impl Into<UpdateNote>) -> Result<bool> {
        self.db.apply_update(dto)
    }

    /// TODO: Interior Mutability?
    pub fn delete(&mut self, dto: impl Into<DeleteNote>) -> Result<bool> {
        self.db.apply_delete(dto)
    }
}
