// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crossbeam_channel::Sender;
use eframe::{
    egui::{Button, Grid, Layout, ScrollArea, Ui},
    emath::Align,
};
use egui_toast::Toast;

use crate::types::Note;

use super::{super::settings::AppSettings, ToApp};

pub struct NoteList {
    notes: Vec<Note>,
    app_sender: Sender<ToApp>,
    toast_sender: Sender<Toast>,
}

impl NoteList {
    pub fn new(
        app_sender: Sender<ToApp>,
        toast_sender: Sender<Toast>,
        settings: &AppSettings,
    ) -> Self {
        Self {
            notes: Vec::new(),
            app_sender,
            toast_sender,
        }
    }

    pub fn with_notes(
        notes: Vec<Note>,
        app_sender: Sender<ToApp>,
        toast_sender: Sender<Toast>,
        settings: &AppSettings,
    ) -> Self {
        Self {
            notes,
            app_sender,
            toast_sender,
        }
    }

    pub fn render(&mut self, ui: &mut Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            let max_width = ui.available_width();
            Grid::new("note_selection_grid")
                .num_columns(1)
                .max_col_width(max_width)
                .min_col_width(max_width)
                .min_row_height(15.)
                .show(ui, |ui| {
                    for (i, note) in self.notes.iter().enumerate() {
                        let button = Button::new(note.title()).wrap(true);
                        ui.allocate_ui_with_layout(
                            [max_width, 75.].into(),
                            Layout::top_down_justified(Align::Center),
                            |ui| {
                                let res = ui.add(button).context_menu(|ui| {
                                    if ui.small_button("Delete this note.").clicked() {
                                        Self::send_app_msg(
                                            &self.app_sender,
                                            ToApp::DeleteNote(note.clone()),
                                        );
                                        ui.close_menu();
                                    }
                                });
                                if res.clicked() {
                                    Self::send_app_msg(
                                        &self.app_sender,
                                        ToApp::SetActiveNote(note.clone()),
                                    );
                                }
                            },
                        );
                        ui.end_row();
                    }
                });
        });
    }

    pub fn update_note_list(&mut self, notes: Vec<Note>) {
        self.notes = notes;
    }

    pub fn clear_note_list(&mut self) {
        self.update_note_list(Vec::new());
    }

    fn send_app_msg(sender: &Sender<ToApp>, msg: ToApp) {
        sender.send(msg).expect("Unable to send message to GuiApp");
    }
}
