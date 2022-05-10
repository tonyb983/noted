// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::types::Note;

pub fn execute(db: &mut crate::db::Database, backend: super::Backend) -> crate::Result<()> {
    execute_with(db, backend, &None)
}

pub fn execute_with(
    db: &mut crate::db::Database,
    backend: super::Backend,
    options: &Option<Note>,
) -> crate::Result<()> {
    const CMDS: &[&str] = &[
        "View",
        "Edit Title",
        "Edit Content",
        "Edit Tags",
        "Touch",
        "Exit",
    ];
    let mut note = match options {
        Some(n) => n.clone(),
        None => {
            eprintln!("No note selected for editing.");
            return Ok(());
        }
    };

    loop {
        if match backend.select_str("Edit Note:", CMDS)? {
            "View" => {
                super::view_note_with(db, backend, Some(note.clone()))?;
                continue;
            }
            "Edit Title" => edit_title(&mut note, backend)?,
            "Edit Content" => edit_content(&mut note, backend)?,
            "Edit Tags" => edit_tags(&mut note, backend)?,
            "Touch" => touch(&mut note)?,
            "Exit" => {
                break;
            }
            _ => unreachable!(),
        } {
            db.ensure_sync_v2(&mut note);
        }
    }

    Ok(())
}

fn edit_title(note: &mut Note, backend: super::Backend) -> crate::Result<bool> {
    let title = backend.text("Title:", Some(note.title()))?;
    note.set_title(&title);
    Ok(note.dirty())
}

fn edit_content(note: &mut Note, backend: super::Backend) -> crate::Result<bool> {
    use minime::{
        editor::{keybindings::NormalKeybinding, Editor},
        renderer::{
            full::CrosstermRenderer,
            styles::classic::{ClassicFooter, ClassicGutter, ClassicHeader},
        },
        Result,
    };
    let content = {
        let stdout = std::io::stdout();
        let mut lock = stdout.lock();

        let renderer = CrosstermRenderer::render_to(&mut lock)
            .max_height(Some(10))
            .margin(ClassicGutter)
            .header(ClassicHeader {
                message: "Enter on the last line or Esc to submit your input!",
            })
            .footer(ClassicFooter);

        // Print out some prompt using styling options.
        let mut term = Editor::with_renderer(renderer);
        term.set_contents(note.content().as_bytes())
            .map_err(crate::Error::ui)?;
        term.read(NormalKeybinding).map_err(crate::Error::ui)?
    };
    note.set_content(&content);
    Ok(note.dirty())
}

fn edit_tags(note: &mut Note, backend: super::Backend) -> crate::Result<bool> {
    const NOTAG_CMDS: &[&str] = &["Add Tag", "Exit"];
    const TAG_CMDS: &[&str] = &[
        "Add Tag",
        "Edit Tag",
        "Remove Tag",
        "Clear All Tags",
        "Exit",
    ];

    let mut tags = note.tags().to_vec();

    loop {
        match backend.select_str(
            "Tag Operations:",
            if tags.is_empty() {
                NOTAG_CMDS
            } else {
                TAG_CMDS
            },
        )? {
            "Add Tag" => {
                let new_tag = backend.text("New Tag:", None)?;
                tags.push(new_tag);
            }
            "Edit Tag" => {
                let tag = backend.select("Select Tag:", &tags)?;
                let changed = backend.text("Value:", Some(&tag))?;
                tags.retain(|t| t.as_str() != tag);
                tags.push(changed);
            }
            "Remove Tag" => {
                let tag = backend.select("Select Tag:", &tags)?;
                tags.retain(|t| t.as_str() != tag);
            }
            "Clear All Tags" => {
                tags.clear();
            }
            "Exit" => {
                break;
            }
            _ => unreachable!(),
        }
    }

    note.set_tags(tags);
    Ok(note.dirty())
}

fn touch(note: &mut Note) -> crate::Result<bool> {
    note.touch();
    Ok(note.dirty())
}
