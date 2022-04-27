// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

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
