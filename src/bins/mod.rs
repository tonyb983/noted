// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::term_ui;

pub mod cli;
pub mod icli;
pub mod tui;

pub fn run_canvas() {
    if let Err(err) = term_ui::canvas::execute() {
        eprintln!("An error has occurred: {}", err);
    }
}
