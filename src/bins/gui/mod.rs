// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod app;
mod backend;
mod hotkey;
mod settings;
mod theme;
mod widgets;

pub fn get_app_theme() -> &'static theme::AppTheme {
    use once_cell::sync::OnceCell;
    static APP_THEME: OnceCell<theme::AppTheme> = OnceCell::new();
    APP_THEME.get_or_init(theme::AppTheme::default)
}

///
/// ## Errors
pub fn execute() -> crate::Result {
    crate::util::profiling::init_profiling("gui");

    // let db = crate::db::Database::load_dev()?;

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Noted Gui App",
        options,
        Box::new(|cc| Box::new(app::GuiApp::new(cc))),
    );
}
