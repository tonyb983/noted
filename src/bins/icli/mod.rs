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

mod parts;

/// # Errors
/// - All the fucking time!
/// # Panics
/// - Exactly 5% of the time it is called, on a completely random basis. Suck it.
pub fn execute() -> crate::Result {
    let mut db = match inquire::Select::new("Choose Database:", vec!["Dev (Random Data)", "Empty"])
        .prompt()?
    {
        "Dev (Random Data)" => crate::db::Database::load_dev()?,
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

    // println!("Running with backend {:?}", backend);
    let choice = parts::menu::execute(backend)?;
    // println!("Choice = {}", choice);

    match choice {
        parts::menu::MenuOptions::CreateNote => {
            parts::add_note(&mut db, backend)?;
        }
        parts::menu::MenuOptions::ViewNote => todo!("View Note not implemented."),
        parts::menu::MenuOptions::ListNotes => {
            let result = parts::list_notes(&mut db, backend)?;
            if let Some(note) = result {
                println!("You chose note:\n{}", note);
            }
        }
        parts::menu::MenuOptions::UpdateNote => {
            return crate::Error::Unknown(
                "Update Note is not yet implemented idiot, you should already know that..."
                    .to_string(),
            )
            .into();
        }
        parts::menu::MenuOptions::ViewTags => todo!("View Tags not implemented"),
        parts::menu::MenuOptions::DeleteNote => todo!("Delete Note not implemented"),
        parts::menu::MenuOptions::Exit => {
            println!("Exiting application...");
        }
    }

    // println!("Database State: {:#?}", db);

    Ok(())
}
