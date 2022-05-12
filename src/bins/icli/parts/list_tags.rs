// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use tinyid::TinyId;

use crate::types::Note;

pub fn execute(
    db: &mut crate::db::Database,
    backend: super::Backend,
) -> crate::Result<Option<String>> {
    execute_with(db, backend, super::NoOptions::default())
}

pub fn execute_with(
    db: &mut crate::db::Database,
    backend: super::Backend,
    options: super::NoOptions,
) -> crate::Result<Option<String>> {
    let tag_counts = db.get_all_tags_and_counts();
    if tag_counts.is_empty() {
        println!("There are no tags to display!");
        return Ok(None);
    }

    let labels = tag_counts
        .iter()
        .map(|(tag, count)| format!("{} ({})", tag, count))
        .collect::<Vec<_>>();

    let choice = backend.select("Tag (Occurrences):", &labels)?;
    Ok(Some(choice))
}
