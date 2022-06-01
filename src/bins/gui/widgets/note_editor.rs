// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::sync::Once;

use chrono::Datelike;
use crossbeam_channel::Sender;
use eframe::egui::{self, TextEdit, Ui};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use egui_extras::DatePickerButton;
use egui_toast::{Toast, ToastKind};
use time::OffsetDateTime;
use tinyid::TinyId;

use crate::{
    bins::gui::app,
    types::{
        time::{Hour, Hour12, Minute, TimePeriod},
        Note, Reminder,
    },
};

use super::{super::settings::AppSettings, ToApp, WidgetState};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum PreviewState {
    Open,
    Closed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ReminderFocus {
    Date,
    Time,
    Text,
}

pub struct NoteEditor {
    active: WidgetState,
    active_note: Option<Note>,
    active_tag: Option<usize>,
    active_reminder: Option<(usize, ReminderFocus)>,
    md_cache: CommonMarkCache,
    has_changes: bool,
    preview_state: PreviewState,
    app_sender: Sender<ToApp>,
    toast_sender: Sender<Toast>,
    humanize_dates: bool,
    force_save: bool,
}

impl NoteEditor {
    pub fn new(
        app_sender: Sender<ToApp>,
        toast_sender: Sender<Toast>,
        note: Option<Note>,
        active: bool,
        settings: &AppSettings,
    ) -> Self {
        Self {
            app_sender,
            active: active.into(),
            active_note: note,
            active_tag: None,
            active_reminder: None,
            has_changes: false,
            preview_state: PreviewState::Closed,
            md_cache: CommonMarkCache::default(),
            humanize_dates: true,
            toast_sender,
            force_save: false,
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
                            Self::send_app_msg(&self.app_sender, ToApp::CreateNewNote);
                        }
                        ui.label("Or select an existing note to start editing.");
                    });
                });
                return;
            }
        };

        changes = changes || self.render_title_editor(ui, &mut active_note);
        ui.add(egui::Separator::default().horizontal().spacing(25.));
        changes = changes || self.render_content_editor(ui, &mut active_note);
        ui.add(egui::Separator::default().horizontal().spacing(25.));
        changes = changes || self.render_tags_editor(ui, &mut active_note);
        ui.add(egui::Separator::default().horizontal().spacing(25.));
        changes = changes || self.render_reminders(ui, &mut active_note);
        ui.add(egui::Separator::default().horizontal().spacing(25.));
        self.render_metadata(ui, &mut active_note);

        if changes {
            self.active_note = Some(active_note);
            self.has_changes = true;
        }

        if self.force_save {
            self.force_save = false;
            Self::send_app_msg(&self.app_sender, ToApp::SaveRequested);
            Self::send_app_msg(
                &self.app_sender,
                ToApp::Toast(ToastKind::Info, "Force save requested...".to_string()),
            );
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
        self.active_reminder = None;
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
        self.active_reminder = None;
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
    fn local_offset() -> time::UtcOffset {
        static LOCAL_OFFSET: once_cell::sync::OnceCell<time::UtcOffset> =
            once_cell::sync::OnceCell::new();
        *LOCAL_OFFSET.get_or_init(|| time::UtcOffset::current_local_offset().unwrap())
    }

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

    /// Renders the list of reminders for the note editor. If the reminders are changed the active Note will be updated
    /// and `true` will be returned. This will also set `self.active_reminder` based on user interactions.
    fn render_reminders(&mut self, ui: &mut egui::Ui, note: &mut Note) -> bool {
        type ChronoUtcDate = chrono::Date<chrono::Utc>;
        type TimeDate = time::Date;
        use time::{Date, Time, UtcOffset};

        let mut reminders = note.reminders().to_vec();
        let mut removals: Vec<usize> = Vec::new();
        let mut reminders_changed = false;
        ui.add(egui::Label::new(
            egui::RichText::new("Reminders:").underline(),
        ));
        ui.add_space(10.);

        if reminders.is_empty() {
            ui.add(egui::Label::new(
                egui::RichText::new("No reminders set.").weak(),
            ));
        } else {
            let grid_id = egui::Id::new("note_reminders_grid");

            match self.active_reminder {
                Some((idx, state)) => {
                    egui::Grid::new(grid_id)
                        .num_columns(4)
                        .min_col_width(20.)
                        .max_col_width(60.)
                        .show(ui, |ui| {
                            for (i, reminder) in reminders.iter_mut().enumerate() {
                                if i == idx {
                                    reminders_changed |=
                                        self.render_enabled_reminder_line(ui, reminder, i, state);
                                } else {
                                    self.render_disabled_reminder_line(ui, reminder, i);
                                }

                                if ui.small_button("x").clicked() {
                                    removals.push(i);
                                }
                                ui.end_row();
                            }
                        });
                }
                None => {
                    egui::Grid::new(grid_id).num_columns(4).show(ui, |ui| {
                        for (i, reminder) in reminders.iter_mut().enumerate() {
                            self.render_disabled_reminder_line(ui, reminder, i);
                            if ui.small_button("x").clicked() {
                                removals.push(i);
                            }
                            ui.end_row();
                        }
                    });
                }
            }
        }

        if ui.small_button("Create Reminder").clicked() {
            reminders.push(Reminder::default());
            self.active_reminder = Some((reminders.len() - 1, ReminderFocus::Text));
            reminders_changed = true;
        }

        if !removals.is_empty() {
            removals.sort_unstable();
            removals.reverse();
            for i in removals {
                reminders.remove(i);
            }
            reminders_changed = true;
        }

        if reminders_changed {
            note.set_reminders(reminders);
            self.force_save = true;
        }

        reminders_changed
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

    fn render_enabled_reminder_line(
        &mut self,
        ui: &mut Ui,
        reminder: &mut Reminder,
        idx: usize,
        focus: ReminderFocus,
    ) -> bool {
        let mut changes = false;
        let (mut hour, mut min, mut period) = reminder.get_due_time();
        let mut date = reminder
            .get_due_date_chrono_local()
            .with_timezone(&chrono::Utc);

        if focus == ReminderFocus::Date {
            let picker = egui_extras::DatePickerButton::new(&mut date);

            let dp_res = ui.add(picker);

            if dp_res.changed() || dp_res.lost_focus() {
                changes = true;
                reminder.set_due_date_chrono(&date);
            }

            if dp_res.lost_focus() {
                self.active_reminder = None;
            }
        } else {
            let dp_res =
                ui.add(egui::Label::new(reminder.date_display()).sense(egui::Sense::click()));
            if dp_res.double_clicked() {
                self.active_reminder = Some((idx, ReminderFocus::Date));
            }
        }

        if focus == ReminderFocus::Time {
            let mut lost_focus = false;

            changes |=
                Self::render_time_picker(ui, reminder, idx, &mut lost_focus, &self.app_sender);

            if lost_focus {
                self.active_reminder = None;
            }
        } else {
            let res = ui.add(egui::Label::new(reminder.time_display()).sense(egui::Sense::click()));
            if res.double_clicked() {
                self.active_reminder = Some((idx, ReminderFocus::Time));
            }
        }

        if focus == ReminderFocus::Text {
            let mut reminder_text = reminder.text().to_string();
            let text_res = ui.text_edit_singleline(&mut reminder_text);

            if text_res.changed() {
                changes = true;
                reminder.set_text(&reminder_text);
            }

            if text_res.lost_focus() {
                self.active_reminder = None;
            }
        } else {
            let res = ui.add(egui::Label::new(reminder.text()));
            if res.double_clicked() {
                self.active_reminder = Some((idx, ReminderFocus::Text));
            }
        }

        changes
    }

    fn render_disabled_reminder_line(&mut self, ui: &mut Ui, reminder: &mut Reminder, idx: usize) {
        let dp_res = ui.add(egui::Label::new(reminder.date_display()).sense(egui::Sense::click()));
        if dp_res.double_clicked() {
            self.active_reminder = Some((idx, ReminderFocus::Date));
        }

        let res = ui.add(egui::Label::new(reminder.time_display()).sense(egui::Sense::click()));
        if res.double_clicked() {
            self.active_reminder = Some((idx, ReminderFocus::Time));
        }

        let res = ui.add(egui::Label::new(reminder.text()).sense(egui::Sense::click()));
        if res.double_clicked() {
            self.active_reminder = Some((idx, ReminderFocus::Text));
        }
    }

    fn render_time_picker(
        ui: &mut Ui,
        reminder: &mut Reminder,
        idx: usize,
        lost_focus: &mut bool,
        sender: &Sender<ToApp>,
    ) -> bool {
        static ONCE_HR: Once = Once::new();
        static ONCE_MIN: Once = Once::new();
        static ONCE_HOR: Once = Once::new();
        const HOURS: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        const MINS: &[u8] = &[0, 15, 30, 45];

        let mut changes = false;

        let (mut hour, mut minute, mut period) = reminder.get_due_time();

        let res = ui.horizontal(|ui| {
            let hr_res = egui::ComboBox::from_id_source("time_picker_hour")
                .width(20.)
                .selected_text(hour.to_string())
                .show_ui(ui, |ui| {
                    HOURS
                        .iter()
                        .map(|&hr| {
                            ui.selectable_value(&mut hour, Hour12::from_u8(hr), hr.to_string())
                        })
                        .collect::<Vec<_>>()
                    // for hr in HOURS {
                    //     ui.selectable_value(&mut hour, Hour::from_u8(*hr), hr.to_string());
                    // }
                });

            if let Some(resps) = &hr_res.inner && resps.iter().any(egui::Response::changed) {
            // if hr_res.response.changed() {
                ONCE_HR.call_once(|| {
                    sender
                        .send(ToApp::Debug(format!(
                            "Hour Response: {:#?}",
                            hr_res.response
                        )))
                        .expect("Unable to send debug message to app");
                });
            }

            changes |= hr_res.response.changed();

            // if hr_res.inner.is_none() {
            //     *lost_focus = true;
            // }

            let min_res = egui::ComboBox::from_id_source("time_picker_min")
                .width(20.)
                .selected_text(minute.to_string())
                .show_ui(ui, |ui| {
                    MINS.iter()
                        .map(|&min| {
                            ui.selectable_value(&mut minute, Minute::from_u8(min), min.to_string())
                        })
                        .collect::<Vec<_>>()
                    // for min in MINS {
                    //     ui.selectable_value(&mut minute, Minute::from_u8(*min), min.to_string());
                    // }
                });

            if let Some(resps) = &min_res.inner && resps.iter().any(egui::Response::changed) {
            // if min_res.response.changed() {
                ONCE_MIN.call_once(|| {
                    sender
                        .send(ToApp::Debug(format!(
                            "Minute Response: {:#?}",
                            min_res.response
                        )))
                        .expect("Unable to send debug message to app");
                });
            }

            let mut is_am = period == TimePeriod::Am;
            if ui.toggle_value(&mut is_am, period.to_string()).clicked() {
                period.toggle();
                changes = true;
            }

            // if min_res.inner.is_none() {
            //     *lost_focus = true;
            // }
            (hr_res, min_res)
        });

        if res.response.changed() {
            ONCE_HOR.call_once(|| {
                sender
                    .send(ToApp::Debug(format!(
                        "Horizontal Response: {:#?}",
                        res.response
                    )))
                    .expect("Unable to send debug message to app");
            });
        }

        if res.response.clicked_elsewhere() {
            *lost_focus = true;
        }

        if changes || *lost_focus {
            reminder.set_due_hmp(hour, minute, period);
        }

        changes
    }

    fn send_app_msg(sender: &Sender<ToApp>, msg: ToApp) {
        sender.send(msg).expect("Unable to send message to GuiApp");
    }
}
