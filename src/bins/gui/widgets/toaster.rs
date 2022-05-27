// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Plans for the `Toaster`:
//! `Toast` will represent a single "toast message" or notification. `ToastKind` will determine how it is styled. `Toaster` will
//! expose very few public methods, for adding a new toast, clearing all toasts, and ...? Ideal workflow would be:
//! ```ignore
//!
//! ```
//!
use std::collections::VecDeque;

use eframe::{
    egui::{self, Area, Context, Id, Order, Ui},
    emath::{vec2, Align2},
};

pub enum ToastKind {
    Info,
    Warning,
    Error,
    Custom,
}

pub struct Toast {
    pub id: Id,
    pub kind: ToastKind,
    pub text: String,
    pub duration: f32,
}

pub struct Toaster {
    anchor: Align2,
    toasts: VecDeque<Toast>,
}

impl Toaster {
    const TOAST_AREA_ID: &'static str = "noted_toaster_area";
    const TOAST_ID_BASE: &'static str = "noted_toast";

    pub fn render(&mut self, ctx: &Context) {
        // let bottom_right = ctx.input().screen_rect();
        Area::new(Self::TOAST_AREA_ID)
            .anchor(Align2::RIGHT_BOTTOM, vec2(-5.0, -5.0))
            .interactable(false)
            .movable(false)
            .order(Order::Foreground)
            .show(ctx, |ui| {
                for toast in self.toasts.iter_mut().take(5) {
                    ui.label(toast.text.as_str());
                }
            });
    }
}
