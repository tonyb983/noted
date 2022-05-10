// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use tinyid::TinyId;

use crate::types::Note;

mod with_d {
    use dialoguer::{theme::ColorfulTheme, Select};

    pub fn execute(
        db: &mut crate::db::Database,
        _options: super::super::NoOptions,
    ) -> crate::Result<Option<String>> {
        let all_tags = db.get_all_tags();
        if all_tags.is_empty() {
            println!("There are no tags to display!");
            return Ok(None);
        }
        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Tags:")
            .max_length(10)
            .items(&all_tags)
            .interact_opt()?;

        let tag = if let Some(idx) = choice {
            all_tags[idx]
        } else {
            return Ok(None);
        };

        Ok(Some(tag.clone()))
    }
}

mod with_i {
    use inquire::Select;

    use crate::types::Note;

    pub fn execute(
        db: &mut crate::db::Database,
        _options: super::super::NoOptions,
    ) -> crate::Result<Option<String>> {
        let all_tags = db.get_all_tags();
        if all_tags.is_empty() {
            println!("There are no tags to display!");
            return Ok(None);
        }

        let mut select = Select::new("Tags:", all_tags).with_page_size(10);
        let choice = select.prompt_skippable()?;

        let tag = if let Some(tag) = choice {
            tag
        } else {
            return Ok(None);
        };

        Ok(Some(tag.clone()))
    }
}

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
    match backend {
        super::Backend::Dialoguer => with_d::execute(db, options),
        super::Backend::Inquire => with_i::execute(db, options),
    }
}
