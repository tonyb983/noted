// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! `icli` Module
//!
//! This is going to be an **interactive** cli application, along the lines of `gh repo create`. It will not be a full blown TUI,
//! with a full interface and whatnot, but it will query the user in a pretty way, prompting for information until a request or
//! command has been "built", at which point it will execute and display results, very much like the normal `cli`.

use crate::types::Note;

mod parts;

/// # Errors
/// - All the fucking time!
/// # Panics
/// - Exactly 5% of the time it is called, on a completely random basis. Suck it.
pub fn execute() -> crate::Result {
    let mut dev_db = false;
    let mut db = match inquire::Select::new(
        "Choose Database:",
        vec!["Existing Dev Db", "Create New Dev Db", "Empty"],
    )
    .prompt()?
    {
        "Existing Dev Db" => {
            dev_db = true;
            crate::db::Database::load_dev()?
        }
        "Create New Dev Db" => crate::db::Database::create_random(),
        "Empty" => crate::db::Database::empty(),
        _ => unreachable!(),
    };
    // let backend = parts::Backend::Inquire;
    let backend =
        match inquire::Select::new("Choose Backend:", vec!["Dialoguer", "Inquire"]).prompt()? {
            "Dialoguer" => parts::Backend::Dialoguer,
            "Inquire" => parts::Backend::Inquire,
            _ => unreachable!(),
        };

    loop {
        // println!("Running with backend {:?}", backend);
        let choice = parts::menu::execute(backend)?;
        // println!("Choice = {}", choice);

        match choice {
            parts::menu::MenuOptions::CreateNote => {
                parts::add_note(&mut db, backend)?;
            }
            parts::menu::MenuOptions::ViewNote => {
                let choice = parts::pick_note(&mut db, backend)?;
                parts::view_note_with(&mut db, backend, choice)?;
            }
            parts::menu::MenuOptions::ListNotes => {
                let result = parts::list_notes(&mut db, backend)?;
                if let Some(note) = result {
                    println!("You chose note:\n{}", note);
                }
            }
            parts::menu::MenuOptions::UpdateNote => {
                let choice = parts::pick_note(&mut db, backend)?;
                parts::update_note_with(&mut db, backend, &choice)?;
            }
            parts::menu::MenuOptions::ViewTags => {
                let tag = if let Some(tag) = parts::list_tags(&mut db, backend)? {
                    tag
                } else {
                    return Ok(());
                };
                let choice = parts::pick_note_with(
                    &mut db,
                    backend,
                    parts::pick_note::PickNoteOptions {
                        filter: Some(box move |n: &Note| {
                            let s = &tag;
                            n.tags().contains(s)
                        }),
                        ..Default::default()
                    },
                )?;
                parts::view_note_with(&mut db, backend, choice)?;
            }
            parts::menu::MenuOptions::DeleteNote => {
                parts::delete_note(&mut db, backend)?;
            }
            parts::menu::MenuOptions::Exit => {
                println!("Exiting application...");
                break;
            }
        }
    }

    if inquire::Confirm::new("Save Database?").prompt()? {
        if dev_db {
            db.save_dev()?;
        } else {
            let project_dir = std::env::var("CARGO_MANIFEST_DIR")?;
            let data_path = std::path::Path::new(&project_dir).join("data");
            let db_name = loop {
                let mut filename = inquire::Text::new("Enter filename:").prompt()?;
                if filename.is_empty() || filename.contains(['\\', '/', ':', '.']) {
                    println!("Invalid filename, try again.");
                    continue;
                }

                break filename;
            };

            let path = data_path.join(&db_name);
            if !path.exists()
                || inquire::Confirm::new("File already exists, overwrite?").prompt()?
            {
                db.save(&path)?;
            }
        }
    };

    // println!("Database State: {:#?}", db);

    Ok(())
}
