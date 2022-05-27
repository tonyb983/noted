// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::path::{Path, PathBuf};

use crossbeam_channel::{Receiver, Sender};
use eframe::egui;
use egui_toast::{Toast, ToastKind, ToastOptions, Toasts};
use tinyid::TinyId;

use crate::types::Note;

use super::{
    backend::{Backend, ToBackend, ToFrontend},
    hotkey::{HotkeyEditor, HotkeyState, Hotkeys},
    settings::{AppSettings, AppSettingsUi},
    widgets::{NoteEditor, SimplePrompt, ToApp},
};

fn default_toast_options() -> ToastOptions {
    ToastOptions {
        show_icon: true,
        expires_at: Some(std::time::Instant::now() + std::time::Duration::from_secs(5)),
    }
}

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum DeletingState {
    None,
    Prompting(TinyId),
    Confirmed(TinyId),
}

impl DeletingState {
    pub const DELETE_PROMPT_NAME: &'static str = "confirm_delete";

    pub fn confirm(&mut self) {
        if let DeletingState::Prompting(id) = self {
            *self = DeletingState::Confirmed(*id);
        }
    }

    pub fn cancel(&mut self) {
        if let DeletingState::Prompting(id) = self {
            *self = DeletingState::None;
        }
    }

    pub fn prompting(&mut self, id: TinyId) {
        *self = DeletingState::Prompting(id);
    }
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
    widget_rx: Receiver<ToApp>,
    toast_tx: Sender<Toast>,
    toast_rx: Receiver<Toast>,
    deleting_state: DeletingState,
    time: f64,
    hotkeys: Hotkeys,
}

impl GuiApp {
    #[allow(clippy::redundant_clone)]
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let settings = AppSettings::load_or_create().expect("Unable to load/create app settings");
        Self::setup_custom_fonts(&cc.egui_ctx);
        let notes = Vec::new();

        let (front_tx, front_rx) = crossbeam_channel::unbounded();
        let (back_tx, back_rx) = crossbeam_channel::unbounded();
        let (widget_tx, widget_rx) = crossbeam_channel::unbounded();
        let (toast_tx, toast_rx) = crossbeam_channel::unbounded();

        let note_editor = NoteEditor::new(widget_tx.clone(), None, true, &settings);

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

        let error_log = vec![
            "This is a test error log entry".to_string(),
            "This is another fake error message".to_string(),
            "And here's a third, lets make this one really long. Like super super super super super supser upser upserup super super supser upser upser super duper long.".to_string(),
        ];

