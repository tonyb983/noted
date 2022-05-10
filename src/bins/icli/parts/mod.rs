// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod add_note;
pub mod delete_note;
pub mod edit_note;
pub mod list_notes;
pub mod list_tags;
pub mod menu;
pub mod pick_note;
pub mod update_note;
pub mod view_note;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NoOptions;

pub trait Component {
    type Output = ();
    type Options: Default = NoOptions;
    fn execute(db: &mut crate::db::Database, backend: Backend) -> crate::Result<Self::Output> {
        Self::execute_with(db, backend, Self::Options::default())
    }

    fn execute_with(
        db: &mut crate::db::Database,
        backend: Backend,
        options: Self::Options,
    ) -> crate::Result<Self::Output>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Backend {
    Dialoguer,
    Inquire,
}

impl Backend {
    pub fn select_str<'s>(self, prompt: &'_ str, items: &'s [&'s str]) -> crate::Result<&'s str> {
        match self {
            Backend::Dialoguer => {
                let idx =
                    dialoguer::FuzzySelect::with_theme(&dialoguer::theme::ColorfulTheme::default())
                        .with_prompt(prompt)
                        .items(items)
                        .interact()?;
                Ok(items[idx])
            }
            Backend::Inquire => {
                let item = inquire::Select::new(prompt, items.to_vec()).prompt()?;
                Ok(item)
            }
        }
    }

    pub fn select<T: std::fmt::Display + Clone>(
        self,
        prompt: &str,
        items: &[T],
    ) -> crate::Result<T> {
        match self {
            Backend::Dialoguer => {
                let idx =
                    dialoguer::FuzzySelect::with_theme(&dialoguer::theme::ColorfulTheme::default())
                        .with_prompt(prompt)
                        .items(items)
                        .interact()?;
                Ok(items[idx].clone())
            }
            Backend::Inquire => {
                let item = inquire::Select::new(prompt, items.to_vec()).prompt()?;
                Ok(item)
            }
        }
    }

    pub fn confirm(self, prompt: &'_ str) -> crate::Result<bool> {
        match self {
            Backend::Dialoguer => {
                let choice =
                    dialoguer::Confirm::with_theme(&dialoguer::theme::ColorfulTheme::default())
                        .with_prompt(prompt)
                        .interact()?;
                Ok(choice)
            }
            Backend::Inquire => {
                let choice = inquire::Confirm::new(prompt).prompt()?;
                Ok(choice)
            }
        }
    }

    pub fn text(self, prompt: &str, starting_value: Option<&str>) -> crate::Result<String> {
        match self {
            Backend::Dialoguer => {
                if let Some(s) = starting_value {
                    dialoguer::Input::with_theme(&dialoguer::theme::ColorfulTheme::default())
                        .with_prompt(prompt)
                        .with_initial_text(s)
                        .allow_empty(true)
                        .interact()
                        .map_err(crate::Error::from)
                } else {
                    dialoguer::Input::with_theme(&dialoguer::theme::ColorfulTheme::default())
                        .with_prompt(prompt)
                        .allow_empty(true)
                        .interact()
                        .map_err(crate::Error::from)
                }
            }
            Backend::Inquire => {
                if let Some(s) = starting_value {
                    inquire::Text::new(prompt)
                        .with_initial_value(s)
                        .prompt()
                        .map_err(crate::Error::from)
                } else {
                    inquire::Text::new(prompt)
                        .prompt()
                        .map_err(crate::Error::from)
                }
            }
        }
    }

    /// TODO: Allow for more customization here.
    #[allow(clippy::unused_self)]
    pub fn multiline_text(
        self,
        prompt: &str,
        starting_value: Option<&str>,
    ) -> crate::Result<String> {
        use minime::{
            editor::{keybindings::NormalKeybinding, Editor},
            renderer::{
                full::CrosstermRenderer,
                styles::classic::{ClassicFooter, ClassicGutter, ClassicHeader},
            },
            Result,
        };
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
        if let Some(s) = starting_value {
            term.set_contents(s.as_bytes()).map_err(crate::Error::ui)?;
        }
        term.read(NormalKeybinding).map_err(crate::Error::ui)
    }

    pub fn text_array(self, prompt: &str) -> crate::Result<Vec<String>> {
        let mut ts = Vec::new();
        loop {
            let tag = self.text(prompt, None)?;
            if tag.is_empty() {
                break;
            }
            ts.push(tag);
        }
        ts.sort_unstable();
        ts.dedup();
        Ok(ts)
    }
}

pub use add_note::execute as add_note;
pub use delete_note::execute as delete_note;
pub use edit_note::{execute as edit_note, execute_with as edit_note_with};
pub use list_notes::execute as list_notes;
pub use list_tags::execute as list_tags;
pub use menu::execute as menu;
pub use pick_note::{execute as pick_note, execute_with as pick_note_with};
pub use update_note::execute_with as update_note_with;
pub use view_note::execute_with as view_note_with;
