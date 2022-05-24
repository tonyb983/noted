// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use tinyid::TinyId;

use crate::types::Note;

#[derive(Debug, Clone)]
pub struct NoteEditor {
    active: bool,
    active_note: Option<Note>,
    active_tag: Option<usize>,
    has_changes: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for NoteEditor {
    fn default() -> Self {
        Self {
            active: true,
            active_note: None,
            active_tag: None,
            has_changes: false,
        }
    }
}

impl NoteEditor {
    pub fn render(&mut self, ui: &mut egui::Ui) {
        if !self.active {
            return;
        }

        let mut changes = false;
        let mut active_note = match &self.active_note {
            Some(note) => note.clone(),
            None => {
                ui.centered_and_justified(|ui| {
                    if ui.button("Select a note to start editing!").clicked() {}
                });
                ui.centered_and_justified(|ui| ui.label("Select a note to start editing!"));
                return;
            }
        };

        changes |= self.render_title_editor(ui, &mut active_note);
        changes |= self.render_content_editor(ui, &mut active_note);
        changes |= self.render_tags_editor(ui, &mut active_note);

        if changes {
            self.active_note = Some(active_note);
            self.has_changes = true;
        }
    }

    pub fn get_active_note(&self) -> Option<&Note> {
        self.active_note.as_ref()
    }

    pub fn set_active_note(&mut self, note: &Note) {
        if let Some(ref current) = self.active_note {
            if current.id() == note.id() {
                return;
            }
        }

        self.active_note = Some(note.clone());
        self.active_tag = None;
        self.has_changes = false;
    }

    pub fn clear_active_note(&mut self) {
        self.active_tag = None;
        self.active_note = None;
        self.has_changes = false;
    }

    pub fn clear_if_active(&mut self, note: &Note) {
        self.clear_if_active_id(note.id());
    }

    pub fn clear_if_active_id(&mut self, id: TinyId) {
        if let Some(ref current) = self.active_note {
            if current.id() == id {
                self.clear_active_note();
            }
        }
    }

    pub fn set_active(&mut self, state: bool) {
        self.active = state;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn has_changes(&self) -> bool {
        self.has_changes
    }

    pub fn clear_has_changes(&mut self) {
        self.has_changes = false;
    }
}

impl NoteEditor {
    #[allow(clippy::unused_self)]
    fn render_title_editor(&self, ui: &mut egui::Ui, note: &mut Note) -> bool {
        let mut note_title = note.title().to_string();
        let title_response = ui.text_edit_singleline(&mut note_title);
        if title_response.changed() {
            note.set_title(note_title.as_str());
            return true;
        }

        false
    }

    #[allow(clippy::unused_self)]
    fn render_content_editor(&self, ui: &mut egui::Ui, note: &mut Note) -> bool {
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
}