        Self {
            notes,
            front_tx,
            back_rx,
            state: AppState::NoDatabase,
            settings,
            settings_open: false,
            error_log,
            exit_state: ExitState::Running,
            note_editor,
            widget_rx,
            deleting_state: DeletingState::None,
            time: cc.egui_ctx.input().time,
            toast_rx,
            toast_tx,
            hotkeys: Hotkeys::default(),
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

    fn autosave(&mut self) {
        if self.settings.autosave_enabled {
            self.save_data();
        }
    }

    fn save_data(&mut self) {
        self.update_active_note();
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

    fn change_active_note(&mut self, note: Option<Note>) {
        self.update_active_note();
        self.note_editor.set_note(note);
    }

    fn update_active_note(&mut self) {
        if self.note_editor.has_active_note() && self.note_editor.has_changes() {
            if let Some(note) = self.note_editor.get_active_note() {
                self.front_tx
                    .send(ToBackend::UpdateNote { note: note.clone() })
                    .unwrap();
            }
            self.note_editor.clear_has_changes();
        }
    }

    fn delete_note_v2(&mut self, id: TinyId) {
        self.note_editor.clear_if_active_id(id);
        self.front_tx
            .send(ToBackend::DeleteNote { id })
            .expect("Unable to send delete note message to backend");
    }

    fn check_hotkeys(&mut self, ctx: &egui::Context) {
        let state = self.hotkeys.check_hotkeys(ctx);
        if state.new_note {
            self.new_note();
        }
        if state.close_note_editor {
            self.change_active_note(None);
        }
        if state.delete {
            if let Some(note) = self.note_editor.get_active_note() {
                self.deleting_state = DeletingState::Prompting(note.id());
            }
        }
        if state.quit {
            self.exit_state = ExitState::ExitRequested;
        }
        if state.toggle_settings {
            self.settings_open = !self.settings_open;
        }
        if state.save {
            self.save_data();
        }
        if state.copy {
            // TODO: Implement copying for text AND notes
            self.error_log.push("COPY hotkey pressed.".to_string());
        }
        if state.cut {
            // TODO: Implement cutting for text AND notes
            self.error_log.push("CUT hotkey pressed.".to_string());
        }
        if state.paste {
            // TODO: Implement pasting for text AND notes
            self.error_log.push("PASTE hotkey pressed.".to_string());
        }
        if state.undo {
            // TODO: Implement undo/redo
            self.error_log.push("UNDO hotkey pressed.".to_string());
        }
        if state.redo {
            // TODO: Implement undo/redo
            self.error_log.push("REDO hotkey pressed.".to_string());
        }
    }

    fn render_db_loaded(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // let mut delete_requested = None;
        let mut change_active = None;

        let mut side_panel = egui::SidePanel::left("note_list_panel")
            .width_range(50.0..=200.0)
            .default_width(100.0);
        side_panel.show(ctx, |ui| {
            crate::profile_guard!("SidePanel", "gui::GuiApp::update");
            let side_width = ui.available_width();
            ui.allocate_ui_with_layout(
                egui::Vec2::new(side_width, 30.),
                egui::Layout::right_to_left(),
                |ui| {
                    if ui.button("+").clicked() {
                        self.new_note();
                    }
                    ui.allocate_ui_with_layout(
                        ui.available_size(),
                        egui::Layout::centered_and_justified(egui::Direction::TopDown),
                        |ui| {
                            ui.heading(egui::RichText::new("Notes").heading());
                        },
                    );
                },
            );
            ui.separator();
            egui::ScrollArea::vertical().show(ui, |ui| {
                let max_width = ui.available_width();
                egui::Grid::new("note_selection_grid")
                    .num_columns(1)
                    .max_col_width(max_width)
                    .min_col_width(max_width)
                    .min_row_height(15.)
                    .show(ui, |ui| {
                        for (i, note) in self.notes.iter().enumerate() {
                            let button = egui::Button::new(note.title()).wrap(true);
                            ui.allocate_ui_with_layout(
                                [max_width, 75.].into(),
                                egui::Layout::top_down_justified(egui::Align::Center),
                                |ui| {
                                    let res = ui.add(button).context_menu(|ui| {
                                        if ui.small_button("Delete this note.").clicked() {
                                            self.deleting_state =
                                                DeletingState::Prompting(note.id());
                                            ui.close_menu();
                                        }
                                    });
                                    if res.clicked() {
                                        change_active = Some(note.clone());
                                    }
                                },
                            );
                            ui.end_row();
                        }
                    });
            });
        });

        // Currently I have to do this because I can't call this function while iterating through the list of GuiApp::notes
        if let Some(note) = change_active {
            self.change_active_note(Some(note));
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            crate::profile_guard!("CentralPanel", "gui::GuiApp::update");
            self.note_editor.render(ui);
        });

        // self.update_active_note();

        match self.deleting_state {
            DeletingState::None => {}
            DeletingState::Prompting(id) => {
                SimplePrompt::show(ctx, "Delete Note?", |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Yes").clicked() {
                            self.deleting_state.confirm();
                        } else if ui.button("No").clicked() {
                            self.deleting_state.cancel();
                        }
                    });
                });
            }
            DeletingState::Confirmed(id) => {
                self.delete_note_v2(id);
                self.deleting_state = DeletingState::None;
            }
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
                self.note_editor.settings_updated(&self.settings);
            }

            //ui.separator();
            HotkeyEditor::render(ui, &mut self.hotkeys);
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
                        ui.add_space(10.);
                        ui.style_mut().override_font_id = Some(egui::FontId::monospace(12.));
                        ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
                        let label = egui::Label::new("\u{f013}").sense(egui::Sense::click());
                        if ui.add(label).clicked() {
                            self.settings_open = !self.settings_open;
                        }
                        let log_label = egui::Label::new("Log").wrap(false);
                        let space = ui.available_width() - 50.0;
                        ui.add_space(space);
                        ui.label("Log");
                        ui.reset_style();
                    },
                );
                ui.centered_and_justified(|ui| {
                    egui::ScrollArea::vertical()
                        .stick_to_bottom()
                        .hscroll(false)
                        .show(ui, |ui| {
                            let width = ui.available_width();
                            egui::Grid::new("error_log_grid")
                                .num_columns(1)
                                .striped(true)
                                .max_col_width(width)
                                .show(ui, |ui| {
                                    for error in &self.error_log {
                                        ui.label(error);
                                        ui.end_row();
                                    }
                                });
                        });
                    // .show_rows(ui, row_height, self.error_log.len(), |ui, range| {
                    //     for i in range {
                    //         ui.add(egui::Label::new(&self.error_log[i]));
                    //     }
                    // });
                })
            });
    }

    fn render_exit_prompt(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.deleting_state = DeletingState::None;
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

    fn render_toasts(&mut self, ctx: &egui::Context) {
        let mut anchor = ctx.input().screen_rect().shrink(5.0).max;
        anchor.x *= 0.9;
        let mut toasts = Toasts::new(ctx)
            .direction(egui::Direction::BottomUp)
            .anchor(anchor)
            .align_to_end(true);

        if !self.toast_rx.is_empty() {
            match self.toast_rx.try_recv() {
                Ok(toast) => {
                    toasts.add(toast.text, toast.kind, toast.options);
                }
                Err(err) => {
                    let _ = err;
                }
            }
        }

        toasts.show();
    }
}

