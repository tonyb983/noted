// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crossbeam_channel::Sender;
use eframe::egui::{self, TextEdit};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use tinyid::TinyId;

use crate::types::Note;

use super::{super::settings::AppSettings, ToApp, WidgetState};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum PreviewState {
    Open,
    Closed,
}

pub struct NoteEditor {
    active: WidgetState,
    active_note: Option<Note>,
    active_tag: Option<usize>,
    md_cache: CommonMarkCache,
    has_changes: bool,
    preview_state: PreviewState,
    app_sender: Sender<ToApp>,
    humanize_dates: bool,
}

impl NoteEditor {
    pub fn new(
        app_sender: Sender<ToApp>,
        note: Option<Note>,
        active: bool,
        settings: &AppSettings,
    ) -> Self {
        Self {
            app_sender,
            active: active.into(),
            active_note: note,
            active_tag: None,
            has_changes: false,
            preview_state: PreviewState::Closed,
            md_cache: CommonMarkCache::default(),
            humanize_dates: true,
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui) {
        if self.active.is_disabled() {
            return;
        }

        let mut changes = false;
        let mut active_note = match &self.active_note {
            Some(note) => note.clone(),
            None => {
                let max_rect = ui.max_rect();
                let shrink_x = max_rect.max.x / 4.;
                let shrink_y = max_rect.max.y / 4.;
                ui.allocate_ui_at_rect(max_rect.shrink2(egui::vec2(shrink_x, shrink_y)), |ui| {
                    ui.vertical_centered(|ui| {
                        if ui.button("Create Note").clicked() {
                            self.app_sender.send(ToApp::CreateNewNote).unwrap();
                        }
                        ui.label("Or select an existing note to start editing.");
                    });
                });
                return;
            }
        };

        changes |= self.render_title_editor(ui, &mut active_note);
        ui.add(egui::Separator::default().horizontal().spacing(30.));
        changes |= self.render_content_editor(ui, &mut active_note);
        ui.add(egui::Separator::default().horizontal().spacing(30.));
        changes |= self.render_tags_editor(ui, &mut active_note);
        ui.add(egui::Separator::default().horizontal().spacing(30.));
        self.render_metadata(ui, &mut active_note);

        if changes {
            self.active_note = Some(active_note);
            self.has_changes = true;
        }
    }

    pub fn get_active_note(&self) -> Option<&Note> {
        self.active_note.as_ref()
    }

    pub fn has_active_note(&self) -> bool {
        self.active_note.is_some()
    }

    pub fn clear_note(&mut self) {
        self.active_tag = None;
        self.active_note = None;
        self.has_changes = false;
        self.preview_state = PreviewState::Closed;
    }

    pub fn clear_if_active(&mut self, note: &Note) {
        self.clear_if_active_id(note.id());
    }

    pub fn clear_if_active_id(&mut self, id: TinyId) {
        if let Some(ref current) = self.active_note {
            if current.id() == id {
                self.clear_note();
            }
        }
    }

    pub fn set_active(&mut self, state: bool) {
        self.active = state.into();
    }

    pub fn set_note(&mut self, note: Option<Note>) {
        if let (Some(n), Some(curr)) = (&note, &self.active_note) {
            if n.id() == curr.id() {
                return;
            }
        }

        self.active_note = note;
        self.active_tag = None;
        self.has_changes = false;
        self.preview_state = PreviewState::Closed;
    }

    pub fn is_active(&self) -> bool {
        self.active.is_enabled()
    }

    pub fn has_changes(&self) -> bool {
        self.has_changes
    }

    pub fn clear_has_changes(&mut self) {
        self.has_changes = false;
    }

    pub fn settings_updated(&mut self, settings: &AppSettings) {
        self.humanize_dates = settings.humanize_dates;
    }
}

impl NoteEditor {
    #[allow(clippy::unused_self)]
    fn render_title_editor(&self, ui: &mut egui::Ui, note: &mut Note) -> bool {
        let mut note_title = note.title().to_string();
        let title_response =
            ui.add(TextEdit::singleline(&mut note_title).font(egui::TextStyle::Heading));
        // let title_response = ui.text_edit_singleline(&mut note_title);
        if title_response.changed() {
            note.set_title(note_title.as_str());
            return true;
        }

        false
    }

    #[allow(clippy::unused_self)]
    fn render_content_editor(&mut self, ui: &mut egui::Ui, note: &mut Note) -> bool {
        let mut note_content = note.content().to_string();
        let mut preview_open = self.preview_state == PreviewState::Open;

        ui.horizontal_top(|ui| {
            if ui
                .toggle_value(
                    &mut preview_open,
                    match self.preview_state {
                        PreviewState::Open => "Hide Markdown Preview",
                        PreviewState::Closed => "Show Markdown Preview",
                    },
                )
                .clicked()
            {
                self.preview_state = if preview_open {
                    PreviewState::Open
                } else {
                    PreviewState::Closed
                };
            }
        });

        match self.preview_state {
            PreviewState::Open => {
                CommonMarkViewer::new("note_content_viewer").show(
                    ui,
                    &mut self.md_cache,
                    note_content.as_str(),
                );
            }
            PreviewState::Closed => {
                let content_response = ui.code_editor(&mut note_content);
                if content_response.changed() {
                    note.set_content(note_content.as_str());
                    return true;
                }
            }
        }

        false
    }

    #[allow(clippy::collapsible_if)]
    #[allow(clippy::collapsible_else_if)]
    fn render_tags_editor(&mut self, ui: &mut egui::Ui, note: &mut Note) -> bool {
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
                            self.active_tag = Some(i);
                        }
                        if ui.small_button("x").clicked() {
                            removals.push(i);
                        }
                    });
                }

                if ui.small_button("+").clicked() {
                    // note.add_tag("New Tag".to_string());
                    note_tags.push("New Tag".to_string());
                    self.active_tag = Some(note_tags.len() - 1);
                    tags_changed = true;
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
                                self.active_tag = None;
                            }
                        } else {
                            if ui.button(tag.as_str()).clicked() {
                                self.active_tag = Some(i);
                            }
                        }
                        if ui.small_button("x").clicked() {
                            removals.push(i);
                        }
                    });
                }

                if ui.small_button("+").clicked() {
                    // note.add_tag("New Tag".to_string());
                    note_tags.push("New Tag".to_string());
                    // self.active_tag = Some(note.tag_len() - 1);
                    self.active_tag = Some(note_tags.len() - 1);
                    tags_changed = true;
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

    #[allow(clippy::unused_self)]
    fn render_metadata(&mut self, ui: &mut egui::Ui, note: &mut Note) {
        ui.horizontal(|ui| {
            let height = ui.text_style_height(&egui::TextStyle::Body);
            ui.set_height(height);
            ui.horizontal_centered(|ui| {
                if self.humanize_dates {
                    ui.label(format!("Created: {}", note.created_humanized()));
                    ui.label("|");
                    ui.label(format!("Updated: {}", note.updated_humanized()));
                } else {
                    ui.label(format!(
                        "Created: {}",
                        crate::util::dtf::timestamp_to_string(note.created())
                    ));
                    ui.label("|");
                    ui.label(format!(
                        "Updated: {}",
                        crate::util::dtf::timestamp_to_string(note.updated())
                    ));
                }
            });
        });
    }
}
