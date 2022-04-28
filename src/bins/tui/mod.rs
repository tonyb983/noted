// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! `tui` Module
//!
//! This is planned to be a fully interactive terminal interface. It will have a panel on the left that lists all notes,
//! a panel on the right to display the currently selected note, and will allow for editing notes in place. I would like
//! to have these notes render as markdown (at least as much markdown as a terminal can support, think golang something like
//! [`glow`](https://github.com/charmbracelet/glow) library / bin).

/// Runs the Terminal User Interface for the notes application.
///
/// ## Errors
/// - If the underlying process errors.
pub fn run_tui(args: std::env::Args) -> crate::Result<()> {
    println!("Noted TUI. Args: {}", args.collect::<Vec<_>>().join(" "));

    if let Err(err) = crate::term_ui::list::execute() {
        eprintln!("{}", err);
        return crate::Error::unknown(err.to_string()).into();
    }

    Ok(())
}
