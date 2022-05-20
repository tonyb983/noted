// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod app;

///
/// ## Errors
pub fn execute() -> crate::Result {
    crate::util::profiling::start_puffin_server(); // NOTE: you may only want to call this if the users specifies some flag or clicks a button!

    let db = crate::db::Database::load_dev()?;

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Noted Gui App",
        options,
        Box::new(|cc| Box::new(app::GuiApp::new(cc, db))),
    );
}
