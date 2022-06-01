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
    option: &Option<Note>,
) -> crate::Result<()> {
    use minime::{
        editor::{keybindings::NormalKeybinding, Editor},
        renderer::{
            full::CrosstermRenderer,
            styles::classic::{ClassicFooter, ClassicGutter, ClassicHeader},
        },
        Result,
    };

    let mut note = match option {
        Some(n) => n.clone(),
        None => {
            eprintln!("No note selected!");
            return Ok(());
        }
    };

    let title: String = match backend {
        super::Backend::Dialoguer => {
            dialoguer::Input::with_theme(&dialoguer::theme::ColorfulTheme::default())
                .with_initial_text(note.title())
                .with_prompt("Title:")
                .interact()?
        }
        super::Backend::Inquire => inquire::Text::new("Title:")
            .with_initial_value(note.title())
            .prompt()?,
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

    note.set_title(title.as_str());
    note.set_content(content.as_str());
    db.ensure_sync(&mut note);

    Ok(())
}