impl eframe::App for GuiApp {
    #[allow(clippy::cast_possible_truncation)]
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        crate::profile_guard!("update", "gui::GuiApp");

        self.time = ctx.input().time;
        // let time = ctx.input().time;
        // let truncated = (time * 1000.0).trunc() / 1000.0;
        // let available = ctx.available_rect();
        // let animated = ctx.animate_value_with_time(*tester_id(), (time / 2.) as f32, 5.);
        // ctx.debug_painter().text(
        //     egui::pos2(400., 50.),
        //     egui::Align2::CENTER_CENTER,
        //     format!(
        //         "Time: {}\nAvailable: {:?}\nAnimated: {}",
        //         truncated, available, animated
        //     ),
        //     egui::FontId::monospace(14.0),
        //     egui::Color32::BLUE,
        // );

        match self.back_rx.try_recv() {
            Ok(msg) => match msg {
                ToFrontend::RefreshNoteList { notes } => {
                    self.notes = notes;
                    self.toast_tx
                        .send(Toast {
                            kind: ToastKind::Info,
                            text: "Note list updated".into(),
                            options: default_toast_options(),
                        })
                        .expect("Unable to send toast");
                }
                ToFrontend::Error { error_msg } => {
                    self.toast_tx
                        .send(Toast {
                            kind: ToastKind::Error,
                            text: format!("Backend Error: {}", &error_msg).into(),
                            options: default_toast_options(),
                        })
                        .expect("Unable to send toast");
                    self.error_log.push(error_msg);
                }
                ToFrontend::NoteCreated { note } => {
                    self.change_active_note(Some(note));
                    self.toast_tx
                        .send(Toast {
                            kind: ToastKind::Info,
                            text: "New note created".into(),
                            options: default_toast_options(),
                        })
                        .expect("Unable to send toast");
                }
                ToFrontend::DatabaseLoaded { notes } => {
                    self.state = AppState::DatabaseOpen;
                    self.notes = notes;
                }
                ToFrontend::DatabaseClosed => {
                    self.state = AppState::NoDatabase;
                    self.notes = Vec::new();
                    self.note_editor.clear_note();
                }
            },
            Err(err) => {
                let _ = err;
            }
        }

        if !self.widget_rx.is_empty() {
            match self.widget_rx.try_recv() {
                Ok(msg) => match msg {
                    ToApp::CreateNewNote => {
                        self.new_note();
                    }
                },
                Err(err) => {
                    self.error_log
                        .push(format!("Widget channel error: {}", err));
                }
            }
        }

        self.check_hotkeys(ctx);

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

        self.render_toasts(ctx);

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
