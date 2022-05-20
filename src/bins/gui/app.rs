// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::types::Note;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum AppState {
    Main,
    EditingNote,
}

pub struct GuiApp {
    db: crate::db::Database,
    notes: Vec<Note>,
    active_note: Option<Note>,
    active_tag: Option<usize>,
    state: AppState,
}

impl GuiApp {
    pub fn new(cc: &eframe::CreationContext<'_>, db: crate::db::Database) -> Self {
        Self::setup_custom_fonts(&cc.egui_ctx);
        let notes = db.get_all().to_vec();
        Self {
            db,
            notes,
            active_note: None,
            active_tag: None,
            state: AppState::Main,
        }
    }

    pub fn update_notes(&mut self) {
        if let Some(ref mut note) = self.active_note {
            self.db.ensure_sync_v2(note);
        }
        self.notes = self.db.get_all().to_vec();
    }

    pub fn save_data(&mut self) {
        self.update_notes();
        let _res = self.db.save_dev();
    }

    pub fn set_active_note(&mut self, note: Note) {
        if let Some(ref current) = self.active_note {
            if current.id() == note.id() {
                return;
            }
        }
        self.active_note = Some(note);
        self.active_tag = None;
    }

    pub fn set_editing_tag(&mut self, index: usize) {
        self.active_tag = Some(index);
    }

    pub fn set_adding_tag(&mut self) {
        if let Some(ref mut note) = self.active_note {
            note.add_tag("New Tag".to_string());
            self.active_tag = Some(note.tag_len() - 1);
        }
    }

    pub fn clear_editing_tag(&mut self) {
        self.active_tag = None;
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
}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        crate::profile_guard!("update", "gui::GuiApp");
        let mut note_changed = false;
        egui::SidePanel::left("note_list_panel").show(ctx, |ui| {
            crate::profile_guard!("SidePanel", "gui::GuiApp::update");
            ui.heading(egui::RichText::new("Notes").heading());
            ui.separator();
            egui::ScrollArea::vertical().show(ui, |ui| {
                for note in &self.notes {
                    if ui.button(note.title()).clicked() {
                        self.active_note = Some(note.clone());
                        self.state = AppState::EditingNote;
                    }
                }
            });
        });
        let mut active = self.active_note.clone();
        egui::CentralPanel::default().show(ctx, |ui| {
            crate::profile_guard!("CentralPanel", "gui::GuiApp::update");
            match active {
                Some(ref mut note) => {
                    if render_title(ui, note) {
                        note_changed = true;
                    }
                    if render_content(ui, note) {
                        note_changed = true;
                    }
                    if render_tags(ui, note, self) {
                        note_changed = true;
                    }
                }
                None => {
                    ui.centered_and_justified(|ui| ui.label("Select a note to start editing!"));
                }
            }
        });

        if note_changed {
            self.active_note = active;
            self.update_notes();
        }
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        crate::profile_guard!("save", "gui::GuiApp");
        self.update_notes();
        let _res = self.db.save_dev();
    }

    fn on_exit(&mut self, _gl: &eframe::glow::Context) {
        self.save_data();
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }
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
fn render_tags(ui: &mut egui::Ui, note: &mut Note, app: &mut GuiApp) -> bool {
    let mut note_tags = note.tags().to_vec();
    let mut removals = Vec::new();
    let mut tags_changed = false;
    ui.label("Tags:");

    match app.active_tag {
        None => {
            for (i, tag) in note_tags.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    let tag_response = ui.add(egui::Label::new(tag.as_str()).sense(egui::Sense::click()));
                    if tag_response.double_clicked() {
                        app.set_editing_tag(i);
                    }
                    if ui.small_button("x").clicked() {
                        removals.push(i);
                    }
                });
            }

            if ui.small_button("+").clicked() {
                app.set_adding_tag();
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
                            app.clear_editing_tag();
                        }
                    } else {
                        if ui.button(tag.as_str()).clicked() {
                            app.set_editing_tag(i);
                        }
                    }
                    if ui.small_button("x").clicked() {
                        removals.push(i);
                    }
                });
            }

            if ui.small_button("+").clicked() {
                app.set_adding_tag();
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
