// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::path::{Path, PathBuf};

use crossbeam_channel::{Receiver, Sender};
use egui_toast::Toasts;
use tinyid::TinyId;

use crate::types::Note;

use super::{
    backend::{Backend, ToBackend, ToFrontend},
    settings::{AppSettings, AppSettingsUi},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum AppState {
    NoDatabase,
    DatabaseOpen,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ExitState {
    Running,
    ExitRequested,
    Exiting,
}

pub struct GuiApp {
    notes: Vec<Note>,
    active_note: Option<Note>,
    active_tag: Option<usize>,
    state: AppState,
    settings: AppSettings,
    settings_open: bool,
    front_tx: Option<Sender<ToBackend>>,
    back_rx: Option<Receiver<ToFrontend>>,
    error_log: Vec<String>,
    exit_state: ExitState,
}

impl GuiApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let settings = AppSettings::load_or_create().expect("Unable to load/create app settings");
        Self::setup_custom_fonts(&cc.egui_ctx);
        let notes = Vec::new();

        let (front_tx, front_rx) = crossbeam_channel::unbounded();
        let (back_tx, back_rx) = crossbeam_channel::unbounded();

        let frame_clone = cc.egui_ctx.clone();
        std::thread::spawn(move || {
            Backend::new(back_tx, front_rx, frame_clone).init();
        });

        front_tx
            .send(ToBackend::Startup)
            .expect("Unable to send startup message to backend");

        if settings.load_default_on_start {
            if settings.default_database.exists() {
                front_tx
                    .send(ToBackend::OpenDatabase {
                        path: settings.default_database.clone(),
                    })
                    .expect("Unable to send open database message to backend");
            } else {
                front_tx
                    .send(ToBackend::CreateDatabase {
                        path: settings.default_database.clone(),
                    })
                    .expect("Unable to send open database message to backend");
            }
        }

        Self {
            notes,
            active_note: None,
            active_tag: None,
            front_tx: Some(front_tx),
            back_rx: Some(back_rx),
            state: AppState::NoDatabase,
            settings,
            settings_open: false,
            error_log: Vec::new(),
            exit_state: ExitState::Running,
        }
    }

    fn setup_custom_fonts(ctx: &egui::Context) {
        // Start with the default fonts (we will be adding to them rather than replacing them).
        let mut fonts = egui::FontDefinitions::default();

        // Install my own font (maybe supporting non-latin characters).
        // .ttf and .otf files supported.
        fonts.font_data.insert(
            "monofur_nf".to_owned(),
            egui::FontData::from_static(include_bytes!("../../../assets/fonts/monofur_nf.ttf")),
        );
        fonts.font_data.insert(
            "calisto".to_owned(),
            egui::FontData::from_static(include_bytes!("../../../assets/fonts/calisto.ttf")),
        );

        // Put my font first (highest priority) for proportional text:
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "calisto".to_owned());

        // Put my font as last fallback for monospace:
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .insert(0, "monofur_nf".to_owned());

        // Tell egui to use these fonts:
        ctx.set_fonts(fonts);
    }

    fn new_db(&mut self, path: PathBuf) {
        if let Some(tx) = &self.front_tx {
            tx.send(ToBackend::CreateDatabase { path })
                .expect("Unable to send create database message to backend");
        }
    }

    fn load_db(&mut self, path: PathBuf) {
        if let Some(tx) = &self.front_tx {
            tx.send(ToBackend::OpenDatabase { path })
                .expect("Unable to send open database message to backend");
        }
    }

    fn save_data(&mut self) {
        if let Some(tx) = &self.front_tx {
            tx.send(ToBackend::SaveData)
                .expect("Unable to send timed save message to backend");
        }
    }

    fn set_active_note(&mut self, note: Note) {
        if let Some(ref current) = self.active_note {
            if current.id() == note.id() {
                return;
            }
        }
        self.active_note = Some(note);
        self.active_tag = None;
    }

    fn set_editing_tag(&mut self, index: usize) {
        self.active_tag = Some(index);
    }

    fn set_adding_tag(&mut self) {
        if let Some(ref mut note) = self.active_note {
            note.add_tag("New Tag".to_string());
            self.active_tag = Some(note.tag_len() - 1);
        }
    }

    fn clear_editing_tag(&mut self) {
        self.active_tag = None;
    }

    fn new_note(&mut self) {
        if let Some(tx) = &self.front_tx {
            tx.send(ToBackend::CreateNote {
                dto: (String::from("New Note"), String::new()).into(),
            })
            .expect("Unable to send new note message to backend");
        }
    }

    fn update_active_note(&self) {
        if let Some(ref note) = self.active_note {
            if let Some(ref tx) = self.front_tx {
                tx.send(ToBackend::UpdateNote { note: note.clone() })
                    .expect("Unable to send update note message to backend");
            }
        }
    }

    fn delete_note(&mut self, id: TinyId) {
        if let Some(ref active) = self.active_note {
            if active.id() == id {
                self.active_note = None;
                self.active_tag = None;
            }
        }
        if let Some(tx) = &self.front_tx {
            tx.send(ToBackend::DeleteNote { id })
                .expect("Unable to send delete note message to backend");
        }
    }

    fn render_db_loaded(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let mut note_changed = false;
        let mut delete_requested = None;

        let mut side_panel = egui::SidePanel::left("note_list_panel")
            .width_range(50.0..=200.0)
            .default_width(100.0);
        side_panel.show(ctx, |ui| {
            crate::profile_guard!("SidePanel", "gui::GuiApp::update");
            let side_width = ui.available_width();
            let side_width = side_width.min(100.0);
            ui.allocate_ui_with_layout(
                egui::Vec2::new(side_width, 50.),
                egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                |ui| {
                    ui.heading(egui::RichText::new("Notes").heading());
                    if ui.button("New Note").clicked() {
                        self.new_note();
                    }
                },
            );
            ui.separator();
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (i, note) in self.notes.iter().enumerate() {
                    let button = ui.button(note.title()).context_menu(|ui| {
                        if ui.small_button("Delete this note.").clicked() {
                            delete_requested = Some(note.id());
                            ui.close_menu();
                        }
                    });
                    if button.clicked() {
                        self.active_note = Some(note.clone());
                    }
                }
            });
        });
        let mut active = self.active_note.clone();
        egui::CentralPanel::default().show(ctx, |ui| {
            crate::profile_guard!("CentralPanel", "gui::GuiApp::update");
            match active {
                Some(ref mut note) => {
                    note_changed = note_changed || Self::render_title(ui, note);
                    note_changed = note_changed || Self::render_content(ui, note);
                    note_changed = note_changed || self.render_tags(ui, note);
                }
                None => {
                    ui.centered_and_justified(|ui| ui.label("Select a note to start editing!"));
                }
            }
        });

        if note_changed {
            self.active_note = active;
            self.update_active_note();
        }

        if let Some(id) = delete_requested {
            self.delete_note(id);
        }
    }

    fn render_no_db(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.allocate_ui_with_layout(
                egui::Vec2::new(200., 200.),
                egui::Layout::centered_and_justified(egui::Direction::TopDown),
                |ui| {
                    ui.label("No Database Open...");
                    if ui.button("New").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .set_directory(env!("CARGO_MANIFEST_DIR"))
                            .add_filter("Note Data", &["db", "fdb", "data", "noted"])
                            .save_file()
                        {
                            self.new_db(path);
                        }
                    }
                    if ui.button("Open").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .set_directory(env!("CARGO_MANIFEST_DIR"))
                            .add_filter("Note Data", &["db", "fdb", "data", "noted"])
                            .pick_file()
                        {
                            self.load_db(path);
                        }
                    }
                },
            );
        });
    }

    fn render_settings(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let changed = AppSettingsUi::render(ui, &mut self.settings);
            if changed {
                if let Err(err) = self.settings.save_default() {
                    self.error_log.push(err.to_string());
                }
            }
        });
    }

    fn render_title(ui: &mut egui::Ui, note: &mut Note) -> bool {
        let mut note_title = note.title().to_string();
        let title_response = ui.text_edit_singleline(&mut note_title);
        if title_response.changed() {
            note.set_title(note_title.as_str());
            return true;
        }

        false
    }

    fn render_content(ui: &mut egui::Ui, note: &mut Note) -> bool {
        let mut note_content = note.content().to_string();
        let content_response = ui.code_editor(&mut note_content);
        if content_response.changed() {
            note.set_content(note_content.as_str());
            return true;
        }
        false
    }

    #[allow(clippy::collapsible_if)]
    #[allow(clippy::collapsible_else_if)]
    fn render_tags(&mut self, ui: &mut egui::Ui, note: &mut Note) -> bool {
        let mut note_tags = note.tags().to_vec();
        let mut removals = Vec::new();
        let mut tags_changed = false;
        ui.label("Tags:");

        match self.active_tag {
            None => {
                for (i, tag) in note_tags.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        let tag_response =
                            ui.add(egui::Label::new(tag.as_str()).sense(egui::Sense::click()));
                        if tag_response.double_clicked() {
                            self.set_editing_tag(i);
                        }
                        if ui.small_button("x").clicked() {
                            removals.push(i);
                        }
                    });
                }

                if ui.small_button("+").clicked() {
                    self.set_adding_tag();
                }
            }
            Some(idx) => {
                for (i, tag) in note_tags.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        if i == idx {
                            let edit_response = ui.text_edit_singleline(tag);
                            if edit_response.changed() {
                                tags_changed = true;
                            } else if edit_response.lost_focus() {
                                self.clear_editing_tag();
                            }
                        } else {
                            if ui.button(tag.as_str()).clicked() {
                                self.set_editing_tag(i);
                            }
                        }
                        if ui.small_button("x").clicked() {
                            removals.push(i);
                        }
                    });
                }

                if ui.small_button("+").clicked() {
                    self.set_adding_tag();
                }
            }
        }

        if !removals.is_empty() {
            removals.sort_unstable();
            removals.reverse();
            for i in removals {
                note_tags.remove(i);
            }
            tags_changed = true;
        }

        if tags_changed {
            note.set_tags(note_tags);
            return true;
        }

        false
    }

    fn render_error_log(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::bottom("error_log")
            .default_height(100.)
            .height_range(20.0..=300.0)
            .resizable(true)
            .show(ctx, |ui| {
                let width = ui.available_width();
                ui.allocate_ui_with_layout(
                    egui::Vec2::new(width, 30.),
                    egui::Layout::right_to_left(),
                    |ui| {
                        ui.style_mut().override_font_id = Some(egui::FontId::monospace(12.));
                        ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
                        let label = egui::Label::new("\u{f013}").sense(egui::Sense::click());
                        if ui.add(label).clicked() {
                            self.settings_open = !self.settings_open;
                        }
                        let log_label = egui::Label::new("Log").wrap(false);
                        let space = ui.available_width() - 40.0;
                        ui.add_space(space);
                        ui.label("Log");
                        ui.reset_style();
                    },
                );
                let text_style = egui::TextStyle::Body;
                let row_height = ui.text_style_height(&text_style) * 2.;
                egui::ScrollArea::vertical()
                    .stick_to_bottom()
                    .hscroll(false)
                    .show_rows(ui, row_height, self.error_log.len(), |ui, range| {
                        for i in range {
                            ui.add(egui::Label::new(&self.error_log[i]));
                        }
                    });
            });
    }

    pub(crate) fn render_exit_prompt(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::Window::new("Save before exiting?")
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Yes").clicked() {
                        self.save_data();
                        self.exit_state = ExitState::Exiting;
                        frame.quit();
                    }

                    if ui.button("No").clicked() {
                        self.exit_state = ExitState::Exiting;
                        frame.quit();
                    }

                    if ui.button("Cancel").clicked() {
                        self.exit_state = ExitState::Running;
                    }
                });
            });
    }
}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        crate::profile_guard!("update", "gui::GuiApp");

        if let Some(rx) = &self.back_rx {
            match rx.try_recv() {
                Ok(msg) => match msg {
                    ToFrontend::RefreshNoteList { notes } => {
                        self.notes = notes;
                    }
                    ToFrontend::Error { error_msg } => self.error_log.push(error_msg),
                    ToFrontend::NoteCreated { note } => {
                        self.active_note = Some(note);
                        self.active_tag = None;
                    }
                    ToFrontend::DatabaseLoaded { notes } => {
                        self.state = AppState::DatabaseOpen;
                        self.notes = notes;
                        self.active_note = None;
                        self.active_tag = None;
                    }
                    ToFrontend::DatabaseClosed => {
                        self.state = AppState::NoDatabase;
                        self.notes = Vec::new();
                        self.active_note = None;
                        self.active_tag = None;
                    }
                },
                Err(err) => {
                    let _ = err;
                }
            }
        }

        match self.exit_state {
            ExitState::Running => {}
            ExitState::ExitRequested => {
                self.render_exit_prompt(ctx, frame);
            }
            ExitState::Exiting => {
                return;
            }
        }

        self.render_error_log(ctx, frame);

        if self.settings_open {
            self.render_settings(ctx, frame);
            return;
        }

        match self.state {
            AppState::NoDatabase => self.render_no_db(ctx, frame),
            AppState::DatabaseOpen => self.render_db_loaded(ctx, frame),
        }
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        crate::profile_guard!("save", "gui::GuiApp");
        self.save_data();
    }

    fn on_exit(&mut self, _gl: &eframe::glow::Context) {
        self.save_data();
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.settings.autosave_interval)
    }

    fn on_exit_event(&mut self) -> bool {
        if let Some(tx) = &mut self.front_tx {
            tx.send(ToBackend::Shutdown)
                .expect("Unable to send message to backend.");
        }

        if self.exit_state == ExitState::Running {
            self.exit_state = ExitState::ExitRequested;
        }

        self.exit_state == ExitState::Exiting
    }

    fn max_size_points(&self) -> egui::Vec2 {
        egui::Vec2::INFINITY
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).into()

        // _visuals.window_fill() would also be a natural choice
    }

    fn persist_native_window(&self) -> bool {
        true
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn warm_up_enabled(&self) -> bool {
        false
    }
}
