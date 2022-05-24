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
    widgets::NoteEditor,
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
    state: AppState,
    settings: AppSettings,
    settings_open: bool,
    front_tx: Sender<ToBackend>,
    back_rx: Receiver<ToFrontend>,
    error_log: Vec<String>,
    exit_state: ExitState,
    note_editor: NoteEditor,
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
            front_tx,
            back_rx,
            state: AppState::NoDatabase,
            settings,
            settings_open: false,
            error_log: Vec::new(),
            exit_state: ExitState::Running,
            note_editor: NoteEditor::default(),
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

    fn new_db(&self, path: PathBuf) {
        self.front_tx
            .send(ToBackend::CreateDatabase { path })
            .expect("Unable to send create database message to backend");
    }

    fn load_db(&mut self, path: PathBuf) {
        self.front_tx
            .send(ToBackend::OpenDatabase { path })
            .expect("Unable to send open database message to backend");
    }

    fn autosave(&self) {
        if self.settings.autosave_enabled {
            self.save_data();
        }
    }

    fn save_data(&self) {
        self.front_tx
            .send(ToBackend::SaveData)
            .expect("Unable to send timed save message to backend");
    }

    fn new_note(&mut self) {
        self.front_tx
            .send(ToBackend::CreateNote {
                dto: (String::from("New Note"), String::new()).into(),
            })
            .expect("Unable to send new note message to backend");
    }

    fn delete_note_v2(&mut self, id: TinyId) {
        self.note_editor.clear_if_active_id(id);
        self.front_tx
            .send(ToBackend::DeleteNote { id })
            .expect("Unable to send delete note message to backend");
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
                        self.note_editor.set_active_note(note);
                    }
                }
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            crate::profile_guard!("CentralPanel", "gui::GuiApp::update");
            self.note_editor.render(ui);
        });

        if self.note_editor.has_changes() {
            if let Some(note) = self.note_editor.get_active_note() {
                self.front_tx
                    .send(ToBackend::UpdateNote { note: note.clone() })
                    .unwrap();
            }
            self.note_editor.clear_has_changes();
        }

        if let Some(id) = delete_requested {
            self.delete_note_v2(id);
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

fn tester_id() -> &'static egui::Id {
    use once_cell::sync::OnceCell;
    static ID: OnceCell<egui::Id> = OnceCell::new();
    ID.get_or_init(|| egui::Id::new("tester").with(1234))
}
static mut FIRST: bool = true;

impl eframe::App for GuiApp {
    #[allow(clippy::cast_possible_truncation)]
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        crate::profile_guard!("update", "gui::GuiApp");

        let time = ctx.input().time;
        let truncated = (time * 1000.0).trunc() / 1000.0;
        let available = ctx.available_rect();
        let animated = ctx.animate_value_with_time(*tester_id(), (time / 2.) as f32, 5.);
        ctx.debug_painter().text(
            egui::pos2(400., 50.),
            egui::Align2::CENTER_CENTER,
            format!(
                "Time: {}\nAvailable: {:?}\nAnimated: {}",
                truncated, available, animated
            ),
            egui::FontId::monospace(14.0),
            egui::Color32::BLUE,
        );

        match self.back_rx.try_recv() {
            Ok(msg) => match msg {
                ToFrontend::RefreshNoteList { notes } => {
                    self.notes = notes;
                }
                ToFrontend::Error { error_msg } => self.error_log.push(error_msg),
                ToFrontend::NoteCreated { note } => {
                    self.note_editor.set_active_note(&note);
                }
                ToFrontend::DatabaseLoaded { notes } => {
                    self.state = AppState::DatabaseOpen;
                    self.notes = notes;
                }
                ToFrontend::DatabaseClosed => {
                    self.state = AppState::NoDatabase;
                    self.notes = Vec::new();
                    self.note_editor.clear_active_note();
                }
            },
            Err(err) => {
                let _ = err;
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
        self.autosave();
    }

    fn on_exit(&mut self, _gl: &eframe::glow::Context) {
        // self.save_data();
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        if self.settings.autosave_enabled {
            std::time::Duration::from_secs(self.settings.autosave_interval)
        } else {
            std::time::Duration::MAX
        }
    }

    fn on_exit_event(&mut self) -> bool {
        self.front_tx
            .send(ToBackend::Shutdown)
            .expect("Unable to send message to backend.");

        if self.exit_state == ExitState::Running {
            self.exit_state = ExitState::ExitRequested;
        }

        self.exit_state == ExitState::Exiting
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).into()

        // _visuals.window_fill() would also be a natural choice
    }
}
